use std::fmt;

use codespan::{ByteIndex, ByteSpan};

pub type ID = String;
pub type Span = ByteSpan;

pub fn to_span(start: ByteIndex, end: ByteIndex) -> ByteSpan {
    ByteSpan::new(start, end)
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

    Break(Span),

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

impl Binop {
    pub fn is_equality(&self) -> bool {
        match self {
        | Binop::Eq | Binop::Neq => true,
        _                        => false,
        }
    }

    pub fn is_comparison(&self) -> bool {
        match self {
        | Binop::Eq | Binop::Neq | Binop::Gt
        | Binop::Ge | Binop::Lt | Binop::Le => true,
        _                                   => false,
        }
    }
}

/// AST pretty printer
pub trait DisplayIndent {

    fn display_indent(&self, level: usize, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error>;

}

impl fmt::Display for Exp {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.display_indent(0, fmt)
    }
}

macro_rules! indent {
    ($fmt:expr, $level:expr, $str:expr) => { write!($fmt, "{}{}\n", "  ".repeat($level), $str)? }
}

macro_rules! enclose {
    ($fmt:expr, $level:expr, $block:block) => {
        indent!($fmt, $level, "(");
        $block
        indent!($fmt, $level, ")");
    }
}

// ### `Dec::Fun`
//
// ```
// (
//   <FUNDEC>
//   <FUNDEC>
// )
// ```
//
// ### `Dec::Var`
//
// ```
// (
//   var <NAME> : <TYPEID> :=
//   <INIT>
// )
// ```
//
// ### `Dec::Type`
//
// ```
// (
//   <TYPEDEC>
//   <TYPEDEC>
// )
// ```
impl DisplayIndent for Dec {

    fn display_indent(&self, level: usize, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {

        enclose!(fmt, level, {
            let level = level + 1;
            match self {
            | Dec::Var { name, ty, init, .. } => {

                match ty {
                | None     => indent!(fmt, level, format!("var {} :=", name)),
                | Some(ty) => indent!(fmt, level, format!("var {} : {} :=", name, ty)),
                };

                init.display_indent(level, fmt)?;
            },
            | Dec::Type(decs, _) => for d in decs { d.display_indent(level, fmt)?; },
            | Dec::Fun(decs, _)  => for d in decs { d.display_indent(level, fmt)?; },
            };
        });

        Ok(())
    }
}

// ### FunDec
//
// ```
// (
//   function <NAME> : <TYPEID>
//   (
//     <FIELDDEC>
//     <FIELDDEC>
//   )
//   =
//   <BODY>
// )
// ```
impl DisplayIndent for FunDec {
    fn display_indent(&self, level: usize, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {

        enclose!(fmt, level, {
            let level = level + 1;
            let FunDec { name, args, rets, body, .. } = self;

            match rets {
            | None      => indent!(fmt, level, format!("function {}", name)),
            | Some(ret) => indent!(fmt, level, format!("function {} : {}", name, ret)),
            };

            enclose!(fmt, level, {
                let level = level + 1;
                for a in args { a.display_indent(level, fmt)?; }
            });

            indent!(fmt, level, "=");
            body.display_indent(level, fmt)?;
        });

        Ok(())
    }
}

// ### `FieldDec`
//
// ```
// <NAME> : <TYPEID>
// ```
impl DisplayIndent for FieldDec {
    fn display_indent(&self, level: usize, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {

        let FieldDec { name, ty, .. } = self;
        indent!(fmt, level, format!("{} : {}", name, ty));

        Ok(())
    }
}

// ### `TypeDec`
//
// ```
// (
//   type <NAME> =
//   <TYPE>
// )
// ```
impl DisplayIndent for TypeDec {
    fn display_indent(&self, level: usize, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {

        enclose!(fmt, level, {
            let level = level + 1;
            let TypeDec { name, ty, .. } = self;
            indent!(fmt, level, format!("type {} =", name));
            ty.display_indent(level, fmt)?;
        });

        Ok(())
    }
}

// ### `Field`
//
// ```
// <NAME> = <EXP>
// ```
impl DisplayIndent for Field {
    fn display_indent(&self, level: usize, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {

        let Field { name, exp, .. } = self;
        indent!(fmt, level, format!("{} =", name));
        (**exp).display_indent(level, fmt)?;

        Ok(())
    }
}

// ### `Type::Name`
//
// ```
// <NAME>
// ```
//
// ### `Type::Rec`
//
// ```
// (
//   <FIELDDEC>
//   <FIELDDEC>
// )
// ```
//
// ### `Type::Arr`
//
// ```
// array of <NAME>
// ```
impl DisplayIndent for Type {
    fn display_indent(&self, level: usize, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {

        match self {
        | Type::Arr(name, _)  => indent!(fmt, level, format!("array of {}", name)),
        | Type::Name(name, _) => indent!(fmt, level, name),
        | Type::Rec(decs, _)  => {
            enclose!(fmt, level, {
                let level = level + 1;
                for d in decs { d.display_indent(level, fmt)?; }
            });
        },

        }

        Ok(())
    }
}

// ### `Var::Simple`
//
// ```
// <NAME>
// ```
//
// ### `Var::Field`
//
// ```
// (
//   <VAR>
//   .
//   <NAME>
// )
// ```
//
// ### `Var::Index`
//
// ```
// (
//   <VAR>
//   []
//   <EXP>
// )
//
//
impl DisplayIndent for Var {

    fn display_indent(&self, level: usize, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {

        match self {
        | Var::Simple(name, _) => indent!(fmt, level, name),
        | Var::Field(var, field, _) => {
            enclose!(fmt, level, {
                let level = level + 1;
                var.display_indent(level, fmt)?;
                indent!(fmt, level, ".");
                indent!(fmt, level, field);
            });
        },
        | Var::Index(var, idx, _) => {
            enclose!(fmt, level, {
                let level = level + 1;
                var.display_indent(level, fmt)?;
                indent!(fmt, level, "[]");
                (**idx).display_indent(level, fmt)?;
            });
        },
        }

        Ok(())

    }
}

// ### `Exp::Break`
//
// ```
// break
// ```
//
// ### `Exp::Nil`
//
// ```
// nil
// ```
//
// ### `Exp::Var`
//
// ```
// <VAR>
// ```
//
// ### `Exp::Int`
//
// ```
// <VALUE>
// ```
//
// ### `Exp::Str`
//
// ```
// "<VALUE>"
// ```
//
// ### `Exp::Call`
//
// ```
// (
//   call <NAME>
//   (
//     <EXP>
//     <EXP>
//   )
// )
// ```
//
// ### `Exp::Neg`
//
// ```
// (
//   -
//   <EXP>
// )
//
// ### `Exp::Bin`
//
// ```
// (
//   <EXP>
//   <OP>
//   <EXP>
// )
// ```
//
// ### `Exp::Rec`
//
// ```
// <NAME>
// (
//   <FIELD>
//   <FIELD>
// )
// ```
//
// ### `Exp::Seq`
//
// ```
// (
//   <EXP>
//   <EXP>
//   ...
// )
// ```
//
// ### `Exp::Ass`
//
// ```
// (
//   <VAR>
//   :=
//   <EXP>
// )
// ```
//
// ### `Exp::If`
//
// ```
// (
//   if
//   <EXP>
//   then
//   <EXP>
//   else
//   <EXP>
// )
// ```
//
// ### `Exp::While`
//
// ```
// (
//   while
//   <EXP>
//   do
//   <EXP>
// )
// ```
//
// ### `Exp::For`
//
// ```
// (
//   for <NAME> :=
//   <EXP>
//   to
//   <EXP>
//   do
//   <EXP>
// )
// ```
//
// ### `Exp::Let`
//
// ```
// (
//   let
//   (
//     <DEC>
//     <DEC>
//   )
//   in
//   <EXP>
// )
//
// ```
//
// ### `Exp::Arr`
//
// ```
// (
//   <NAME>
//   size
//   <EXP>
//   of
//   <EXP>
// )
// ```
impl DisplayIndent for Exp {

    fn display_indent(&self, level: usize, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {

        match self {
        | Exp::Break(_)    => { indent!(fmt, level, "break"); return Ok(()) }
        | Exp::Nil(_)      => { indent!(fmt, level, "nil"); return Ok(()) },
        | Exp::Var(var, _) => { var.display_indent(level, fmt)?; return Ok(()) },
        | Exp::Int(n, _)   => { indent!(fmt, level, n); return Ok(()) },
        | Exp::Str(s, _)   => { indent!(fmt, level, format!("\"{}\"", s)); return Ok(()) },
        | _                => (),
        };

        enclose!(fmt, level, {
            let level = level + 1;
            match self {
            | Exp::Call { name, args, .. } => {
                indent!(fmt, level, format!("call {}", name));
                enclose!(fmt, level, {
                    let level = level + 1;
                    for a in args { a.display_indent(level, fmt)?; }
                });
            },
            | Exp::Neg(exp, _) => {
                indent!(fmt, level, "-");
                (**exp).display_indent(level, fmt)?;
            },
            | Exp::Bin { lhs, op, rhs, .. } => {
                (**lhs).display_indent(level, fmt)?;
                op.display_indent(level, fmt)?;
                (**rhs).display_indent(level, fmt)?;
            },
            | Exp::Rec { name, fields, .. } => {
                indent!(fmt, level, name);
                enclose!(fmt, level, {
                    let level = level + 1;
                    for f in fields { f.display_indent(level, fmt)?; }
                });
            },
            | Exp::Seq(exps, _) => {
                for e in exps {
                    e.display_indent(level, fmt)?;
                }
            },
            | Exp::Ass { name, exp, .. } => {
                name.display_indent(level, fmt)?;
                indent!(fmt, level, ":=");
                (**exp).display_indent(level, fmt)?;
            },
            | Exp::If { guard, then, or, .. } => {
                indent!(fmt, level, "if");
                (**guard).display_indent(level, fmt)?;
                indent!(fmt, level, "then");
                (**then).display_indent(level, fmt)?;
                if let Some(or) = or {
                    indent!(fmt, level, "else");
                    (**or).display_indent(level, fmt)?;
                }
            },
            | Exp::While { guard, body, .. } => {
                indent!(fmt, level, "while");
                (**guard).display_indent(level, fmt)?;
                indent!(fmt, level, "do");
                (**body).display_indent(level, fmt)?;
            },
            | Exp::For { name, lo, hi, body, .. } => {
                indent!(fmt, level, format!("for {} :=", name));
                (**lo).display_indent(level, fmt)?;
                indent!(fmt, level, "to");
                (**hi).display_indent(level, fmt)?;
                indent!(fmt, level, "do");
                (**body).display_indent(level, fmt)?;
            },
            | Exp::Let { decs, body, .. } => {
                indent!(fmt, level, "let");
                enclose!(fmt, level, {
                    let level = level + 1;
                    for d in decs { d.display_indent(level, fmt)?; }
                });
                indent!(fmt, level, "in");
                (**body).display_indent(level, fmt)?;
            },
            | Exp::Arr { name, size, init, .. } => {
                indent!(fmt, level, name);
                indent!(fmt, level, "size");
                (**size).display_indent(level, fmt)?;
                indent!(fmt, level, "of");
                (**init).display_indent(level, fmt)?;
            },
            _ => panic!("Unreachable"),
            }
        });

        Ok(())
    }
}

impl DisplayIndent for Binop {

    fn display_indent(&self, level: usize, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let sym = match self {
        | Binop::Add  => "+",
        | Binop::Sub  => "-",
        | Binop::Mul  => "*",
        | Binop::Div  => "/",
        | Binop::Eq   => "=",
        | Binop::Neq  => "<>",
        | Binop::Lt   => "<",
        | Binop::Le   => "<=",
        | Binop::Gt   => ">",
        | Binop::Ge   => ">=",
        | Binop::LAnd => "&",
        | Binop::LOr  => "|",
        };

        indent!(fmt, level, sym);
        Ok(())
    }
}
