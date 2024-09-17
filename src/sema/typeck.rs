use crate::{
    ast::{visitor::Visit, Block, Expr, ExprBin, ExprCall, ExprLit, File, Ident, Stmt},
    resolution::{Local, Resolver, Symbol, Type},
    shared::Span,
};

pub struct TypeCkError {
    /// The cause of this error.
    pub reason: String,

    /// The (optional) span of this error.
    pub span: Option<Span>,
}

pub type TypeCkResult<T> = Result<T, TypeCkError>;

pub struct TypeCk<'a> {
    resolver: Resolver<'a>,
    result: TypeCkResult<()>,
}

impl<'a> TypeCk<'a> {
    pub fn new(resolver: Resolver<'a>) -> Self {
        TypeCk {
            resolver,
            result: Ok(()),
        }
    }

    pub fn run(mut self, file: &'a File) -> TypeCkResult<()> {
        self.visit_file(file);
        self.result
    }
}

impl<'a> Visit<'a> for TypeCk<'a> {
    fn visit_item_fn(&mut self, item_fn: &'a crate::ast::ItemFn) {
        // Does the type of the body match the expected return type?
        match self.resolver.resolve_ty(&item_fn.ty.ident) {
            Some(expected) => {
                match self.typeck_block(&item_fn.body) {
                    Err(err) => self.result = Err(err),
                    Ok(actual) => {
                        if expected != actual {
                            self.result = Err(TypeCkError { reason: format!("Function must return type '{}' but type '{}' is returned instead", expected, actual), span: Some(item_fn.ty.span.clone()) })
                        }
                    }
                }
            }

            None => {
                self.result = Err(TypeCkError {
                    reason: format!("Unknown type '{}'", item_fn.ty.ident.repr),
                    span: Some(item_fn.ty.span.clone()),
                })
            }
        }
    }
}

impl<'a> TypeCk<'a> {
    fn typeck_block(&mut self, block: &'a Block) -> TypeCkResult<Type> {
        let mut result: Type = Type(String::from("()"));

        for (index, stmt) in block.stmts.iter().enumerate() {
            // Throw away the result of typechecking every statement except the last one
            let _ = self.typeck_stmt(stmt);

            if index == block.stmts.len() - 1 {
                // This is the return statement, and must be the type of the block
                result = self.typeck_stmt(stmt)?;
            }
        }

        Ok(result)
    }

    fn typeck_stmt(&mut self, stmt: &'a Stmt) -> TypeCkResult<Type> {
        match stmt {
            Stmt::Local(local) => {
                // Type check the expression
                let actual = self.typeck_expr(&local.expr)?;
                let expected = self.resolver.resolve_ty(&local.ty.ident);

                match expected {
                    Some(expected) => {
                        if expected == actual {
                            // This statement checks out
                            self.resolver.table.insert(
                                &local.ident.repr,
                                Symbol::Local(Local { ty: actual.clone() }),
                            );
                            Ok(actual)
                        } else {
                            // The expected type doesn't match the actual type
                            Err(TypeCkError {
                                reason: format!("The expression assigned to variable '{}' must have type '{}' but it actually has type '{}'", local.ident.repr, expected, actual),
                                span: Some(local.expr.span().clone())
                            })
                        }
                    }

                    None => {
                        // The type assigned to this local variable doesn't exist
                        Err(TypeCkError {
                            reason: format!("The type '{}' doesn't exist", local.ty.ident.repr),
                            span: Some(local.ty.ident.span.clone()),
                        })
                    }
                }
            }

            Stmt::Return(ret) => {
                // Type check the returned expression
                self.typeck_expr(&ret.expr)
            }

            _ => todo!(),
        }
    }

    fn typeck_expr(&mut self, expr: &'a Expr) -> TypeCkResult<Type> {
        match expr {
            Expr::Binary(expr_bin) => self.typeck_expr_bin(expr_bin),
            Expr::Call(expr_call) => self.typeck_expr_call(expr_call),
            Expr::Ident(ident) => self.typeck_ident(ident),
            Expr::Lit(expr_lit) => self.typeck_expr_lit(expr_lit),
        }
    }

    fn typeck_expr_lit(&mut self, expr_lit: &'a ExprLit) -> TypeCkResult<Type> {
        match expr_lit {
            ExprLit::Num(_) => Ok(Type(String::from("i32"))), // Right now, all literal numbers are `i32` values
        }
    }

    fn typeck_ident(&mut self, ident: &'a Ident) -> TypeCkResult<Type> {
        match self.resolver.resolve_local(ident) {
            Some(ty) => Ok(ty),
            None => Err(TypeCkError {
                reason: format!("Cannot find '{}' in this scope", ident.repr),
                span: Some(ident.span.clone()),
            }),
        }
    }

    fn typeck_expr_call(&mut self, expr_call: &'a ExprCall) -> TypeCkResult<Type> {
        match expr_call {
            ExprCall::Fn(call) => {
                // First, we need to collect the function signature
                match self.resolver.resolve_fn(&call.ident) {
                    Some(sig) => Ok(sig.return_type),

                    None => Err(TypeCkError {
                        reason: format!("Undefined function '{}'", call.ident.repr),
                        span: Some(call.ident.span.clone()),
                    }),
                }
            }
        }
    }

    fn typeck_expr_bin(&mut self, expr_bin: &'a ExprBin) -> TypeCkResult<Type> {
        // What's the type of the lhs?
        let lhs = self.typeck_expr(&expr_bin.lhs)?;
        let rhs = self.typeck_expr(&expr_bin.rhs)?;

        if lhs == rhs {
            // We're good!
            Ok(lhs)
        } else {
            // The type of the lhs doesn't match the rhs
            Err(TypeCkError {
                reason: format!("Left hand side of binary expression has type '{}' but the right hand side has type '{}'", lhs, rhs),
                span: Some(expr_bin.rhs.span().clone())
            })
        }
    }
}
