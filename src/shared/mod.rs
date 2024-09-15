#![allow(unused_imports)]

mod interner;
mod map;
mod span;

pub use interner::*;
pub use map::*;
pub use span::*;

pub type Index = usize;
