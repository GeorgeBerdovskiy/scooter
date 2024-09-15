use crate::ast::visitor::{self, Visit};
use crate::ast::{File, Ident};
use crate::shared::Span;

use super::{Analysis, SemaError, SemaResult};

/// Make sure the source code contains a valid main function.
pub struct CheckMain {
    /// The main function identifier.
    main: Option<Ident>,

    /// How many parameters does the main function have?
    params: usize,
}

impl CheckMain {
    pub fn new() -> Self {
        CheckMain {
            main: None,
            params: 0,
        }
    }
}

impl Analysis for CheckMain {
    fn run(&mut self, file: &File) -> SemaResult<()> {
        self.visit_file(file);

        match &self.main {
            Some(ident) if self.params == 1 => {
                return Err(SemaError {
                    reason: format!("Main function takes no arguments, but 1 was provided"),
                    span: Some(ident.span.clone()),
                });
            }

            Some(ident) if self.params > 1 => {
                return Err(SemaError {
                    reason: format!(
                        "Main function takes no arguments, but {} were provided",
                        self.params
                    ),
                    span: Some(ident.span.clone()),
                });
            }

            None => {
                return Err(SemaError {
                    reason: format!("Could not find the main function"),
                    span: Some(Span::single(file.span.start.line, file.span.start.column)),
                });
            }

            _ => {}
        }

        Ok(())
    }
}

impl Visit<'_> for CheckMain {
    fn visit_item_fn(&mut self, item_fn: &'_ crate::ast::ItemFn) {
        if item_fn.ident.repr == "main" {
            self.main = Some(item_fn.ident.clone());
            self.params = item_fn.params.len();
        } else {
            visitor::visit_item_fn(self, item_fn);
        }
    }
}
