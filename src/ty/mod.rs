use im::HashMap;
use uuid::Uuid;

use ast::*;
use error::{Error, TypeError};

pub enum Ty {
    Nil,
    Int,
    Str,
    Unit,
    Arr(Box<Ty>, Uuid),
    Rec(Vec<(String, Ty)>, Uuid),
    Name(String, Option<Box<Ty>>),
}

pub struct Typed {
    ty: Ty,
    exp: (),
}

pub enum Binding {
    Var(Ty),
    Fun(Vec<Ty>, Ty),
}

type Context<T> = HashMap<String, T>;
type TypeContext = Context<Ty>;
type VarContext = Context<Binding>;

pub fn check_var(vc: VarContext, tc: TypeContext, var: Var) -> Result<Typed, Error> {

    unreachable!()
}

pub fn check_exp(vc: VarContext, tc: TypeContext, exp: Exp) -> Result<Typed, Error> {

    unreachable!()
}

pub fn check_dec(vc: VarContext, tc: TypeContext, dec: Dec) -> Result<(VarContext, TypeContext), Error> {

    unreachable!()
}

pub fn check_type(tc: TypeContext, ty: Type) -> Result<Ty, Error> {

    unreachable!()
}
