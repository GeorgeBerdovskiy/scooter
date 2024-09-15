use super::Index;

/// Represents a label, which identifies the start of a chunk of code. Labels are used for many purposes,
/// such as functions, loops, and conditional branching.
#[derive(Clone)]
pub struct Label(pub Index);

/// Represents an address, which is either a name defined by the user, a constant value, or a temporary name we
/// generated ourselves. Note that the actual values are interned.
#[derive(Clone)]
pub enum Addr {
    Name(Index),
    Const(Index),
    Temp(Index),
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum Instr {
    Binary(BinInstr),
    Unary(UnInstr),
    Copy(CopyInstr),
    Call(CallInstr),
    Return(RetInstr),
}

impl Instr {
    /// Return the destination address of this instruction.
    pub fn da(&self) -> &Addr {
        match self {
            Instr::Binary(bin) => &bin.da,
            Instr::Unary(un) => &un.da,
            Instr::Copy(cop) => &cop.da,
            Instr::Call(call) => &call.da,
            Instr::Return(_) => panic!("Return statements don't have a destination address!"),
        }
    }

    /// Set the label of this instruction.
    pub fn set_label(&mut self, label: Label) {
        match self {
            Instr::Binary(bin) => bin.label = Some(label),
            Instr::Unary(un) => un.label = Some(label),
            Instr::Copy(cop) => cop.label = Some(label),
            Instr::Call(call) => call.label = Some(label),
            Instr::Return(ret) => ret.label = Some(label),
        }
    }
}

/// Represents an instruction of the form `<name|temp> = <addr> <op> <addr>`.
#[derive(Clone)]
pub struct BinInstr {
    /// The optional label.
    pub label: Option<Label>,

    /// The destination address, which absolutely **cannot** be a constant.
    pub da: Addr,

    /// The left operand (address), which can be any kind of address.
    pub la: Addr,

    /// The operator.
    pub op: Op,

    /// The right operand (address), which can be any kind of address.
    pub ra: Addr,
}

impl BinInstr {
    /// Create a new binary instruction.
    pub fn new(da: Addr, la: Addr, op: Op, ra: Addr) -> Self {
        BinInstr {
            label: None,
            da,
            la,
            op,
            ra,
        }
    }
}

/// Represents an instruction of the form `<name|temp> = <op> <addr>`.
#[derive(Clone)]
#[allow(dead_code)]
pub struct UnInstr {
    /// The optional label.
    pub label: Option<Label>,

    /// The destination address, which absolutely **cannot** be a constant.
    pub da: Addr,

    /// The operator.
    pub op: Op,

    /// The operand address, which can be any kind of address.
    pub ad: Addr,
}

/// Represents an operator. This is different from the source level operator construct.
#[derive(Clone)]
pub enum Op {
    Plus, // +
    Mult, // *
}

/// Represents an instruction of the form `<name|temp> = <addr>`.
#[derive(Clone)]
pub struct CopyInstr {
    /// The optional label.
    pub label: Option<Label>,

    /// The destination address, which absolutely **cannot** be a constant.
    pub da: Addr,

    /// The source address, which can be any kind of address.
    pub ad: Addr,
}

impl CopyInstr {
    pub fn new(da: Addr, ad: Addr) -> Self {
        CopyInstr {
            label: None,
            da,
            ad,
        }
    }
}

/// Represents an instruction of the form `ret ad`
#[derive(Clone)]
pub struct RetInstr {
    /// The optional label.
    pub label: Option<Label>,

    /// The address being returned.
    pub ad: Addr,
}

impl RetInstr {
    pub fn new(ad: Addr) -> Self {
        RetInstr { label: None, ad }
    }
}

/// Represents an instruction of the form `da = fl, n`
#[derive(Clone)]
#[allow(dead_code)]
pub struct CallInstr {
    /// The optional label.
    pub label: Option<Label>,

    /// The destination address, which absolutely **cannot** be a constant.
    pub da: Addr,

    /// The function label.
    pub fl: Label,

    /// The number of parameters.
    pub n: u32,
}

impl CallInstr {
    pub fn new(da: Addr, fl: Label, n: u32) -> Self {
        CallInstr {
            label: None,
            da,
            fl,
            n,
        }
    }
}
