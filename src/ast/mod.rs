#![allow(dead_code)]
pub mod visitor;
use crate::{lexer::Token, shared::Span};

#[derive(Debug)]
pub struct File {
    pub items: Vec<Item>,
    pub span: Span,
}

#[derive(Debug)]
pub enum Item {
    Fn(ItemFn),
    Struct(ItemStruct),
    Impl(ItemImpl),
}

/// Represents a function item (declaration).
#[derive(Debug)]
pub struct ItemFn {
    /// The `fn` keyword.
    pub kw: Token,

    /// The function identifier.
    pub ident: Ident,

    /// The left parenthesis.
    pub lp: Token,

    /// The function parameters
    pub params: ParamList,

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

/// Represents a list of impl function parameters.
#[derive(Debug)]
pub struct ImplParamList {
    /// Receiver
    pub receiver: Option<Token>,

    /// List of parameters
    pub params: Vec<Param>,

    /// Span of the entire puncuated list.
    pub span: Span,
}

/// Represents a list of function parameters.
#[derive(Debug)]
pub struct ParamList {
    /// List of parameters
    pub params: Vec<Param>,

    /// Span of the entire puncuated list.
    pub span: Span,
}

impl ParamList {
    /// Returns the length of the internal list of parameters.
    pub fn len(&self) -> usize {
        self.params.len()
    }
}

/// Represents a function parameter.
#[derive(Debug)]
pub struct Param {
    /// The parameter identifier.
    pub ident: Ident,

    /// The `:` symbol.
    pub colon: Token,

    /// The parameter type.
    pub ty: Ty,
}

/// Represents a struct item (declaration).
#[derive(Debug)]
pub struct ItemStruct {
    // The `struct` keyword
    pub kw: Token,

    /// The struct identifier.
    pub ident: Ident,

    /// The struct fields.
    pub fields: Fields,

    /// The span of the entire struct.
    pub span: Span,
}

/// Represents either a list of named fields, or a list of positional fields.
#[derive(Debug)]
pub enum Fields {
    Named(FieldsNamed),
}

/// Represents a list of named fields.
#[derive(Debug)]
pub struct FieldsNamed {
    /// The left curly brace.
    pub lb: Token,

    /// The list of fields.
    pub fields: Vec<FieldNamed>,

    /// The right curly brace.
    pub rb: Token,

    /// The span of the entire fields block.
    pub span: Span,
}

//// Represents a single named field.
#[derive(Debug)]
pub struct FieldNamed {
    /// The name of this field.
    pub ident: Ident,

    /// The ':' symbol.
    pub colon: Token,

    /// The type of this field.
    pub ty: Ty,

    /// The span of the entire field.
    pub span: Span,
}

#[derive(Debug)]
/// Represents an implementation.
pub struct ItemImpl {
    /// The `impl` keyword.
    pub kw: Token,

    pub ident: Ident,

    pub lb: Token,

    pub items: Vec<ImplItem>,

    pub rb: Token,

    pub span: Span,
}

#[derive(Debug)]
pub enum ImplItem {
    Fn(ImplItemFn),
}

#[derive(Debug)]
pub struct ImplItemFn {
    /// The `fn` keyword.
    pub kw: Token,

    /// The function identifier.
    pub ident: Ident,

    /// The left parenthesis.
    pub lp: Token,

    /// The function parameters
    pub params: ImplParamList,

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

#[derive(Debug, Clone)]
pub struct Ident {
    /// The raw string representation of this identifier.
    pub repr: String,

    /// The identifier span.
    pub span: Span,
}

#[derive(Debug)]
pub struct Ty {
    /// The raw string representation of this type.
    pub ident: Ident,

    /// The type span.
    pub span: Span,
}

#[derive(Debug)]
pub struct Block {
    /// The left curly brace.
    pub lc: Token,

    /// The statements in this block
    pub stmts: Vec<Stmt>,

    /// The right curly brace.
    pub rc: Token,

    /// The span of the entire block.
    pub span: Span,
}

#[derive(Debug)]
pub enum Stmt {
    Local(Local),
    Expr(Expr),
    Return(Return),
}

#[derive(Debug)]
pub struct Return {
    /// The `return` keyword.
    pub kw: Token,

    /// The expression being returned.
    pub expr: Expr,

    /// The span of the entire return statement.
    pub span: Span,
}

#[derive(Debug)]
pub enum Expr {
    Call(ExprCall),
    Binary(ExprBin),
    Struct(ExprStruct),
    Lit(ExprLit),
    Ident(Ident),
}

impl Expr {
    pub fn span(&self) -> &Span {
        match self {
            Self::Call(expr_call) => expr_call.span(),
            Self::Binary(expr_bin) => &expr_bin.span,
            Self::Struct(expr_struct) => &expr_struct.span,
            Self::Lit(expr_lit) => expr_lit.span(),
            Self::Ident(ident) => &ident.span,
        }
    }
}

#[derive(Debug)]
pub struct ExprStruct {
    pub ident: Ident,

    pub lb: Token,

    pub args: NamedArgList,

    pub rb: Token,

    pub span: Span,
}

#[derive(Debug)]
pub struct NamedArgList {
    pub args: Vec<NamedArg>,

    pub span: Span,
}

#[derive(Debug)]
pub struct NamedArg {
    pub ident: Ident,

    pub colon: Token,

    pub expr: Expr,

    pub span: Span,
}

#[derive(Debug)]
pub enum ExprLit {
    Num(LitNum),
}

impl ExprLit {
    pub fn span(&self) -> &Span {
        match self {
            Self::Num(lit_num) => &lit_num.span,
        }
    }
}

#[derive(Debug)]
pub struct LitNum {
    pub value: i32,

    pub span: Span,
}

#[derive(Debug)]
pub struct Local {
    /// The `let` keyword.
    pub kw: Token,

    /// The identifier being locally bound.
    pub ident: Ident,

    /// The semicolon following the identifier.
    pub colon: Token,

    /// The type of this variable.
    pub ty: Ty,

    /// The `=` symbol.
    pub eq: Token,

    /// The expression assigned to this
    pub expr: Expr,

    /// The span of the entire statement.
    pub span: Span,
}

#[derive(Debug)]
pub enum ExprCall {
    Fn(CallFn),
}

impl ExprCall {
    pub fn span(&self) -> &Span {
        match self {
            Self::Fn(call_fn) => &call_fn.span,
        }
    }
}

#[derive(Debug)]
pub struct CallFn {
    /// The name of the function being called.
    pub ident: Ident,

    /// The left parenthesis.
    pub lp: Token,

    /// The list of arguments.
    pub args: ArgList,

    /// The right parenthesis.
    pub rp: Token,

    /// The span of the entire function call.
    pub span: Span,
}

/// Represents a list of function arguments.
#[derive(Debug)]
pub struct ArgList {
    /// the list of arguments.
    pub args: Vec<Expr>,

    /// The span of the entire argument list.
    pub span: Span,
}

impl ArgList {
    /// Returns the length of the internal list of arguments.
    pub fn len(&self) -> usize {
        self.args.len()
    }
}

#[derive(Debug)]
pub struct ExprBin {
    /// The left hand side of this expression.
    pub lhs: Box<Expr>,

    /// The operator.
    pub op: BinaryOp,

    /// The right hand side of this expression.
    pub rhs: Box<Expr>,

    /// The span of this expression.
    pub span: Span,
}

#[derive(Debug)]
pub struct BinaryOp {
    /// The kind of operator.
    pub kind: OpKind,

    /// The operator span.
    pub span: Span,
}

#[derive(Debug)]
pub enum OpKind {
    Add,      // +
    Multiply, // *
}
