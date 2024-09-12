use std::collections::HashMap;

use crate::ast::visitor::Visit;

pub struct Name(usize);

/// Lowers the AST to IR by visiting the syntax nodes. The result is later lowered to ASM.
pub struct Generator<'a> {
    map: HashMap<&'a str, Name>,
}

impl<'a> Visit for Generator<'a> {}
