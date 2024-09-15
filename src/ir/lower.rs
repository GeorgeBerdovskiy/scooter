use crate::ast::visitor::*;
use crate::ast::*;
use crate::ir::instr::*;
use crate::shared::{Index, Pool};

use super::mapper::Mapper;
use super::IRRoot;

/// Groups pools for various literals into one central pool.
#[derive(Clone)]
#[allow(dead_code)]
pub struct LoweringPool<'a> {
    /// The integer interner.
    pub integers: Pool<i32>,

    /// The boolean interner.
    pub booleans: Pool<bool>,

    /// The string interner.
    pub strings: Pool<&'a str>,
}

impl LoweringPool<'_> {
    /// Create an empty lowering pool.
    pub fn new() -> Self {
        LoweringPool {
            integers: Pool::new(),
            booleans: Pool::new(),
            strings: Pool::new(),
        }
    }
}

/// Lowers an abstract syntax tree for an entire program to the Wheel intermediate representation.
pub struct LoweringEngine<'a> {
    /// The source abstract syntax tree.
    ast: &'a File,

    /// The generated instructions.
    instrs: Vec<Instr>,

    /// Map from names to their indices.
    name_map: Mapper<'a>,

    /// Map from functions to their labels.
    fn_map: Mapper<'a>,

    /// The lowering pool.
    pool: LoweringPool<'a>,

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
            pool: LoweringPool::new(),
            next_temp: 0,
        }
    }

    /// Generate IR for the provided AST.
    pub fn lower(&mut self) -> IRRoot {
        self.visit_file(self.ast);

        IRRoot {
            last_label: self.fn_map.next - 1,
            interner: self.pool.clone(),
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
                    // First, we need to add a parameter instruction for every argument passed to this function
                    self.process_args(&expr_call_fn.args);

                    let ident = &expr_call_fn.ident.repr;

                    let da = Addr::Temp(self.temp());
                    let fl = Label(self.fn_map.find(ident));

                    self.instrs
                        .push(Instr::Call(CallInstr::new(da, fl, expr_call_fn.args.len())));
                    self.instrs.len() - 1
                }
            },

            Expr::Ident(ident) => {
                let index = self.name_map.find(&ident.repr);

                let da = Addr::Temp(self.temp());
                let ad = Addr::Name(index);

                self.instrs.push(Instr::Copy(CopyInstr::new(da, ad)));
                self.instrs.len() - 1
            }

            Expr::Lit(expr_lit) => match expr_lit {
                ExprLit::Num(lit_num) => {
                    let index = self.pool.integers.insert(lit_num.value);

                    let da = Addr::Temp(self.temp());
                    let ad = Addr::Const(index);

                    self.instrs.push(Instr::Copy(CopyInstr {
                        label: None,
                        da,
                        ad,
                    }));
                    self.instrs.len() - 1
                }
            },
        }
    }

    fn process_args(&mut self, args: &'a ArgList) {
        for arg in &args.args {
            // Generate an instruction for the expression, getting its index
            let i = self.process_expr(&arg);

            // Get the destination address of this expression
            let ad = self.instrs[i].da().clone();

            // Use the destination address in the parameter instruction
            self.instrs
                .push(Instr::Param(ParamInstr { label: None, ad }))
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
        match stmt {
            Stmt::Local(local) => {
                let i = self.process_expr(&local.expr);
                let ad = self.instrs[i].da().clone();

                let da = Addr::Name(self.name_map.insert(&local.ident.repr));
                self.instrs.push(Instr::Copy(CopyInstr::new(da, ad)));
            }

            Stmt::Expr(expr) => {
                let i = self.process_expr(expr);
                let ad = self.instrs[i].da().clone();

                let da = Addr::Temp(self.temp());
                self.instrs.push(Instr::Copy(CopyInstr::new(da, ad)));
            }

            Stmt::Return(ret) => {
                let i = self.process_expr(&ret.expr);
                let ad = self.instrs[i].da().clone();

                self.instrs.push(Instr::Return(RetInstr::new(ad)));
            }
        };
    }

    fn visit_item_fn(&mut self, item_fn: &'a crate::ast::ItemFn) {
        let ident = &item_fn.ident.repr;

        // Move the name mapper up a level
        self.name_map.up();

        // Conver the function name into a label
        let label = self.fn_map.insert(ident);

        // Take note of the next available instruction index
        let index = self.instrs.len();

        // Process all the statements in this function declaration
        self.visit_block(&item_fn.body);

        // Add the function label to the first instruction of the body
        self.instrs.get_mut(index).unwrap().set_label(Label(label));

        // Move the name mapper down a level
        self.name_map.down();
    }
}
