#![allow(unused_imports)]

mod map;
mod pool;
mod span;

pub use map::*;
pub use pool::*;
pub use span::*;

/// Serves as an index for many data structures throughout the compiler.
pub type Index = usize;
