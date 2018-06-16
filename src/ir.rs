use codespan::ByteSpan;
use uuid::Uuid;

#[derive(Clone)]
pub struct Temp {
    id: Uuid,    
    name: String,
}

#[derive(Clone)]
pub struct Label {
    id: Uuid,    
    name: String,
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
    Source(ByteSpan),
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
