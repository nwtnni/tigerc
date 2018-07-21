use std::fmt;

use operand::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Asm<T: Operand> {
    Mov(Binary<T>),
    Bin(Binop, Binary<T>),
    Mul(Unary<T>),
    Div(Unary<T>),
    Un(Unop, Unary<T>),
    Pop(Unary<T>),
    Push(Unary<T>),
    Lea(Mem<T>, T),
    Cmp(Relop, Binary<T>),
    Jmp(Label),
    Jcc(Relop, Label),
    Call(Label),
    Cqo,
    Ret,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Binary<T: Operand> {
    IR(Imm, T),
    IM(Imm, Mem<T>),
    RM(T, Mem<T>),
    MR(Mem<T>, T),
    RR(T, T),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Unary<T: Operand> {
    R(T),
    M(Mem<T>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Unop {
    Inc,
    Dec,
    Not, 
    Neg,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Binop {
    Add,
    Sub,
    And,
    Or,
    XOr,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Relop {
    E,
    Ne,
    Z,
    G,
    Ge,
    L,
    Le,
}

impl <T: Operand> fmt::Display for Asm<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Asm::Mov(bin)      => write!(fmt, "movq {}", bin),
        | Asm::Bin(op, bin)  => write!(fmt, "{} {}", op, bin),
        | Asm::Mul(un)       => write!(fmt, "imulq {}", un),
        | Asm::Div(un)       => write!(fmt, "idivq {}", un),
        | Asm::Un(op, un)    => write!(fmt, "{} {}", op, un),
        | Asm::Pop(un)       => write!(fmt, "popq {}", un),
        | Asm::Push(un)      => write!(fmt, "pushq {}", un),
        | Asm::Lea(mem, reg) => write!(fmt, "leaq {}, {}", mem, reg),
        | Asm::Cmp(op, bin)  => write!(fmt, "j{} {}", op, bin),
        | Asm::Jmp(name)     => write!(fmt, "jmp {}", name),
        | Asm::Jcc(op, name) => write!(fmt, "j{} {}", op,  name),
        | Asm::Call(name)    => write!(fmt, "callq {}", name),
        | Asm::Cqo           => write!(fmt, "cqo"),
        | Asm::Ret           => write!(fmt, "retq"),
        }
    }
}

impl <T: Operand> fmt::Display for Binary<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Binary::IR(imm, reg)     => write!(fmt, "{}, {}", imm, reg),
        | Binary::IM(imm, mem)     => write!(fmt, "{}, {}", imm, mem),
        | Binary::RM(reg, mem)     => write!(fmt, "{}, {}", reg, mem),
        | Binary::MR(mem, reg)     => write!(fmt, "{}, {}", mem, reg),
        | Binary::RR(reg_a, reg_b) => write!(fmt, "{}, {}", reg_a, reg_b),
        }
    }
}

impl <T: Operand> fmt::Display for Unary<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Unary::R(reg) => write!(fmt, "{}", reg),
        | Unary::M(mem) => write!(fmt, "{}", mem),
        }
    }
}

impl fmt::Display for Unop {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Unop::Inc => write!(fmt, "incq"),
        | Unop::Dec => write!(fmt, "decq"),
        | Unop::Not => write!(fmt, "notq"),
        | Unop::Neg => write!(fmt, "negq"),
        }
    }
}

impl fmt::Display for Binop {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Binop::Add => write!(fmt, "addq"),
        | Binop::Sub => write!(fmt, "subq"),
        | Binop::And => write!(fmt, "andq"),
        | Binop::Or  => write!(fmt, "orq"),
        | Binop::XOr => write!(fmt, "xorq"),
        }
    }
}

impl fmt::Display for Relop {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Relop::E  => write!(fmt, "e"),
        | Relop::Ne => write!(fmt, "ne"),
        | Relop::Z  => write!(fmt, "z"),
        | Relop::G  => write!(fmt, "g"),
        | Relop::Ge => write!(fmt, "ge"),
        | Relop::L  => write!(fmt, "l"),
        | Relop::Le => write!(fmt, "le"),
        }
    }
}
