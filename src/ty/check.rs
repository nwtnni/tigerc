use codespan::ByteSpan;
use fnv::FnvHashSet;

use ast::*;
use error::{Error, TypeError};
use span::IntoSpan;
use ty::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Typed {
    ty: Ty,
    mutable: bool,
    _exp: (),
}

fn ok(ty: Ty) -> Result<Typed, Error> {
    Ok(Typed { ty, mutable: true, _exp: () })
}

fn error<T>(span: &ByteSpan, err: TypeError) -> Result<T, Error> {
    Err(Error::semantic(*span, err))
}

pub struct Checker {
    loops: Vec<()>,
    vc: VarContext,
    tc: TypeContext,
}

impl Checker {

    pub fn check(ast: &Exp) -> Result<(), Error> {
        let mut checker = Checker {
            loops: Vec::new(),
            vc: VarContext::default(),
            tc: TypeContext::default(),
        };

        let _ = checker.check_exp(ast)?;
        Ok(())
    }

    fn check_var(&mut self, var: &Var) -> Result<Typed, Error> {

        macro_rules! is_int {
            ($exp:expr) => { self.check_exp($exp)?.ty == Ty::Int }
        }

        match var {
        | Var::Simple(name, span) => {
            let (ty, mutable) = self.vc.get_var(span, name)?;
            Ok(Typed { ty, mutable, _exp: () })
        },
        | Var::Field(rec, field, field_span, _) => {

            // Must be bound to record type
            match self.check_var(&*rec)?.ty {
            | Ty::Rec(fields, _) => {

                // Find corresponding field
                let ty = fields.iter()
                    .find(|(name, _)| field == name)
                    .map(|(_, ty)| self.tc.trace_full(field_span, ty));

                // Check field type
                match ty {
                | Some(ty) => ok(ty?.clone()),
                | None     => error(field_span, TypeError::UnboundField),
                }
            },
            | _ => error(&rec.into_span(), TypeError::NotRecord),
            }
        },
        | Var::Index(arr, index, _) => {

            // Index must be integer
            if !is_int!(&*index) {
                return error(&index.into_span(), TypeError::IndexMismatch)
            }

            // Get element type
            match self.check_var(&*arr)?.ty {
            | Ty::Arr(elem, _) => ok(*elem.clone()),
            | _                => error(&arr.into_span(), TypeError::NotArr),
            }
        },
        }
    }

    fn check_exp(&mut self, exp: &Exp) -> Result<Typed, Error> {

        macro_rules! is_int {
            ($exp:expr) => { self.check_exp($exp)?.ty == Ty::Int }
        }

        macro_rules! is_unit {
            ($exp:expr) => { self.check_exp($exp)?.ty == Ty::Unit }
        }

        match exp {
        | Exp::Nil(_)      => ok(Ty::Nil),
        | Exp::Int(_, _)   => ok(Ty::Int),
        | Exp::Str(_, _)   => ok(Ty::Str),
        | Exp::Var(var, _) => self.check_var(var),
        | Exp::Break(span) => if self.loops.is_empty() { return error(span, TypeError::Break) } else { ok(Ty::Unit) },
        | Exp::Call{name, name_span, args, span} => {

            // Get function header
            let (args_ty, ret_ty) = self.vc.get_fun(span, name)?;

            // Check number of arguments
            if args.len() != args_ty.len() {
                return error(name_span, TypeError::CallCountMismatch)
            }

            // Check that each argument subtypes formal parameter type
            for (arg, ty) in args.iter().zip(args_ty) {
                if !self.check_exp(arg)?.ty.subtypes(&ty) { return error(&arg.into_span(), TypeError::CallTypeMismatch) }
            }

            ok(ret_ty.clone())
        },
        | Exp::Neg(exp, span) => {

            // Unary negation only works on integers
            if !is_int!(&*exp) { return error(&exp.into_span(), TypeError::Neg) }

            ok(Ty::Int)

        },
        | Exp::Bin{lhs, op, op_span, rhs, span} => {

            let lt = self.check_exp(lhs)?.ty;
            let rt = self.check_exp(rhs)?.ty;

            // No binary operators work on unit
            if lt == Ty::Unit {
                return error(&lhs.into_span(), TypeError::BinaryUnit)
            } else if rt == Ty::Unit {
                return error(&rhs.into_span(), TypeError::BinaryUnit)
            }

            // Equality checking is valid for any L<>R, L=R where R: L
            if op.is_equality() && (lt.subtypes(&rt) || rt.subtypes(&lt)) {
                return if lt == Ty::Nil && rt == Ty::Nil {
                    error(span, TypeError::BinaryNil)
                } else {
                    ok(Ty::Int)
                }
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

            error(op_span, TypeError::BinaryMismatch)
        },
        | Exp::Rec{name, name_span, fields, span} => {

            match self.tc.get_full(name_span, name)? {
            | Ty::Rec(fields_ty, _) => {

                if fields.len() != fields_ty.len() {
                    return error(span, TypeError::FieldCountMismatch)
                }

                // Make sure all record fields are fully resolved
                let fields_ty = fields_ty.iter()
                    .map(|(name, ty)| (name, self.tc.trace_full(span, ty)))
                    .collect::<Vec<_>>();

                // Check all field name - value pairs
                for (field, (field_name, field_ty)) in fields.iter().zip(fields_ty) {

                    let exp_ty = self.check_exp(&*field.exp)?.ty;

                    if &field.name != field_name {
                        return error(&field.name_span, TypeError::FieldNameMismatch)
                    }

                    if !exp_ty.subtypes(&field_ty?) {
                        return error(&field.exp.into_span(), TypeError::FieldTypeMismatch)
                    }
                }

                ok(self.tc.get_full(name_span, name)?)
            },
            | _ => error(name_span, TypeError::NotRecord),
            }
        },
        | Exp::Seq(exps, _) => {

            // Empty sequence is just unit
            if exps.len() == 0 { return ok(Ty::Unit) }

            // Check intermediate expressions
            for i in 0..exps.len() - 1 { self.check_exp(&exps[i])?; }

            // Result is type of last exp
            self.check_exp(&exps.last().unwrap())
        },
        | Exp::Ass{name, exp, ..} => {

            let var = self.check_var(name)?;

            if !var.mutable {
                return error(&name.into_span(), TypeError::AssignImmutable)
            }

            if !self.check_exp(exp)?.ty.subtypes(&var.ty) {
                return error(&exp.into_span(), TypeError::VarMismatch)
            }

            ok(Ty::Unit)
        },
        | Exp::If{guard, then, or, span} => {

            // Guard must be boolean
            if !is_int!(&*guard) {
                return error(&guard.into_span(), TypeError::GuardMismatch)
            }

            // Check type of if branch
            let then_ty = self.check_exp(&*then)?.ty;

            if let Some(exp) = or {

                // For if-else, both branches must return the same type
                let or_ty = self.check_exp(&*exp)?.ty;
                if !then_ty.subtypes(&or_ty) && !or_ty.subtypes(&then_ty) {
                    return error(&exp.into_span(), TypeError::BranchMismatch)
                }

                ok(then_ty.clone())

            } else {

                // For if, branch must have no expression
                if then_ty != Ty::Unit {
                    return error(&then.into_span(), TypeError::UnusedBranch)
                }

                ok(Ty::Unit)
            }
        },
        | Exp::While{guard, body, ..} => {

            // Guard must be boolean
            if !is_int!(&*guard) {
                return error(&guard.into_span(), TypeError::GuardMismatch)
            }

            // Enter loop body
            self.loops.push(());

            // Body must be unit
            if !is_unit!(&*body) {
                return error(&body.into_span(), TypeError::UnusedWhileBody)
            }

            ok(Ty::Unit)
        },
        | Exp::For{name, lo, hi, body, ..} => {

            if !is_int!(&*lo) {
                return error(&lo.into_span(), TypeError::ForBound)
            }

            if !is_int!(&*hi) {
                return error(&hi.into_span(), TypeError::ForBound)
            }

            // Enter loop body with new environment and binding
            self.vc.push();
            self.vc.insert(*name, Binding::Var(Ty::Int, false));
            self.loops.push(());

            // Check body with updated VarContext
            if self.check_exp(&*body)?.ty != Ty::Unit {
                return error(&body.into_span(), TypeError::UnusedForBody)
            }

            // Pop environment
            self.vc.pop();
            ok(Ty::Unit)
        },
        | Exp::Let{decs, body, ..} => {

            // Enter let body with new environment and binding
            self.vc.push();  
            self.tc.push();
            for dec in decs { self.check_dec(&*dec)?; }
            let body = self.check_exp(&*body);
            self.vc.pop();
            self.tc.pop();

            body
        },
        | Exp::Arr{name, name_span, size, init, ..} => {

            // Look up element type
            let elem = match self.tc.get_full(name_span, name)? {
            | Ty::Arr(elem, _) => *elem,
            | _                => return error(name_span, TypeError::NotArr),
            };

            // Size must be integer
            if !is_int!(&*size) {
                return error(&size.into_span(), TypeError::ForBound)
            }

            // Initialization expression must subtype element type
            if !self.check_exp(&*init)?.ty.subtypes(&elem) {
                return error(&init.into_span(), TypeError::ArrMismatch)
            }

            ok(self.tc.get_full(name_span, name)?)
        },
        }
    }

    fn check_unique(names: impl Iterator<Item = (Symbol, ByteSpan)>) -> Result<(), Error> {
        let mut unique = FnvHashSet::default();
        for (name, name_span) in names {
            if unique.contains(&name) { return error(&name_span, TypeError::DecConflict) }
            unique.insert(name);
        }
        Ok(())
    }

    fn check_dec(&mut self, dec: &Dec) -> Result<(), Error> {
        match dec {
        | Dec::Fun(funs, span) => {

            // Make sure all top-level names are unique
            Self::check_unique(funs.iter().map(|fun| (fun.name, fun.name_span)))?;

            // Initialize top-level bindings
            for fun in funs {

                let mut args = Vec::new();

                // Get formal parameter types
                for arg in &fun.args {
                    args.push(self.tc.get_full(&arg.name_span, &arg.ty)?);
                }

                // Get return type
                let ret = match &fun.rets {
                | None => Ty::Unit,
                | Some(name) => self.tc.get_full(&fun.rets_span.unwrap(), name)?,
                };

                // Update environment with function header
                self.vc.insert(fun.name, Binding::Fun(args, ret));
            }

            // Evaluate bodies with all function headers
            for fun in funs {

                self.vc.push();

                // Add parameter bindings to body context
                for arg in &fun.args {
                    let arg_ty = self.tc.get_full(&arg.name_span, &arg.ty)?;
                    self.vc.insert(arg.name, Binding::Var(arg_ty, true));
                }

                // Evaluate body with updated context
                let body_ty = self.check_exp(&fun.body)?.ty;

                self.vc.pop();

                // Get return type
                let ret_ty = match &fun.rets {
                | None      => Ty::Unit,
                | Some(ret) => self.tc.get_full(&fun.rets_span.unwrap(), ret)?,
                };

                // Make sure body expression subtypes return
                if !body_ty.subtypes(&ret_ty) {
                    return error(&fun.span, TypeError::ReturnMismatch)
                }
            }

            Ok(())
        },
        | Dec::Var{name, name_span, ty, ty_span, init, span, ..} => {

            // Initialization expression type
            let init_ty = self.check_exp(&init)?.ty;

            // Can't assign nil without type annotation
            if init_ty == Ty::Nil && ty.is_none() {
                return error(name_span, TypeError::UnknownNil)
            }

            // Type annotation on variable
            match ty {
            | None     => self.vc.insert(*name, Binding::Var(init_ty.clone(), true)),
            | Some(id) => {

                // Make sure initialization matches annotation
                let name_ty = self.tc.get_full(&ty_span.unwrap(), id)?;
                if !init_ty.subtypes(&name_ty) {
                    return error(&init.into_span(), TypeError::VarMismatch)
                }

                self.vc.insert(*name, Binding::Var(name_ty, true));
            },
            };

            Ok(())
        },
        | Dec::Type(decs, span) => {

            // Make sure all top-level names are unique
            Self::check_unique(decs.iter().map(|dec| (dec.name, dec.name_span)))?;

            // Initialize top-level declarations
            for dec in decs {
                self.tc.insert(dec.name, Ty::Name(dec.name, None));
            }

            // Fill in type bodies
            for dec in decs {
                let ty = self.check_type(&dec.ty)?;
                self.tc.insert(dec.name, Ty::Name(dec.name, Some(Box::new(ty))));
            }

            Ok(())
        },
        }
    }

    fn check_type(&self, ty: &Type) -> Result<Ty, Error> {

        match ty {
        | Type::Name(name, span) => self.tc.get_partial(span, name),
        | Type::Arr(name, name_span, span) => {

            // Look up array element type
            let elem_ty = Box::new(self.tc.get_partial(name_span, name)?);
            Ok(Ty::Arr(elem_ty, Uuid::new_v4()))

        },
        | Type::Rec(decs, _) => {

            let mut fields = Vec::new();

            // Look up each field type
            for dec in decs {
                fields.push((dec.name, self.tc.get_partial(&dec.ty_span, &dec.ty)?));
            }

            Ok(Ty::Rec(fields, Uuid::new_v4()))

        },
        }
    }
}
