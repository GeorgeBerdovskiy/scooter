#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io;

use crate::asm::lower::Lower;
use crate::asm::register::RegMgr;
use crate::ir::{BinInstr, Instr};
use crate::shared::{Index, Map};

type Integer = isize;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Container {
    Register(Index),
    Offset(Integer),
}

#[allow(non_camel_case_types)]
pub struct RISC_V<'a> {
    /// The file we are writing to.
    file: File,

    /// List of IR instructions to be lowered.
    instrs: &'a [Instr],

    /// Manages temporary registers.
    temps: RegMgr<6>,

    /// Manages argument registers.
    arguments: RegMgr<7>,

    /// Manages saved registers.
    saved: RegMgr<11>,

    /// Maps temporary addresses to their "containers" and vice versa.
    temp_map: Map<Index, Container>,

    /// Maps named addresses to their "containers" and vice versa.
    name_map: Map<Index, Container>,

    /// Current stack pointer offset
    offset: Integer,
}

impl<'a> Lower for RISC_V<'a> {
    fn lower(&mut self) -> io::Result<()> {
        for instr in self.instrs {
            self.lower_instr(instr)?;
        }

        Ok(())
    }
}

impl<'a> RISC_V<'a> {
    fn lower_instr(&mut self, instr: &Instr) -> io::Result<()> {
        match instr {
            Instr::Binary(bin_instr) => self.lower_bin_instr(bin_instr)?,
            _ => todo!(),
        }

        Ok(())
    }

    fn lower_bin_instr(&mut self, bin_instr: &BinInstr) -> io::Result<()> {
        todo!()
    }
}
