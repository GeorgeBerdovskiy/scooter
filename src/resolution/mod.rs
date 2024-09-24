use std::collections::HashMap;
use std::fmt::Display;

use crate::ast::{visitor::Visit, File, Ident, ItemFn};
use crate::ast::{Fields, ItemStruct};
use crate::ir::table::SymbolTable;

#[derive(PartialEq)]
pub enum CollectMode {
    Types,
    Functions,
    Unset,
}

/// Represents a resolved function.
#[derive(Debug, Clone)]
pub struct Function {
    /// The resolved type returned by this function.
    pub return_type: Type,
}

/// Represents a resolved type.
#[derive(Debug, Clone)]
pub enum Type {
    Primitive(String),
    Struct(TyStruct),
}

#[derive(Debug, Clone)]
pub struct TyStruct {
    path: String,
    pub fields: HashMap<String, String>,
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        let left = match self {
            Self::Primitive(repr) => repr,
            Self::Struct(strct) => &strct.path,
        };

        let right = match other {
            Self::Primitive(repr) => repr,
            Self::Struct(strct) => &strct.path,
        };

        left == right
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primitive(repr) => write!(f, "{}", repr),
            Self::Struct(strct) => write!(f, "{}", strct.path),
        }
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

    /// Which construct is being collected.
    mode: CollectMode,
}

impl<'a> Resolver<'a> {
    /// Create a new resolver.
    pub fn new(ast: &'a File) -> Self {
        // Create a new symbol table and populate it with primitive types
        // Populate the map with primitive types
        let mut table = SymbolTable::new();
        table.insert("()", Symbol::Type(Type::Primitive("()".to_owned())));
        table.insert("i32", Symbol::Type(Type::Primitive("i32".to_owned())));

        Resolver {
            file: ast,
            table,
            mode: CollectMode::Unset,
        }
    }

    pub fn collect_tys(&mut self) {
        self.mode = CollectMode::Types;
        self.visit_file(&self.file);
    }

    /// Collect all the functions in the program. This is run during the first name resolution pass.
    pub fn collect_functions(&mut self) {
        self.mode = CollectMode::Functions;
        self.visit_file(&self.file)
    }

    /// Resolve an identifier to the type it represents.
    pub fn resolve_ty(&self, ident: &str) -> Option<Type> {
        self.table.find(&ident).and_then(|symbol| match symbol {
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
        if self.mode != CollectMode::Functions {
            return;
        }

        let name = &item_fn.ident.repr;

        let symbol = Symbol::Function(Function {
            return_type: self
                .resolve_ty(&item_fn.ty.ident.repr)
                .unwrap_or(Type::Primitive(String::from("()"))),
        });

        self.table.insert(name, symbol)
    }

    fn visit_item_struct(&mut self, item_struct: &'a crate::ast::ItemStruct) {
        if self.mode != CollectMode::Types {
            return;
        }

        let name = &item_struct.ident.repr;

        let symbol = Symbol::Type(Type::Struct(TyStruct {
            path: name.clone(),
            fields: item_struct_fields(item_struct),
        }));

        self.table.insert(name, symbol)
    }
}

fn item_struct_fields(item_struct: &ItemStruct) -> HashMap<String, String> {
    let mut result = HashMap::new();

    match &item_struct.fields {
        Fields::Named(named_fields) => {
            for field in &named_fields.fields {
                result.insert(field.ident.repr.clone(), field.ty.ident.repr.clone());
            }
        }

        _ => {}
    }

    result
}
