mod context;

use codespan::ByteSpan;
use uuid::Uuid;

use ast::*;
use error::{Error, TypeError};

pub use ty::context::*;

#[derive(Debug, Eq, Clone)]
pub enum Ty {
    Nil,
    Int,
    Str,
    Unit,
    Arr(Box<Ty>, Uuid),
    Rec(Vec<(String, Ty)>, Uuid),
    Name(String, Option<Box<Ty>>),
}

impl Ty {
    pub fn subtypes(&self, rhs: &Self) -> bool {
        match (self, rhs) {
        | (Ty::Nil, Ty::Rec(_, _)) => true,
        | _                        => self == rhs,
        }
    }
}

impl PartialEq for Ty {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
        | (Ty::Int, Ty::Int)
        | (Ty::Str, Ty::Str)
        | (Ty::Nil, Ty::Nil)
        | (Ty::Unit, Ty::Unit) => true,
        | (Ty::Arr(_, lid), Ty::Arr(_, rid))
        | (Ty::Rec(_, lid), Ty::Rec(_, rid)) => lid == rid,
        | (Ty::Name(_, _), _)
        | (_, Ty::Name(_, _)) => panic!("Internal error: should never compare names"),
        _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Typed {
    ty: Ty,
    _exp: (),
}

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

    pub fn check(ast: &Exp) -> Result<(), Error> {
        let mut checker = Checker { loops: Vec::new() };
        let _ = checker.check_exp(VarContext::default(), TypeContext::default(), ast)?;
        Ok(())
    }

    fn check_var(&mut self, vc: VarContext, tc: TypeContext, var: &Var) -> Result<Typed, Error> {

        macro_rules! is_int {
            ($exp:expr) => { self.check_exp(vc.clone(), tc.clone(), $exp)?.ty == Ty::Int }
        }

        match var {
        | Var::Simple(name, span) => ok(vc.get_var(span, name)?),
        | Var::Field(rec, field, span) => {

            // Must be bound to record type
            match self.check_var(vc, tc.clone(), &*rec)?.ty {
            | Ty::Rec(fields, _) => {

                // Find corresponding field
                let ty = fields.iter()
                    .find(|(name, _)| field == name)
                    .map(|(_, ty)| tc.trace_full(ty));

                match ty {
                | Some(ty) => ok(ty.clone()),
                | None     => error(span, TypeError::UnboundField),
                }
            },
            | _ => error(span, TypeError::NotRecord),
            }
        },
        | Var::Index(arr, index, span) => {

            // Index must be integer
            if !is_int!(&*index) {
                return error(span, TypeError::IndexMismatch)
            }

            // Get element type
            match self.check_var(vc, tc, &*arr)?.ty {
            | Ty::Arr(elem, _) => ok(*elem.clone()),
            | _                => error(span, TypeError::NotArr),
            }
        },
        }
    }

    fn check_exp(&mut self, vc: VarContext, tc: TypeContext, exp: &Exp) -> Result<Typed, Error> {

        macro_rules! is_int {
            ($exp:expr) => { self.check_exp(vc.clone(), tc.clone(), $exp)?.ty == Ty::Int }
        }

        macro_rules! is_unit {
            ($exp:expr) => { self.check_exp(vc.clone(), tc.clone(), $exp)?.ty == Ty::Unit }
        }

        match exp {
        | Exp::Nil(_)      => ok(Ty::Nil),
        | Exp::Int(_, _)   => ok(Ty::Int),
        | Exp::Str(_, _)   => ok(Ty::Str),
        | Exp::Var(var, _) => self.check_var(vc, tc, var),
        | Exp::Break(span) => {

            if self.loops.is_empty() {
                return error(span, TypeError::Break)
            }

            ok(Ty::Unit)

        },
        | Exp::Call{name, args, span} => {

            let (args_ty, ret_ty) = vc.get_fun(span, name)?;

            if args.len() != args_ty.len() {
                return error(span, TypeError::CallMismatch)
            }

            for (arg, ty) in args.iter().zip(args_ty) {
                if self.check_exp(vc.clone(), tc.clone(), arg)?.ty.subtypes(&ty) {
                    return error(span, TypeError::CallMismatch)
                }
            }

            ok(ret_ty.clone())
        },
        | Exp::Neg(exp, span) => {

            if !is_int!(&*exp) { return error(span, TypeError::Neg) }

            ok(Ty::Int)

        },
        | Exp::Bin{lhs, op, rhs, span} => {

            let lt = self.check_exp(vc.clone(), tc.clone(), lhs)?.ty;
            let rt = self.check_exp(vc, tc, rhs)?.ty;

            // No binary operators work on unit
            if lt == Ty::Unit || rt == Ty::Unit {
                return error(span, TypeError::BinaryMismatch)
            }

            // Equality checking is valid for any L<>R, L=R where R: L
            if op.is_equality() && lt.subtypes(&rt) {
                return ok(Ty::Int)
            }

            // Comparisons are valid for
            // - Str and Str
            // - Int and Int
            if op.is_comparison() && (lt == Ty::Int || lt == Ty::Str) && lt == rt {
                return ok(Ty::Int)
            }

            // Arithmetic is valid for
            // - Int and Int
            if lt == Ty::Int && rt == Ty::Int {
                return ok(Ty::Int)
            }

            error(span, TypeError::BinaryMismatch)
        },
        | Exp::Rec{name,fields,span} => {

            match tc.get_full(span, name)? {
            | Ty::Rec(fields_ty, _) => {

                if fields.len() != fields_ty.len() {
                    return error(span, TypeError::FieldMismatch)
                }

                // Make sure all record fields are fully resolved
                let fields_ty = fields_ty.iter().map(|(name, ty)| (name, tc.trace_full(ty)));

                // Check all field name - value pairs
                for (field, (field_name, field_ty)) in fields.iter().zip(fields_ty) {

                    let exp_ty = self.check_exp(vc.clone(), tc.clone(), &*field.exp)?.ty;

                    if &field.name != field_name && !exp_ty.subtypes(&field_ty) {
                        return error(span, TypeError::FieldMismatch)
                    }
                }

                ok(tc.get_full(span, name)?)
            },
            | _ => error(span, TypeError::NotRecord),
            }
        },
        | Exp::Seq(exps, _) => {

            // Empty sequence is just unit
            if exps.len() == 0 {
                return ok(Ty::Unit)
            }

            // Make sure all intermediate steps return unit
            if exps.len() > 1 {
                for i in 0..exps.len() - 1 {
                    self.check_exp(vc.clone(), tc.clone(), &exps[i])?;
                }
            }

            // Result is type of last exp
            self.check_exp(vc, tc, &exps.last().unwrap())
        },
        | Exp::Ass{name, exp, span} => {

            let var = self.check_var(vc.clone(), tc.clone(), name)?.ty;

            if !self.check_exp(vc, tc, exp)?.ty.subtypes(&var) {
                return error(span, TypeError::VarMismatch)
            }

            ok(Ty::Unit)
        },
        | Exp::If{guard, then, or, span} => {

            // Guard must be boolean
            if !is_int!(&*guard) {
                return error(span, TypeError::GuardMismatch)
            }

            // Check type of if branch
            let then_ty = self.check_exp(vc.clone(), tc.clone(), &*then)?.ty;

            if let Some(exp) = or {

                // For if-else, both branches must return the same type
                let or_ty = self.check_exp(vc, tc, &*exp)?.ty;
                if !then_ty.subtypes(&or_ty) && !or_ty.subtypes(&then_ty) {
                    return error(span, TypeError::BranchMismatch)
                }

                ok(then_ty.clone())

            } else {

                // For if, branch must have no expression
                if then_ty != Ty::Unit {
                    return error(span, TypeError::UnusedBranch)
                }

                ok(Ty::Unit)
            }
        },
        | Exp::While{guard, body, span} => {

            // Guard must be boolean
            if !is_int!(&*guard) {
                return error(span, TypeError::GuardMismatch)
            }

            // Enter loop body
            self.loops.push(());

            // Body must be unit
            if !is_unit!(&*body) {
                return error(span, TypeError::UnusedWhileBody)
            }

            ok(Ty::Unit)
        },
        | Exp::For{name, lo, hi, body, span, ..} => {

            if !is_int!(&*lo) {
                return error(span, TypeError::ForBound)
            }

            if !is_int!(&*hi) {
                return error(span, TypeError::ForBound)
            }

            // Bind loop variable as immutable
            let for_vc = vc.insert(name.clone(), Binding::Var(Ty::Int, false));

            // Enter loop body
            self.loops.push(());

            // Check body with updated VarContext
            if self.check_exp(for_vc, tc, &*body)?.ty != Ty::Unit {
                return error(span, TypeError::UnusedForBody)
            }

            ok(Ty::Unit)
        },
        | Exp::Let{decs, body, ..} => {

            let (mut let_vc, mut let_tc) = (vc.clone(), tc.clone());

            for dec in decs {
                let (new_vc, new_tc) = self.check_dec(let_vc, let_tc, &*dec)?;
                let_vc = new_vc;
                let_tc = new_tc;
                println!("{:#?}", let_tc);
            }

            self.check_exp(let_vc, let_tc, &*body)
        },
        | Exp::Arr{name, size, init, span} => {

            let elem = match tc.get_full(span, name)? {
            | Ty::Arr(elem, _) => *elem,
            | _                => return error(span, TypeError::NotArr),
            };

            if !is_int!(&*size) {
                return error(span, TypeError::ForBound)
            }

            if self.check_exp(vc.clone(), tc.clone(), &*init)?.ty.subtypes(&elem) {
                return error(span, TypeError::ArrMismatch)
            }

            ok(tc.get_full(span, name)?)
        },
        }
    }

    fn check_dec(&mut self, mut vc: VarContext, mut tc: TypeContext, dec: &Dec) -> Result<(VarContext, TypeContext), Error> {
        match dec {
        | Dec::Fun(funs, span) => {

            // Initialize top-level bindings
            for fun in funs {

                let mut args = Vec::new();

                // Get formal parameter types
                for arg in &fun.args {
                    args.push(tc.get_full(span, &arg.ty)?);
                }

                let ret = match &fun.rets {
                | None => Ty::Unit,
                | Some(name) => tc.get_full(span, name)?,
                };

                vc.insert_mut(fun.name.clone(), Binding::Fun(args, ret));
            }

            // Evaluate bodies with new bindings
            for fun in funs {

                let mut body_vc = vc.clone();

                // Add parameter bindings to body context
                for arg in &fun.args {
                    let arg_ty = tc.get_full(span, &arg.ty)?;
                    body_vc.insert_mut(arg.name.clone(), Binding::Var(arg_ty, true));
                }

                // Evaluate body with updated context
                let body_ty = self.check_exp(body_vc, tc.clone(), &fun.body)?.ty;
                let ret_ty = if let Some(ret) = &fun.rets { tc.get_full(span, ret)? } else { Ty::Unit };

                if !body_ty.subtypes(&ret_ty) {
                    return error(&fun.span, TypeError::ReturnMismatch)
                }
            }

            Ok((vc, tc))
        },
        | Dec::Var{name, ty, init, span, ..} => {

            let init_ty = self.check_exp(vc.clone(), tc.clone(), &init)?.ty;

            // Can't assign nil without type annotation
            if init_ty == Ty::Nil && ty.is_none() {
                return error(span, TypeError::UnknownNil)
            }

            match ty {
            | None     => Ok((vc.insert(name.clone(), Binding::Var(init_ty.clone(), true)), tc)),
            | Some(id) => {

                let name_ty = tc.get_full(span, id)?;
                if !init_ty.subtypes(&name_ty) {
                    return error(span, TypeError::VarMismatch)
                }

                Ok((vc.insert(name.clone(), Binding::Var(name_ty, true)), tc))
            },
            }
        },
        | Dec::Type(decs, _) => {

            // Initialize top-level declarations
            for dec in decs {
                tc.insert_mut(dec.name.clone(), Ty::Name(dec.name.clone(), None));
            }

            // Fill in type bodies
            for dec in decs {
                let ty = self.check_type(tc.clone(), &dec.ty)?;
                tc.insert_mut(dec.name.clone(), Ty::Name(dec.name.clone(), Some(Box::new(ty))));
            }

            Ok((vc, tc))
        },
        }
    }

    fn check_type(&self, tc: TypeContext, ty: &Type) -> Result<Ty, Error> {

        match ty {
        | Type::Name(name, span) => tc.get_partial(span, name),
        | Type::Arr(name, span) => {

            // Look up array element type
            let elem_ty = Box::new(tc.get_partial(span, name)?);
            Ok(Ty::Arr(elem_ty, Uuid::new_v4()))

        },
        | Type::Rec(decs, span) => {

            let mut fields = Vec::new();

            // Look up each field type
            for dec in decs {
                fields.push((dec.name.clone(), tc.get_partial(span, &dec.ty)?));
            }

            Ok(Ty::Rec(fields, Uuid::new_v4()))

        },
        }
    }
}
