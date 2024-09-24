use std::fs::File;
use std::io::{self, Write};

use crate::shared::Index;

pub mod instr;
pub mod lower;
mod mapper;
pub mod table;

pub use instr::*;
pub use lower::*;

/// The IR representation of a program. Really just a fancy list of instructions right now. Later it will likely
/// become much more complicated!
pub struct IRRoot<'a> {
    pub last_label: Index,
    pub interner: LoweringPool<'a>,
    pub instrs: Vec<Instr>,
}

impl IRRoot<'_> {
    pub fn human_readable(&self, output: &str) -> io::Result<()> {
        let mut file = File::create(output)?;

        // Figure out how much padding is needed for the labels
        // Note that we add three to account for the 'L' character, the colon, and the space
        let max_length = self.last_label.to_string().len() + 3;
        let label_padding = " ".repeat(max_length);

        for instr in &self.instrs {
            match instr {
                Instr::Binary(bin) => {
                    let da = self.addr_readable(&bin.da, true);
                    let la = self.addr_readable(&bin.la, false);
                    let op = op_readable(&bin.op);
                    let ra = self.addr_readable(&bin.ra, false);

                    let pad = label(&bin.label, max_length, &label_padding);

                    writeln!(file, "{pad}{da} = {la} {op} {ra}")?;
                }

                Instr::Copy(cop) => {
                    let da = self.addr_readable(&cop.da, true);
                    let ad = self.addr_readable(&cop.ad, false);
                    let pad = label(&cop.label, max_length, &label_padding);

                    writeln!(file, "{pad}{da} = {ad}")?;
                }

                Instr::Return(ret) => {
                    let ad = self.addr_readable(&ret.ad, false);
                    let pad = label(&ret.label, max_length, &label_padding);

                    writeln!(file, "{pad}ret {ad}")?;
                }

                Instr::Call(call) => {
                    let da = self.addr_readable(&call.da, false);
                    let fl = self.label_readable(&call.fl);

                    let pad = label(&call.label, max_length, &label_padding);

                    writeln!(file, "{pad}{da} = call {fl}, {}", call.n)?;
                }

                Instr::Param(param) => {
                    let ad = self.addr_readable(&param.ad, false);
                    let pad = label(&param.label, max_length, &label_padding);

                    writeln!(file, "{pad}param {ad}")?;
                }

                _ => todo!(),
            }
        }

        Ok(())
    }

    /// Turns an address into a human readable string.
    fn addr_readable(&self, addr: &Addr, is_d: bool) -> String {
        match addr {
            Addr::Name(i) => format!("x{i}"),
            Addr::Temp(i) => format!("t{i}"),
            Addr::Const(i) if !is_d => {
                let value = self.interner.integers.value_of(*i).cloned().unwrap();
                value.to_string()
            }
            _ => panic!("Constant cannot serve as a destination address"),
        }
    }

    /// Turns a label into a human readable label string.
    fn label_readable(&self, label: &Label) -> String {
        format!("l{}", label.0)
    }
}

fn op_readable(op: &Op) -> String {
    match op {
        Op::Plus => "+".to_string(),
        Op::Mult => "*".to_string(),
    }
}

fn label(label: &Option<Label>, max_len: usize, default: &str) -> String {
    match label {
        Some(label) => {
            let l = format!("l{}:", label.0);
            let space = max_len - l.len();
            format!("{l}{}", " ".repeat(space))
        }

        None => default.to_owned(),
    }
}
