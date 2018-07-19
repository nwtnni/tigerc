use std::fmt;

use simple_symbol::{store, Symbol};

generate_counter!(LabelID, usize);
generate_counter!(TempID, usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

impl<'a> Into<Symbol> for &'a Label {
    fn into(self) -> Symbol {
        match self {
        | Label::Fixed(symbol) => *symbol,
        | Label::Unfixed{id, name} => store(&format!("{}_{}", name, id)),
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

impl fmt::Display for Temp {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Temp::Temp{id, name} => write!(fmt, "TEMP_{}_{}", name, id),
        | Temp::Reg(reg)       => write!(fmt, "TEMP_{:?}", reg),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
