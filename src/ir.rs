use sym::{store, Symbol};
use uuid::Uuid;

use span::Span;

#[derive(Clone)]
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

#[derive(Clone)]
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
    Binop(Binop, Box<Exp>, Box<Exp>),
    Mem(Box<Exp>),
    Call(Box<Exp>, Vec<Exp>),
    ESeq(Box<Stm>, Box<Exp>),
}

pub enum Stm {
    Move(Exp, Exp),
    Exp(Exp),
    Jump(Exp, Vec<Label>),
    CJump(Relop, Exp, Exp, Label, Label),
    Seq(Box<Stm>, Box<Stm>),
    Label(Label),
    Source(Span),
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
