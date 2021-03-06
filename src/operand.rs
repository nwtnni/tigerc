use std::fmt;
use std::hash;

use simple_symbol::{store, Symbol};

generate_counter!(LabelID, usize);
generate_counter!(TempID, usize);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Imm {
    Int(i32),    
    Label(Label),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Label {
    Fixed(Symbol),
    Unfixed {
        id: usize,
        name: Symbol,
    }
}

impl Label {
    pub fn from_fixed(name: &'static str) -> Self {
        Label::Fixed(store(name))
    }

    pub fn from_str(name: &'static str) -> Self {
        Label::Unfixed { id: LabelID::next(), name: store(name) }
    }

    pub fn from_symbol(name: Symbol) -> Self {
        Label::Unfixed { id: LabelID::next(), name }
    }
}

impl Into<Symbol> for Label {
    fn into(self) -> Symbol {
        match self {
        | Label::Fixed(symbol) => symbol,
        | Label::Unfixed{id, name} => store(&format!("{}_{}", name, id)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Temp {
    Reg(Reg),
    Temp {
        id: usize,
        name: Symbol,
    },
}

impl Temp {
    pub fn from_str(name: &'static str) -> Self {
        Temp::Temp {
            id: TempID::next(),
            name: store(name),
        }
    }

    pub fn from_reg(reg: Reg) -> Self {
        Temp::Reg(reg)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Reg {
    RAX,
    RBX,
    RCX,
    RDX,
    RBP,
    RSP,
    RSI,
    RDI,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl Reg {
    pub fn is_callee_saved(&self) -> bool {
        match self {
        | Reg::R12
        | Reg::R13
        | Reg::R14
        | Reg::R15
        | Reg::RBX
        | Reg::RSP
        | Reg::RBP => true,
        _          => false,
        }
    }

    pub fn is_caller_saved(&self) -> bool {
        !self.is_callee_saved()
    }

    pub fn get_argument(i: usize) -> Self {
        match i {
        | 0 => Reg::RDI,
        | 1 => Reg::RSI,
        | 2 => Reg::RDX,
        | 3 => Reg::RCX,
        | 4 => Reg::R8,
        | 5 => Reg::R9,
        | _ => panic!("Internal error: can only pass 6 arguments in registers"),
        }
    }

    pub fn get_return() -> Self {
        Reg::RAX
    }
}

pub trait Operand: fmt::Display + Copy + Clone + fmt::Debug + PartialEq + Eq + hash::Hash {}
impl Operand for Temp {}
impl Operand for Reg {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mem<T: Operand> {
    R(T),
    RO(T, i32),
}

impl fmt::Display for Temp {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Temp::Temp{id, name} => write!(fmt, "TEMP_{}_{}", name, id),
        | Temp::Reg(reg)       => write!(fmt, "TEMP_{:?}", reg),
        }
    }
}
    
impl fmt::Display for Reg {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Reg::RAX => write!(fmt, "%rax"),
        | Reg::RBX => write!(fmt, "%rbx"),
        | Reg::RCX => write!(fmt, "%rcx"),
        | Reg::RDX => write!(fmt, "%rdx"),
        | Reg::RBP => write!(fmt, "%rbp"),
        | Reg::RSP => write!(fmt, "%rsp"),
        | Reg::RSI => write!(fmt, "%rsi"),
        | Reg::RDI => write!(fmt, "%rdi"),
        | Reg::R8  => write!(fmt, "%r8"),
        | Reg::R9  => write!(fmt, "%r9"),
        | Reg::R10 => write!(fmt, "%r10"),
        | Reg::R11 => write!(fmt, "%r11"),
        | Reg::R12 => write!(fmt, "%r12"),
        | Reg::R13 => write!(fmt, "%r13"),
        | Reg::R14 => write!(fmt, "%r14"),
        | Reg::R15 => write!(fmt, "%r15"),
        }
    }
}

impl fmt::Display for Imm {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Imm::Int(n) => write!(fmt, "${}", n),
        | Imm::Label(l) => write!(fmt, "${}", l),
        }
    }
}

impl fmt::Display for Label {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Label::Fixed(symbol) => write!(fmt, "{}", symbol),
        | Label::Unfixed{id, name} => write!(fmt, "{}_{}", name, id),
        }
    }
}

impl <T: Operand> fmt::Display for Mem<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Mem::R(reg)                         => write!(fmt, "({})", reg),
        | Mem::RO(reg, offset)                => write!(fmt, "{}({})", offset, reg),
        }
    }
}
