use std::fmt;

use sym::{store, Symbol};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Static {
    id: Uuid,
    label: Label,
    data: String,
}

impl Static {
    pub fn new(data: String) -> Self {
        Static {
            id: Uuid::new_v4(),
            label: Label::with_name("STRING"),
            data,
        }
    }

    pub fn label(&self) -> Label {
        self.label
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Temp {
    id: Uuid,    
    name: Symbol,
}

impl Temp {
    pub fn new() -> Self {
        Temp { id: Uuid::new_v4(), name: store("") }
    }

    pub fn with_name(name: &'static str) -> Self {
        Temp { id: Uuid::new_v4(), name: store(name) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Label {
    id: Uuid,    
    name: Symbol,
}

impl Label {
    pub fn new() -> Self {
        Label { id: Uuid::new_v4(), name: store("") }
    }

    pub fn with_name(name: &'static str) -> Self {
        Label { id: Uuid::new_v4(), name: store(name) }
    }

    pub fn with_symbol(name: Symbol) -> Self {
        Label { id: Uuid::new_v4(), name }
    }
}

pub enum Tree {
    Ex(Exp),
    Nx(Stm),
    Cx(Cond),
}

#[derive(Clone, Debug)]
pub enum Exp {
    Const(i32),
    Name(Label),
    Temp(Temp),
    Binop(Box<Exp>, Binop, Box<Exp>),
    Mem(Box<Exp>),
    Call(Box<Exp>, Vec<Exp>),
    ESeq(Box<Stm>, Box<Exp>),
}

impl From<Tree> for Exp {
    fn from(tree: Tree) -> Self {
        match tree {
        | Tree::Ex(exp) => exp,
        | Tree::Nx(stm) => {
            Exp::ESeq(
                Box::new(stm),
                Box::new(Exp::Const(0)),
            )
        },
        | Tree::Cx(gen_stm) => {
            let r = Temp::with_name("COND_EXP");
            let t = Label::with_name("TRUE_BRANCH");
            let f = Label::with_name("FALSE_BRANCH");
            Exp::ESeq(
                Box::new(Stm::Seq(vec![
                    Stm::Move(Exp::Const(1), Exp::Temp(r)),
                    gen_stm(t, f),
                    Stm::Label(f),
                    Stm::Move(Exp::Const(0), Exp::Temp(r)),
                    Stm::Label(t),
                ])),
                Box::new(Exp::Temp(r)),
            )
        },
        }
    }
}

impl From<Exp> for Tree {
    fn from(exp: Exp) -> Self {
        Tree::Ex(exp)
    }
}

#[derive(Clone, Debug)]
pub enum Stm {
    Move(Exp, Exp),
    Exp(Exp),
    Jump(Exp, Vec<Label>),
    CJump(Exp, Relop, Exp, Label, Label),
    Seq(Vec<Stm>),
    Label(Label),
    Comment(String),
}

impl From<Tree> for Stm {
    fn from(tree: Tree) -> Self {
        match tree {
        | Tree::Nx(stm) => stm,
        | Tree::Ex(exp) => Stm::Exp(exp),
        | Tree::Cx(gen_stm) => {
            let t = Label::with_name("TRUE_BRANCH");
            let f = Label::with_name("FALSE_BRANCH");
            gen_stm(t, f)
        },
        }
    }
}

impl From<Stm> for Tree {
    fn from(stm: Stm) -> Self {
        Tree::Nx(stm)
    }
}

pub type Cond = Box<Fn(Label, Label) -> Stm>;

impl From<Tree> for Cond {
    fn from(tree: Tree) -> Self {
        match tree {
        | Tree::Nx(_) => panic!("Internal compiler error: converting statement to conditional"),
        | Tree::Cx(gen_stm) => gen_stm,
        | Tree::Ex(exp) => {
            Box::new(move |t, f| Stm::CJump(
                Exp::Const(0),     
                Relop::Eq,
                exp.clone(),
                t,
                f,
            ))
        },
        }
    }
}

impl From<Cond> for Tree {
    fn from(cond: Cond) -> Self {
        Tree::Cx(cond)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    LShift,
    RShift,
    ARShift,
    XOr,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
        | Stm::Label(l)              => write!(fmt, "(LABEL {})", l),
        | Stm::Comment(c)            => write!(fmt, "(COMMENT {})", c),
        | Stm::Seq(stms)                => {
            write!(fmt, "(SEQ").unwrap();
            for stm in stms {
                write!(fmt, " {}", stm).unwrap();
            }
            write!(fmt, ")")
        },
        }
    }
}

impl fmt::Display for Binop {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Binop::Add     => write!(fmt, "ADD"),
        | Binop::Sub     => write!(fmt, "SUB"),
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
