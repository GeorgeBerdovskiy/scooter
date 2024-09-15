#![allow(dead_code)]

use std::io;

pub trait Lower {
    fn lower(&mut self) -> io::Result<()>;
}
