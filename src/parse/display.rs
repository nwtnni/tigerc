use std::fmt;

use ast::*;

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
