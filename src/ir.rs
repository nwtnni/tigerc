use std::fmt;

use asm;
use translate::Frame;
use operand::*;

#[derive(Debug)]
pub struct Unit {
    pub data: Vec<Static>,
    pub label: Label,
    pub body: Vec<Stm>,
    pub escapes: usize,
}

impl Unit {
    pub fn new(frame: Frame, data: Vec<Static>, body: Tree) -> Self {
        Unit {
            data,
            label: frame.label,
            escapes: frame.escapes,
            body: vec![
                Stm::Seq(frame.prologue),
                Stm::Move(
                    body.into(),
                    Exp::Temp(Temp::Reg(Reg::get_return())),
                ),
            ],
        }
    }

    pub fn map(self, f: impl Fn(Vec<Stm>) -> Vec<Stm>) -> Self {
        Unit {
            data: self.data,
            label: self.label,
            body: f(self.body),
            escapes: self.escapes,
        }
    }

    pub fn and_then(self, f: impl Fn(Self) -> Self) -> Self {
        f(self)
    }
}

generate_counter!(StaticID, usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Static {
    id: usize,
    label: Label,
    data: String,
}

impl Static {
    pub fn new(data: String) -> Self {
        Static {
            id: StaticID::next(),
            label: Label::from_str("STRING"),
            data,
        }
    }

    pub fn label(&self) -> Label {
        self.label
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
            let r = Temp::from_str("COND_EXP");
            let t = Label::from_str("TRUE_BRANCH");
            let f = Label::from_str("FALSE_BRANCH");
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
            let t = Label::from_str("TRUE_BRANCH");
            let f = Label::from_str("FALSE_BRANCH");
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
                Relop::Ne,
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
    XOr,
}

impl Binop {
    pub fn is_asm_binop(&self) -> bool {
        match self {
        | Binop::Add | Binop::Sub | Binop::And | Binop::Or | Binop::XOr => true,
        | _ => false,
        }
    }

    pub fn into_asm_binop(&self) -> asm::Binop {
        match self {
        | Binop::Add => asm::Binop::Add,
        | Binop::Sub => asm::Binop::Sub,
        | Binop::And => asm::Binop::And,
        | Binop::Or => asm::Binop::Or,
        | Binop::XOr => asm::Binop::XOr,
        | _ => panic!("Internal error: converting non-asm binop"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Relop {
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}



impl fmt::Display for Unit {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {

        write!(fmt, "{}", self.label)?;

        for stm in &self.body {
            write!(fmt, "\n    {}", stm)?;
        }
        
        Ok(())
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
        | Stm::Jump(e, _)            => write!(fmt, "(JUMP {})", e),
        | Stm::CJump(l, op, r, t, f) => write!(fmt, "(CJUMP {} {} {} {} {})", l, op, r, t, f),
        | Stm::Label(l)              => write!(fmt, "(LABEL {})", l),
        | Stm::Comment(c)            => write!(fmt, "(COMMENT {})", c),
        | Stm::Seq(stms)                => {
            write!(fmt, "(SEQ").unwrap();
            for stm in stms {
                write!(fmt, "\n    {}", stm).unwrap();
            }
            write!(fmt, "\n)")
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
        }
    }
}
