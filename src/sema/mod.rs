pub mod basic;
pub mod typeck;

use crate::{ast::File, shared::Span};

/// Represents an error that occured during semantic analysis.
pub struct SemaError {
    /// The cause of this error.
    pub reason: String,

    /// The (optional) span of this error.
    pub span: Option<Span>,
}

/// Represents the result of parsing.
type SemaResult<T> = Result<T, SemaError>;

/// Must be implemented for any semantic analysis.
pub trait Analysis {
    fn run(&mut self, ast: &File) -> SemaResult<()>;
}

/// Contains all semantic analysis to be run on the AST.
pub struct SemaEngine<'a> {
    ast: &'a File,
    analyses: Vec<Box<dyn Analysis>>,
}

impl<'a> SemaEngine<'a> {
    /// Create a new semantic engine.
    pub fn new(ast: &'a File) -> Self {
        SemaEngine {
            ast,
            analyses: vec![],
        }
    }

    /// Register an analysis.
    pub fn register(mut self, analysis: Box<dyn Analysis>) -> Self {
        self.analyses.push(analysis);
        self
    }

    /// Run all analyses.
    pub fn run(&mut self) -> Result<(), Vec<SemaError>> {
        let mut errors = Vec::new();

        for analysis in &mut self.analyses {
            match analysis.run(&self.ast) {
                Ok(_) => {}
                Err(err) => {
                    errors.push(err);
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
