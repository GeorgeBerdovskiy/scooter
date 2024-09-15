#![allow(unused_variables)]
use paste::paste;

use super::{
    Block, CallFn, Expr, ExprBin, ExprCall, ExprLit, File, Ident, Item, ItemFn, LitNum, Local,
    Return, Stmt, Ty,
};

/// This macro generates the `Visitor` trait. Unfortunately, you still have to manually implement each `visit_*` function
/// that exists outside of the trait, but it still saves some time.
macro_rules! visitor {
    ( $( $arg:ident : $ty:ident),* ) => {
        pub trait Visit<'a>: Sized {
            $(
                paste! {
                    fn [<visit_ $arg>] (&mut self, $arg: &'a $ty) {
                        concat_idents!(visit_, $arg) (self, $arg);
                    }
                }
            )*
        }
    };
}

// Generate the visitor trait.
visitor! {
    file: File,
    item: Item,
    item_fn: ItemFn,
    ident: Ident,
    block: Block,
    stmt: Stmt,
    local: Local,
    expr: Expr,
    ty: Ty,
    expr_bin: ExprBin,
    expr_call: ExprCall,
    expr_lit: ExprLit,
    call_fn: CallFn,
    lit_num: LitNum,
    ret: Return
}

pub fn visit_file<'a>(visitor: &mut impl Visit<'a>, program: &'a File) {
    for item in &program.items {
        visitor.visit_item(item)
    }
}

pub fn visit_item<'a>(visitor: &mut impl Visit<'a>, item: &'a Item) {
    match item {
        Item::Fn(item_fn) => visitor.visit_item_fn(&item_fn),
    }
}

pub fn visit_item_fn<'a>(visitor: &mut impl Visit<'a>, item_fn: &'a ItemFn) {
    visitor.visit_ident(&item_fn.ident);
    visitor.visit_block(&item_fn.body);
}

pub fn visit_ident<'a>(visitor: &mut impl Visit<'a>, ident: &'a Ident) {
    // Nothing to do here
}

pub fn visit_block<'a>(visitor: &mut impl Visit<'a>, block: &'a Block) {
    for stmt in &block.stmts {
        visitor.visit_stmt(stmt)
    }
}

pub fn visit_stmt<'a>(visitor: &mut impl Visit<'a>, stmt: &'a Stmt) {
    match stmt {
        Stmt::Local(local) => visitor.visit_local(local),
        Stmt::Expr(expr) => visitor.visit_expr(expr),
        Stmt::Return(ret) => visitor.visit_ret(ret),
    }
}

pub fn visit_local<'a>(visitor: &mut impl Visit<'a>, local: &'a Local) {
    visitor.visit_ident(&local.ident);
    visitor.visit_ty(&local.ty);
    visitor.visit_expr(&local.expr)
}

pub fn visit_expr<'a>(visitor: &mut impl Visit<'a>, expr: &'a Expr) {
    match expr {
        Expr::Binary(expr_bin) => visitor.visit_expr_bin(expr_bin),
        Expr::Call(expr_call) => visitor.visit_expr_call(expr_call),
        Expr::Lit(expr_lit) => visitor.visit_expr_lit(expr_lit),
        Expr::Ident(ident) => visitor.visit_ident(ident),
    }
}

pub fn visit_ty<'a>(visitor: &mut impl Visit<'a>, ty: &'a Ty) {
    // Nothing to do here
}

pub fn visit_expr_bin<'a>(visitor: &mut impl Visit<'a>, expr_bin: &'a ExprBin) {
    visitor.visit_expr(&expr_bin.lhs);
    visitor.visit_expr(&expr_bin.rhs);
}

pub fn visit_expr_call<'a>(visitor: &mut impl Visit<'a>, expr_call: &'a ExprCall) {
    match expr_call {
        ExprCall::Fn(call_fn) => visitor.visit_call_fn(call_fn),
    }
}

pub fn visit_expr_lit<'a>(visitor: &mut impl Visit<'a>, expr_lit: &'a ExprLit) {
    match expr_lit {
        ExprLit::Num(lit_num) => visitor.visit_lit_num(lit_num),
    }
}

pub fn visit_call_fn<'a>(visitor: &mut impl Visit<'a>, call_fn: &'a CallFn) {
    visitor.visit_ident(&call_fn.ident);
}

pub fn visit_lit_num<'a>(visitor: &mut impl Visit<'a>, lit_num: &'a LitNum) {
    // Nothing to do here
}

pub fn visit_ret<'a>(visitor: &mut impl Visit<'a>, ret: &'a Return) {
    visitor.visit_expr(&ret.expr);
}
