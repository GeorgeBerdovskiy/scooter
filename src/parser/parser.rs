use std::fmt::format;

use crate::ast::{
    ArgList, BinaryOp, Block, CallFn, Expr, ExprBin, ExprCall, ExprLit, ExprStruct, FieldNamed,
    Fields, FieldsNamed, File, Ident, ImplItem, ImplItemFn, ImplParamList, Item, ItemFn, ItemImpl,
    ItemStruct, LitNum, Local, NamedArg, NamedArgList, OpKind, Param, ParamList, Return, Stmt, Ty,
};
use crate::lexer::{Token, TokenKind};
use crate::shared::Span;

/// Represents an error that occured during parsing.
pub struct ParseError {
    /// The cause of this error.
    pub reason: String,

    /// The (optional) span of this error.
    pub span: Option<Span>,
}

/// Represents the result of parsing.
type ParseResult<T> = Result<T, ParseError>;

pub struct Parser<'a> {
    /// The tokens of an entire file.
    input: &'a [Token],

    /// The index of the current token.
    index: usize,

    /// The current span.
    starts: Vec<Span>,
}

impl<'a> Parser<'a> {
    /// Create a new parser.
    pub fn new(input: &'a [Token]) -> Self {
        Parser {
            input,
            index: 0,
            starts: vec![],
        }
    }

    /// Parse an entire file.
    pub fn parse_file(&mut self) -> ParseResult<File> {
        self.start();

        let mut items: Vec<Item> = Vec::new();

        while self.current_kind() != &TokenKind::EOF && self.current_kind() != &TokenKind::RBrace {
            items.push(self.parse_item()?);
        }

        Ok(File {
            items,
            span: self.end(),
        })
    }

    /// Parse an item.
    fn parse_item(&mut self) -> ParseResult<Item> {
        let kind = self.current_kind();

        match kind {
            TokenKind::KwFn => self.parse_item_fn(),
            TokenKind::KwStruct => self.parse_item_struct(),
            TokenKind::KwImpl => self.parse_item_impl(),
            _ => Err(ParseError {
                reason: format!("Expected 'fn' or 'mod', found {kind}"),
                span: Some(self.end()),
            }),
        }
    }

    /// Parse an impl block.
    fn parse_item_impl(&mut self) -> ParseResult<Item> {
        self.start();
        let kw = self.expect(TokenKind::KwImpl)?;

        let ident = self.parse_ident()?;

        let lb = self.expect(TokenKind::LBrace)?;

        let mut items = Vec::new();
        while self.current_kind() != &TokenKind::RBrace {
            items.push(self.parse_impl_item()?);
        }

        Ok(Item::Impl(ItemImpl {
            kw,
            ident,
            lb,
            items,
            rb: self.expect(TokenKind::RBrace)?,
            span: self.end(),
        }))
    }

    fn parse_impl_item(&mut self) -> ParseResult<ImplItem> {
        let kind = self.current_kind();
        if kind == &TokenKind::KwFn {
            self.parse_impl_item_fn()
        } else {
            Err(ParseError {
                reason: format!("Expected a function item, found {kind}"),
                span: self.current().span.clone(),
            })
        }
    }

    fn parse_impl_item_fn(&mut self) -> ParseResult<ImplItem> {
        // Start a new span
        self.start();

        Ok(ImplItem::Fn(ImplItemFn {
            kw: self.expect(TokenKind::KwFn)?,
            ident: self.parse_ident()?,
            lp: self.expect(TokenKind::LParen)?,
            params: self.parse_impl_param_list()?,
            rp: self.expect(TokenKind::RParen)?,
            arrow: self.expect(TokenKind::RArrow)?,
            ty: self.parse_ty()?,
            body: self.parse_block()?,
            span: self.end(),
        }))
    }

    /// Parse a struct declaration.
    fn parse_item_struct(&mut self) -> ParseResult<Item> {
        self.start();

        Ok(Item::Struct(ItemStruct {
            kw: self.expect(TokenKind::KwStruct)?,
            ident: self.parse_ident()?,
            fields: self.parse_fields()?,
            span: self.end(),
        }))
    }

    fn parse_fields(&mut self) -> ParseResult<Fields> {
        if self.current_kind() == &TokenKind::LBrace {
            self.parse_fields_named()
        } else {
            todo!()
        }
    }

    fn parse_fields_named(&mut self) -> ParseResult<Fields> {
        self.start();

        let lb = self.expect(TokenKind::LBrace)?;

        let mut fields = Vec::new();

        while self.current_kind() != &TokenKind::RBrace {
            fields.push(self.parse_field_named()?);

            if self.current_kind() != &TokenKind::RBrace {
                self.expect(TokenKind::Comma)?;
            }
        }

        Ok(Fields::Named(FieldsNamed {
            lb,
            fields,
            rb: self.expect(TokenKind::RBrace)?,
            span: self.end(),
        }))
    }

    fn parse_field_named(&mut self) -> ParseResult<FieldNamed> {
        self.start();
        Ok(FieldNamed {
            ident: self.parse_ident()?,
            colon: self.expect(TokenKind::Colon)?,
            ty: self.parse_ty()?,
            span: self.end(),
        })
    }

    /// Parse a function declaration.
    fn parse_item_fn(&mut self) -> ParseResult<Item> {
        // Start a new span
        self.start();

        Ok(Item::Fn(ItemFn {
            kw: self.expect(TokenKind::KwFn)?,
            ident: self.parse_ident()?,
            lp: self.expect(TokenKind::LParen)?,
            params: self.parse_param_list()?,
            rp: self.expect(TokenKind::RParen)?,
            arrow: self.expect(TokenKind::RArrow)?,
            ty: self.parse_ty()?,
            body: self.parse_block()?,
            span: self.end(),
        }))
    }

    /// Parse a list of function parameters.
    fn parse_param_list(&mut self) -> ParseResult<ParamList> {
        self.start();
        let mut params = Vec::new();

        while self.current_kind() != &TokenKind::RParen {
            params.push(self.parse_param()?);

            if self.current_kind() != &TokenKind::RParen {
                // Since we haven't reached the closing parenthesis yet, we expect a comma
                self.expect(TokenKind::Comma)?;
            }
        }

        Ok(ParamList {
            params,
            span: self.end(),
        })
    }

    /// Parse a list of function parameters.
    fn parse_impl_param_list(&mut self) -> ParseResult<ImplParamList> {
        self.start();

        let receiver = match self.current_kind() {
            TokenKind::KwSelf => {
                let rcvr = Some(self.expect(TokenKind::KwSelf)?);

                if self.current_kind() != &TokenKind::RParen {
                    self.expect(TokenKind::Comma)?;
                }

                rcvr
            }
            _ => None,
        };

        let mut params = Vec::new();

        while self.current_kind() != &TokenKind::RParen {
            params.push(self.parse_param()?);

            if self.current_kind() != &TokenKind::RParen {
                // Since we haven't reached the closing parenthesis yet, we expect a comma
                self.expect(TokenKind::Comma)?;
            }
        }

        Ok(ImplParamList {
            receiver,
            params,
            span: self.end(),
        })
    }

    /// Parse a single function parameter.
    fn parse_param(&mut self) -> ParseResult<Param> {
        Ok(Param {
            ident: self.parse_ident()?,
            colon: self.expect(TokenKind::Colon)?,
            ty: self.parse_ty()?,
        })
    }

    /// Parse an identifier.
    fn parse_ident(&mut self) -> ParseResult<Ident> {
        let current = self.current().clone();

        match current.kind {
            TokenKind::Ident(raw) => {
                self.advance(1);
                Ok(Ident {
                    repr: raw,
                    span: current.span.unwrap(),
                })
            }
            _ => Err(ParseError {
                reason: format!("Expected an identifier, found {}", current.kind),
                span: current.span,
            }),
        }
    }

    /// Parse a block of statements enclosed by curly braces.
    fn parse_block(&mut self) -> ParseResult<Block> {
        self.start();

        // Get the left curly brace
        let lc = self.expect(TokenKind::LBrace)?;

        // Collect the statements
        let mut stmts = Vec::new();
        while self.current_kind() != &TokenKind::RBrace {
            stmts.push(self.parse_stmt()?);
            self.expect(TokenKind::Semicolon)?;
        }

        Ok(Block {
            lc,
            stmts,
            rc: self.expect(TokenKind::RBrace)?,
            span: self.end(),
        })
    }

    /// Parse a statement.
    fn parse_stmt(&mut self) -> ParseResult<Stmt> {
        let current = self.current().clone();

        match current.kind {
            TokenKind::KwLet => Ok(Stmt::Local(self.parse_local()?)),
            TokenKind::KwRet => Ok(Stmt::Return(self.parse_return()?)),
            _ => Err(ParseError {
                reason: format!("Unknown statement beginning with {}", current.kind),
                span: current.span,
            }),
        }
    }

    /// Parse a return statement.
    fn parse_return(&mut self) -> ParseResult<Return> {
        self.start();

        Ok(Return {
            kw: self.expect(TokenKind::KwRet)?,
            expr: self.parse_expr()?,
            span: self.end(),
        })
    }

    /// Parse a local `let` binding.
    fn parse_local(&mut self) -> ParseResult<Local> {
        self.start();

        Ok(Local {
            kw: self.expect(TokenKind::KwLet)?,
            ident: self.parse_ident()?,
            colon: self.expect(TokenKind::Colon)?,
            ty: self.parse_ty()?,
            eq: self.expect(TokenKind::Equal)?,
            expr: self.parse_expr()?,
            span: self.end(),
        })
    }

    /// Parse an expression (`expr ::= term { "+" term }`).
    fn parse_expr(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_term()?;

        while self.current_kind() == &TokenKind::Plus {
            let op = self.expect(TokenKind::Plus)?;
            let op = BinaryOp {
                kind: OpKind::Add,
                span: op.span.clone().unwrap(),
            };

            let rhs = self.parse_term()?;
            let start = expr.span().clone().start;
            let end = rhs.span().clone().end;

            expr = Expr::Binary(ExprBin {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
                span: Span::new(start, end),
            })
        }

        Ok(expr)
    }

    /// Parse a term (`term ::= factor { "*" factor }`).
    fn parse_term(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_factor()?;

        while self.current_kind() == &TokenKind::Star {
            let op = self.expect(TokenKind::Star)?;
            let op = BinaryOp {
                kind: OpKind::Multiply,
                span: op.span.clone().unwrap(),
            };

            let rhs = self.parse_factor()?;
            let start = expr.span().clone().start;
            let end = rhs.span().clone().end;

            expr = Expr::Binary(ExprBin {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
                span: Span::new(start, end),
            })
        }

        Ok(expr)
    }

    /// Parse a factor (`factor ::= lit-num | ident | call-fn | "(" expr ")"`).
    fn parse_factor(&mut self) -> ParseResult<Expr> {
        self.start();
        let current = self.current().clone();

        match current.kind {
            TokenKind::LitNum(value) => {
                self.advance(1);

                Ok(Expr::Lit(ExprLit::Num(LitNum {
                    value,
                    span: self.end(),
                })))
            }

            TokenKind::Ident(repr) => {
                let ident = self.parse_ident()?;

                match self.current_kind() {
                    TokenKind::LParen => Ok(Expr::Call(ExprCall::Fn(CallFn {
                        ident,
                        lp: self.expect(TokenKind::LParen)?,
                        args: self.parse_arg_list()?,
                        rp: self.expect(TokenKind::RParen)?,
                        span: self.end(),
                    }))),

                    TokenKind::LBrace => Ok(Expr::Struct(ExprStruct {
                        ident,
                        lb: self.expect(TokenKind::LBrace)?,
                        args: self.parse_named_arg_list()?,
                        rb: self.expect(TokenKind::RBrace)?,
                        span: self.end(),
                    })),

                    _ => Ok(Expr::Ident(ident)),
                }
            }

            _ => {
                self.advance(1);
                Err(ParseError {
                    reason: format!(
                        "Expected an identifier, literal or function call... found {}",
                        current.kind
                    ),
                    span: Some(self.end()),
                })
            }
        }
    }

    fn parse_arg_list(&mut self) -> ParseResult<ArgList> {
        self.start();
        let mut args = Vec::new();

        while self.current_kind() != &TokenKind::RParen {
            args.push(self.parse_expr()?);

            if self.current_kind() != &TokenKind::RParen {
                self.expect(TokenKind::Comma)?;
            }
        }

        Ok(ArgList {
            args,
            span: self.end(),
        })
    }

    fn parse_named_arg_list(&mut self) -> ParseResult<NamedArgList> {
        self.start();
        let mut args = Vec::new();

        while self.current_kind() != &TokenKind::RBrace {
            args.push(self.parse_named_arg()?);

            if self.current_kind() != &TokenKind::RBrace {
                self.expect(TokenKind::Comma)?;
            }
        }

        Ok(NamedArgList {
            args,
            span: self.end(),
        })
    }

    fn parse_named_arg(&mut self) -> ParseResult<NamedArg> {
        self.start();

        Ok(NamedArg {
            ident: self.parse_ident()?,
            colon: self.expect(TokenKind::Colon)?,
            expr: self.parse_expr()?,
            span: self.end(),
        })
    }

    /// Parse a type.
    fn parse_ty(&mut self) -> ParseResult<Ty> {
        self.start();

        let current = self.current();
        match current.kind {
            TokenKind::LParen => {
                // Special case - the unit type '()'
                self.expect(TokenKind::LParen)?;
                self.expect(TokenKind::RParen)?;

                let span: Span = self.end();
                Ok(Ty {
                    ident: Ident {
                        repr: "()".to_owned(),
                        span: span.clone(),
                    },
                    span: span,
                })
            }

            _ => Ok(Ty {
                ident: self.parse_ident()?,
                span: self.end(),
            }),
        }
    }

    /// Start a span at the current location.
    fn start(&mut self) {
        let span = self.input[self.index].clone().span.unwrap();
        self.starts.push(span);
    }

    /// End a span at the current location.
    fn end(&mut self) -> Span {
        let from = self.starts.last().cloned().unwrap();
        let to = self.input[self.index - 1].clone().span.unwrap();

        self.starts.pop();
        Span::new(from.start, to.end)
    }

    /// Get the kind of the current token.
    fn current_kind(&self) -> &TokenKind {
        &self.input[self.index].kind
    }

    /// Get the current token.
    fn current(&self) -> &Token {
        &self.input[self.index]
    }

    /// Advance the token `n` times.
    fn advance(&mut self, n: usize) {
        if self.index + n >= self.input.len() {
            self.index = self.input.len() - 1;
        } else {
            self.index += n;
        }
    }

    /// Return the current token if its kind matches `kind`, or an error otherwise.
    fn expect(&mut self, kind: TokenKind) -> ParseResult<Token> {
        if self.current_kind() == &kind {
            let token = self.current().clone();
            self.advance(1);
            return Ok(token);
        }

        Err(ParseError {
            reason: format!("Expected {kind} but found {}", self.current_kind()),
            span: self.current().span.clone(),
        })
    }
}
