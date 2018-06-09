use codespan::ByteIndex;

pub type ID = String;
pub type Span = (ByteIndex, ByteIndex);

pub fn to_span(l: usize, r: usize) -> Span {
    ((l as u32).into(), (r as u32).into())
}

#[derive(Debug)]
pub enum Dec {
    Fun(Vec<FunDec>, Span),

    Var {
        name: ID,
        escape: bool,
        ty: Option<ID>,
        init: Exp,
        span: Span,
    },

    Type(Vec<TypeDec>, Span),
}

#[derive(Debug)]
pub struct FunDec {
    pub name: ID,
    pub args: Vec<FieldDec>,
    pub rets: Option<ID>,
    pub body: Exp,
    pub span: Span,
}

#[derive(Debug)]
pub struct FieldDec {
    pub name: ID,
    pub escape: bool,
    pub ty: ID,
    pub span: Span,
}

#[derive(Debug)]
pub struct TypeDec {
    pub name: ID,
    pub ty: Type,
    pub span: Span,
}

#[derive(Debug)]
pub struct Field {
    pub name: ID,
    pub exp: Box<Exp>,
    pub span: Span,
}

#[derive(Debug)]
pub enum Type {

    Name(ID, Span),

    Rec(Vec<FieldDec>, Span),

    Arr(ID, Span),
}

#[derive(Debug)]
pub enum Var {

    Simple(ID, Span),

    Field(Box<Var>, ID, Span),

    Index(Box<Var>, Box<Exp>, Span),

}

#[derive(Debug)]
pub enum Exp {

    Unit(Span),

    Nil(Span),

    Var(Var, Span),

    Int(i32, Span),

    Str(String, Span),

    Call {
        name: ID,
        args: Vec<Exp>,
        span: Span,
    },

    Neg(Box<Exp>, Span),

    Bin {
        lhs: Box<Exp>,
        op: Binop,
        rhs: Box<Exp>,
        span: Span,
    },

    Rec {
        name: ID,
        fields: Vec<Field>,
        span: Span,
    },

    Seq(Vec<Exp>, Span),

    Ass {
        name: Var,
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
        name: ID,
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
        name: ID,
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
    LAnd,
    LOr,
}
