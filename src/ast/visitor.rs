use paste::paste;

use super::{Block, Ident, Item, ItemFn, Program};

/// This macro generates the `Visitor` trait. Unfortunately, you still have to manually implement each `visit_*` function
/// that exists outside of the trait, but it still saves some time.
macro_rules! visitor {
    // This pattern matches multiple `(ident) Type` pairs in the macro input.
    ( $( $arg:ident : $ty:ident),* ) => {
        // This trait can be used to visit the AST in a specific way. All the analyzers and generators use this trait.
        pub trait Visit: Sized {
            $(
                paste! {
                    // For each pair, generate a visit method in the trait.
                    fn [<visit_ $arg>] (&mut self, $arg: &$ty) {
                        // Call the corresponding visit function.
                        concat_idents!(visit_, $arg) (self, $arg);
                    }
                }
            )*
        }
    };
}

// Generate the visitor trait.
visitor! {
    program: Program,
    item: Item,
    item_fn: ItemFn,
    ident: Ident,
    block: Block
}

pub fn visit_program(visitor: &mut impl Visit, program: &Program) {
    for item in &program.items {
        visitor.visit_item(item)
    }
}

pub fn visit_item(visitor: &mut impl Visit, item: &Item) {
    match item {
        Item::Fn(item_fn) => visitor.visit_item_fn(&item_fn),
    }
}

pub fn visit_item_fn(visitor: &mut impl Visit, item_fn: &ItemFn) {
    visitor.visit_ident(&item_fn.ident);
    visitor.visit_block(&item_fn.body);
}

pub fn visit_ident(visitor: &mut impl Visit, ident: &Ident) {
    // Nothing to do here
}

pub fn visit_block(visitor: &mut impl Visit, block: &Block) {
    todo!("Visit every statement in the block")
}
