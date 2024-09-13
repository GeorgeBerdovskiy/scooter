use instructions::{Addr, BinInstr, CallInstr, CopyInstr, Instr, Label, Op};
use mapper::Mapper;

use crate::ast::{visitor::Visit, Expr, ExprCall, File, OpKind, Stmt};

mod instructions;
mod mapper;

/// Represents the index of an instruction in the list of instructions.
type Index = usize;

/// The IR representation of a program. Really just a fancy list of instructions right now. Later it will likely
/// become much more complicated!
pub struct IRRoot {
    pub instrs: Vec<Instr>,
}

/// Generates IR code given the AST for an entire program.
pub struct Generator<'a> {
    /// The source abstract syntax tree.
    ast: &'a File,

    /// The generated instructions.
    instrs: Vec<Instr>,

    /// Map from name indices to their source representations.
    name_map: Mapper<&'a str>,

    /// Map from literal indices to their source representations.
    lit_map: Mapper<i32>,

    /// Map from temporary addresses to... nothing!
    temp_map: Mapper<()>,

    /// Map from labels to... nothing!
    label_map: Mapper<()>,
}

impl<'a> Generator<'a> {
    /// Create a new generator instance.
    pub fn new(ast: &'a File) -> Self {
        Generator {
            ast,
            instrs: Vec::new(),
            name_map: Mapper::new(),
            lit_map: Mapper::new(),
            temp_map: Mapper::new(),
            label_map: Mapper::new(),
        }
    }

    /// Generate IR for the provided AST.
    pub fn generate(&mut self) -> IRRoot {
        IRRoot {
            instrs: self.instrs.clone(),
        }
    }

    /// Generate instructions from an expression, which may need to be broken down first.
    fn process_expr(&mut self, expr: &Expr) -> Index {
        match expr {
            Expr::Binary(expr_bin) => {
                // Generate an instruction for the left, getting its index
                let li = self.process_expr(&expr_bin.lhs);

                // Generate an instruction for the left, getting its index
                let ri = self.process_expr(&expr_bin.rhs);

                // Create a new instruction and return its index
                let da = Addr::Temp(self.temp_map.insert(()));

                let op = match expr_bin.op.kind {
                    OpKind::Add => Op::Plus,
                    OpKind::Multiply => Op::Mult,
                };

                let la = self.instrs[li].da().clone();
                let ra = self.instrs[ri].da().clone();

                self.instrs
                    .push(Instr::Binary(BinInstr::new(da, la, op, ra)));
                self.instrs.len() - 1
            }

            Expr::Call(expr_call) => match expr_call {
                ExprCall::Fn(expr_call_fn) => {
                    let da = Addr::Temp(self.temp_map.insert(()));
                    let fl = Label(self.label_map.insert(()));

                    self.instrs.push(Instr::Call(CallInstr::new(da, fl, 0)));
                    self.instrs.len() - 1
                }
            },
        }
    }
}

impl<'a> Visit<'a> for Generator<'a> {
    fn visit_stmt(&mut self, stmt: &'a crate::ast::Stmt) {
        let (ad, da) = match stmt {
            Stmt::Local(local) => {
                let i = self.process_expr(&local.expr);
                let ad = self.instrs[i].da().clone();

                let da = Addr::Name(self.name_map.insert(&local.ident.repr));
                (da, ad)
            }

            Stmt::Expr(expr) => {
                let i = self.process_expr(expr);
                let ad = self.instrs[i].da().clone();

                let da = Addr::Temp(self.temp_map.insert(()));
                (da, ad)
            }
        };

        self.instrs.push(Instr::Copy(CopyInstr::new(da, ad)));
    }
}
