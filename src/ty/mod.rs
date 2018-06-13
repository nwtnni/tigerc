use codespan::ByteSpan;
use im::HashMap;
use uuid::Uuid;

use ast::*;
use error::{Error, TypeError};

#[derive(PartialEq, Eq, Clone)]
pub enum Ty {
    Nil,
    Int,
    Str,
    Unit,
    Arr(Box<Ty>, Uuid),
    Rec(Vec<(String, Ty)>, Uuid),
    Name(String, Option<Box<Ty>>),
}

#[derive(PartialEq, Eq)]
pub struct Typed {
    ty: Ty,
    _exp: (),
}

pub enum Binding {
    Var(Ty, bool),
    Fun(Vec<Ty>, Ty),
}

type Context<T> = HashMap<String, T>;
type TypeContext = Context<Ty>;
type VarContext = Context<Binding>;

fn ok(ty: Ty) -> Result<Typed, Error> {
    Ok(Typed { ty, _exp: () })
}

fn error<T>(span: &ByteSpan, err: TypeError) -> Result<T, Error> {
    Err(Error::semantic(*span, err))
}

pub struct Checker {
    loops: Vec<()>,
}

impl Checker {

    fn check_var(&self, vc: VarContext, tc: TypeContext, var: &Var) -> Result<Typed, Error> {

        unreachable!()
    }

    fn check_exp(&mut self, vc: VarContext, tc: TypeContext, exp: &Exp) -> Result<Typed, Error> {

        match exp {
        | Exp::Break(span)             => if self.loops.is_empty() { error(span, TypeError::Break) } else { ok(Ty::Unit) },
        | Exp::Nil(_)                  => ok(Ty::Nil),
        | Exp::Int(_, _)               => ok(Ty::Int),
        | Exp::Str(_, _)               => ok(Ty::Str),
        | Exp::Var(var, _)             => self.check_var(vc, tc, var),
        | Exp::Call {name, args, span} => {

            if !vc.contains_key(name) { return error(span, TypeError::UnboundFunction) }

            match &vc[name] {
            | Binding::Var(_, _) => error(span, TypeError::NotFunction),
            | Binding::Fun(args_ty, ret_ty) => {
                for (arg, ty) in args.iter().zip(args_ty) {
                    if &self.check_exp(vc.clone(), tc.clone(), arg)?.ty != ty {
                        return error(span, TypeError::CallMismatch)
                    }
                }
                ok(ret_ty.clone())
            },
            }
        },
        | Exp::Neg(exp, span) => if self.check_exp(vc, tc, exp)?.ty != Ty::Int { error(span, TypeError::Neg) } else { ok(Ty::Int) }
                
        | _ => unreachable!(),



        }

    }

    fn check_dec(&self, vc: VarContext, tc: TypeContext, dec: &Dec) -> Result<(VarContext, TypeContext), Error> {

        unreachable!()
    }

    fn check_type(&self, tc: TypeContext, ty: &Type) -> Result<Ty, Error> {

        unreachable!()
    }

}
