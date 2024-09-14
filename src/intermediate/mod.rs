use instructions::{Addr, BinInstr, CallInstr, CopyInstr, Instr, Label, Op};
use mapper::Mapper;

use crate::ast::{visitor::Visit, Expr, ExprCall, File, OpKind, Stmt};

mod instructions;
mod mapper;
mod table;

/// Represents the index of an instruction in the list of instructions.
type Index = usize;

/// The IR representation of a program. Really just a fancy list of instructions right now. Later it will likely
/// become much more complicated!
pub struct IRRoot {
    pub instrs: Vec<Instr>,
}

/// Generates IR code given the AST for an entire program.
pub struct LoweringEngine<'a> {
    /// The source abstract syntax tree.
    ast: &'a File,

    /// The generated instructions.
    instrs: Vec<Instr>,

    /// Map from names to their indices.
    name_map: Mapper<'a>,

    /// Map from functions to their labels.
    fn_map: Mapper<'a>,

    /// The next available temporary address.
    next_temp: Index,
}

impl<'a> LoweringEngine<'a> {
    /// Create a new generator instance.
    pub fn new(ast: &'a File) -> Self {
        LoweringEngine {
            ast,
            instrs: Vec::new(),
            name_map: Mapper::new(),
            fn_map: Mapper::new(),
            next_temp: 0,
        }
    }

    /// Generate IR for the provided AST.
    pub fn generate(&mut self) -> IRRoot {
        self.visit_file(&self.ast);

        IRRoot {
            instrs: self.instrs.clone(),
        }
    }

    /// Generate instructions from an expression, which may need to be broken down first.
    fn process_expr(&mut self, expr: &'a Expr) -> Index {
        match expr {
            Expr::Binary(expr_bin) => {
                // Generate an instruction for the left, getting its index
                let li = self.process_expr(&expr_bin.lhs);

                // Generate an instruction for the left, getting its index
                let ri = self.process_expr(&expr_bin.rhs);

                // Create a new instruction and return its index
                let da = Addr::Temp(self.temp());

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
                    let ident = &expr_call_fn.ident.repr;

                    let da = Addr::Temp(self.temp());
                    let fl = Label(self.fn_map.find(ident));

                    self.instrs.push(Instr::Call(CallInstr::new(da, fl, 0)));
                    self.instrs.len() - 1
                }
            },
        }
    }

    /// Get the next free temporary address.
    fn temp(&mut self) -> Index {
        let index = self.next_temp;
        self.next_temp += 1;
        index
    }
}

impl<'a> Visit<'a> for LoweringEngine<'a> {
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

                let da = Addr::Temp(self.temp());
                (da, ad)
            }
        };

        self.instrs.push(Instr::Copy(CopyInstr::new(da, ad)));
    }

    fn visit_item_fn(&mut self, item_fn: &'a crate::ast::ItemFn) {
        let ident = &item_fn.ident.repr;

        // Move the mappers up a level
        self.fn_map.up();
        self.name_map.up();

        // Conver the function name into a label
        let label = self.fn_map.insert(&ident);

        // Take note of the next available instruction index
        let index = self.instrs.len();

        // Process all the statements in this function declaration
        self.visit_block(&item_fn.body);

        // Add the function label to the first instruction of the body
        self.instrs.get_mut(index).unwrap().set_label(Label(label));

        // Move the mappers down a level
        self.fn_map.down();
        self.name_map.down();
    }
}
