use std::fmt::Display;

use crate::{
    ast::{visitor::Visit, File, Ident, ItemFn},
    ir::table::{self, SymbolTable},
    shared::Index,
};

/// Represents a resolved function.
#[derive(Debug, Clone)]
pub struct Function {
    /// The resolved type returned by this function.
    pub return_type: Type,
}

/// Represents a resolved type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Type(pub String);

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Raw strings should resolve to one of the following kinds of symbols.
#[derive(Debug, Clone)]
pub enum Symbol {
    Function(Function),
    Local(Local),
    Type(Type),
}

/// Represents a resolved local.
#[derive(Debug, Clone)]
pub struct Local {
    /// The resolved type of this local.
    pub ty: Type,
}

/// This structure is responsible for name resolution.
pub struct Resolver<'a> {
    /// The root of the abstract syntax tree.
    file: &'a File,

    /// The global symbol table.
    pub table: SymbolTable<'a, Symbol>,
}

impl<'a> Resolver<'a> {
    /// Create a new resolver.
    pub fn new(ast: &'a File) -> Self {
        // Create a new symbol table and populate it with primitive types
        // Populate the map with primitive types
        let mut table = SymbolTable::new();
        table.insert("()", Symbol::Type(Type("()".to_owned())));
        table.insert("i32", Symbol::Type(Type("i32".to_owned())));

        Resolver { file: ast, table }
    }

    /// Collect all the functions in the program. This is run during the first name resolution pass.
    pub fn collect_functions(&mut self) {
        self.visit_file(&self.file)
    }

    /// Resolve an identifier to the type it represents.
    pub fn resolve_ty(&self, ident: &Ident) -> Option<Type> {
        self.table
            .find(&ident.repr)
            .and_then(|symbol| match symbol {
                Symbol::Type(ty) => Some(ty),
                _ => None,
            })
    }

    /// Resolve an identifier to the local it represents.
    pub fn resolve_local(&self, ident: &Ident) -> Option<Type> {
        self.table
            .find(&ident.repr)
            .and_then(|symbol| match symbol {
                Symbol::Local(local) => Some(local.ty),
                _ => None,
            })
    }

    /// Resolve an identifier to the function it represents.
    pub fn resolve_fn(&self, ident: &Ident) -> Option<Function> {
        self.table
            .find(&ident.repr)
            .and_then(|symbol| match symbol {
                Symbol::Function(fn_) => Some(fn_),
                _ => None,
            })
    }
}

impl<'a> Visit<'a> for Resolver<'a> {
    fn visit_item_fn(&mut self, item_fn: &'a ItemFn) {
        let name = &item_fn.ident.repr;

        let symbol = Symbol::Function(Function {
            return_type: self
                .resolve_ty(&item_fn.ty.ident)
                .unwrap_or(Type(String::from("()"))),
        });

        self.table.insert(name, symbol)
    }
}
