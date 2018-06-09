use codespan::ByteIndex;

pub type Symbol = String;
pub type Span = (ByteIndex, ByteIndex);

pub fn to_span(l: usize, r: usize) -> Span {
    ((l as u32).into(), (r as u32).into())
}

#[derive(Debug)]
pub enum Dec {
    Fun(Vec<Fun>, Span),

    Var {
        name: Symbol,
        escape: bool,
        ty: Option<Symbol>,
        init: Exp,
        span: Span,
    },

    Type {
        name: Symbol,
        ty: Type,
        span: Span,
    },
}

#[derive(Debug)]
pub enum Type {

    Name(Symbol, Span),

    Rec(Vec<Field>, Span),

    Arr(Symbol, Span),
}

#[derive(Debug)]
pub struct Fun {
    pub name: Symbol,
    pub args: Vec<Field>,
    pub rets: Option<Symbol>,
    pub body: Exp,
}

#[derive(Debug)]
pub struct Field {
    pub name: Symbol,
    pub escape: bool,
    pub ty: Symbol,
}

#[derive(Debug)]
pub enum Var {

    Simple(Symbol),

    Field(Symbol),

    Sub(Symbol),

}

#[derive(Debug)]
pub enum Exp {

    Nil(Span),

    Var(Var, Span),

    Int(i32, Span),

    Str(String, Span),

    Call {
        name: Symbol,
        args: Vec<Exp>,
        span: Span,
    },

    Bin {
        lhs: Box<Exp>,
        op: Binop,
        rhs: Box<Exp>,
        span: Span,
    },

    Rec {
        name: Symbol,
        fields: Vec<(Symbol, Exp)>,
        span: Span,
    },

    Seq(Vec<Exp>, Span),

    Ass {
        name: Symbol,
        exp: Box<Exp>,
        span: Span,
    },

    If {
        guard: Box<Exp>,
        then: Box<Exp>,
        or: Option<Box<Exp>>,
        span: Span,
    },

    While {
        guard: Box<Exp>,
        body: Box<Exp>,
        span: Span,
    },

    For {
        name: Symbol,
        escape: bool,
        lo: Box<Exp>,
        hi: Box<Exp>,
        body: Box<Exp>,
        span: Span,
    },

    Break(Span),

    Let {
        decs: Vec<Dec>,
        body: Box<Exp>,
        span: Span,
    },

    Arr {
        ty: Symbol,
        size: Box<Exp>,
        init: Box<Exp>,
        span: Span,
    },
}

#[derive(Debug)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
}
