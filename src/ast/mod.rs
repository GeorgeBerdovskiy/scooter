pub mod visitor;

use crate::{lexer::Token, shared::Span};

pub struct Program {
    pub items: Vec<Item>,
}

pub enum Item {
    Fn(ItemFn),
}

pub struct ItemFn {
    /// The `fn` keyword.
    pub kw: Token,

    /// The function identifier.
    pub ident: Ident,

    /// The left parenthesis.
    pub lp: Token,

    /// The right parenthesis.
    pub rp: Token,

    /// The `->` symbol.
    pub arrow: Token,

    /// The return type.
    pub ty: Ty,

    /// The function body.
    pub body: Block,

    /// The function span.
    pub span: Span,
}

pub struct Ident {
    /// The raw string representation of this identifier.
    pub repr: String,

    /// The identifier span.
    pub span: Span,
}

pub struct Ty {
    /// The raw string representation of this type.
    pub repr: String,

    /// The type span.
    pub span: Span,
}

pub struct Block {
    /// The left curly brace.
    pub lc: Token,

    /// The statements in this block
    pub stmts: Vec<Stmt>,

    /// The right curly brace.
    pub rc: Token,
}

pub enum Stmt {
    Local(Local),
    Expr(Expr),
}

pub enum Expr {
    Call(CallExpr),
    Binary(BinaryExpr),
}

pub struct Local {
    /// The `let` keyword.
    pub kw: Token,

    /// The identifier being locally bound.
    pub ident: Ident,

    /// The semicolon following the identifier.
    pub semi: Token,

    /// The type of this variable.
    pub ty: Ty,

    /// The `=` symbol.
    pub eq: Token,

    /// The expression assigned to this
    pub expr: Expr,

    /// The span of the entire statement.
    pub span: Span,
}

pub enum CallExpr {
    CallFn(CallFn),
}

pub struct CallFn {
    /// The name of the function being called.
    pub ident: Ident,

    /// The left parenthesis.
    pub lp: Token,

    /// The right parenthesis.
    pub rp: Token,

    /// The span of the entire function call.
    pub span: Span,
}

pub struct BinaryExpr {
    /// The left hand side of this expression.
    pub lhs: Box<Expr>,

    /// The operator.
    pub op: BinaryOp,

    /// The right hand side of this expression.
    pub rhs: Box<Expr>,

    /// The span of this expression.
    pub span: Span,
}

pub struct BinaryOp {
    /// The kind of operator.
    pub kind: OpKind,

    /// The operator span.
    pub span: Span,
}

pub enum OpKind {
    Add,      // +
    Multiply, // *
}
