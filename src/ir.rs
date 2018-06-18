use std::fmt;

use sym::{store, Symbol};
use uuid::Uuid;

use span::Span;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Temp {
    id: Uuid,    
    name: Symbol,
}

impl Temp {
    pub fn new() -> Self {
        Temp { id: Uuid::new_v4(), name: store("TEMP") }
    }

    pub fn with_name(name: Symbol) -> Self {
        Temp { id: Uuid::new_v4(), name }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Label {
    id: Uuid,    
    name: Symbol,
}

impl Label {
    pub fn new() -> Self {
        Label { id: Uuid::new_v4(), name: store("LABEL") }
    }

    pub fn with_name(name: Symbol) -> Self {
        Label { id: Uuid::new_v4(), name }
    }
}

pub enum Exp {
    Const(i32),
    Name(Label),
    Temp(Temp),
    Binop(Box<Exp>, Binop, Box<Exp>),
    Mem(Box<Exp>),
    Call(Box<Exp>, Vec<Exp>),
    ESeq(Box<Stm>, Box<Exp>),
}

pub enum Stm {
    Move(Exp, Exp),
    Exp(Exp),
    Jump(Exp, Vec<Label>),
    CJump(Exp, Relop, Exp, Label, Label),
    Seq(Box<Stm>, Box<Stm>),
    Label(Label),
    Comment(String),
}

pub enum Binop {
    Plus,
    Minus,
    Mul,
    Div,
    And,
    Or,
    LShift,
    RShift,
    ARShift,
    XOr,
}

pub enum Relop {
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Ult,
    Ule,
    Ugt,
    Uge,
}

impl fmt::Display for Temp {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}_{}", self.name, self.id.simple())
    }
}

impl fmt::Display for Label {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}_{}", self.name, self.id.simple())
    }
}

impl fmt::Display for Exp {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Exp::Const(n)        => write!(fmt, "(CONST {})", n),
        | Exp::Name(l)         => write!(fmt, "(NAME {})", l),
        | Exp::Temp(t)         => write!(fmt, "(TEMP {})", t),
        | Exp::Binop(l, op, r) => write!(fmt, "(BINOP {} {} {})", l, op, r),
        | Exp::Mem(e)          => write!(fmt, "(MEM {})", e),
        | Exp::ESeq(s, e)      => write!(fmt, "(ESEQ {} {})", s, e),
        | Exp::Call(f, args)   => {
            write!(fmt, "(CALL {}", f).unwrap();
            for arg in args {
                write!(fmt, " {}", arg).unwrap();
            }
            write!(fmt, ")")
        },
        }
    }
}

impl fmt::Display for Stm {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Stm::Move(d, s)            => write!(fmt, "(MOVE {} {})", d, s),
        | Stm::Exp(e)                => write!(fmt, "(EXP {})", e),
        | Stm::Jump(e, _)            => write!(fmt, "(LABEL {})", e),
        | Stm::CJump(l, op, r, t, f) => write!(fmt, "(CJUMP {} {} {} {} {})", l, op, r, t, f),
        | Stm::Seq(s1, s2)           => write!(fmt, "(SEQ {} {})", s1, s2),
        | Stm::Label(l)              => write!(fmt, "(LABEL {})", l),
        | Stm::Comment(c)            => write!(fmt, "(COMMENT {})", c),
        }
    }
}

impl fmt::Display for Binop {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Binop::Plus    => write!(fmt, "ADD"),
        | Binop::Minus   => write!(fmt, "SUB"),
        | Binop::Mul     => write!(fmt, "MUL"),
        | Binop::Div     => write!(fmt, "DIV"),
        | Binop::And     => write!(fmt, "LAND"),
        | Binop::Or      => write!(fmt, "LOR"),
        | Binop::LShift  => write!(fmt, "LSHIFT"),
        | Binop::RShift  => write!(fmt, "RSHIFT"),
        | Binop::ARShift => write!(fmt, "ARSHIFT"),
        | Binop::XOr     => write!(fmt, "XOR"),
        }
    }
}

impl fmt::Display for Relop {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Relop::Eq  => write!(fmt, "EQ"),
        | Relop::Ne  => write!(fmt, "NE"),
        | Relop::Lt  => write!(fmt, "LT"),
        | Relop::Gt  => write!(fmt, "GT"),
        | Relop::Le  => write!(fmt, "LE"),
        | Relop::Ge  => write!(fmt, "GE"),
        | Relop::Ult => write!(fmt, "ULT"),
        | Relop::Ule => write!(fmt, "ULE"),
        | Relop::Ugt => write!(fmt, "UGT"),
        | Relop::Uge => write!(fmt, "UGE"),
        }
    }
}
