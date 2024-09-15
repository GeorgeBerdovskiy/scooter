use std::fs::File;
use std::io::{self, Write};

use crate::shared::Index;

pub mod instr;
pub mod lower;
mod mapper;
mod table;

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
                    let da = self.ad_readable(&bin.da, true);
                    let la = self.ad_readable(&bin.la, false);
                    let op = op_readable(&bin.op);
                    let ra = self.ad_readable(&bin.ra, false);

                    let pad = label(&bin.label, max_length, &label_padding);

                    writeln!(file, "{pad}{da} = {la} {op} {ra}")?;
                }

                Instr::Copy(cop) => {
                    let da = self.ad_readable(&cop.da, true);
                    let ad = self.ad_readable(&cop.ad, false);
                    let pad = label(&cop.label, max_length, &label_padding);

                    writeln!(file, "{pad}{da} = {ad}")?;
                }

                Instr::Return(ret) => {
                    let ad = self.ad_readable(&ret.ad, false);
                    let pad = label(&ret.label, max_length, &label_padding);

                    writeln!(file, "{pad}ret {ad}")?;
                }

                _ => writeln!(file, "INSTR")?,
            }
        }

        Ok(())
    }

    fn ad_readable(&self, ad: &Addr, is_d: bool) -> String {
        match ad {
            Addr::Name(i) => format!("x{i}"),
            Addr::Temp(i) => format!("t{i}"),
            Addr::Const(i) if !is_d => {
                let value = self.interner.integers.value_of(*i).cloned().unwrap();
                value.to_string()
            }
            _ => panic!("Constant cannot serve as a destination address"),
        }
    }
}

fn op_readable(op: &Op) -> String {
    match op {
        Op::Plus => "+".to_string(),
        Op::Mult => "*".to_string(),
    }
}

fn label(label: &Option<Label>, max_len: usize, default: &String) -> String {
    match label {
        Some(label) => {
            let l = format!("L{}:", label.0);
            let space = max_len - l.len();
            format!("{l}{}", " ".repeat(space))
        }

        None => default.clone(),
    }
}
