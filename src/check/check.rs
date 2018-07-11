use fnv::{FnvHashSet, FnvHashMap};

use sym::Symbol;

use ast::*;
use ir;
use ty::*;
use operand::Label;
use check::context::{Binding, VarContext, TypeContext};
use translate::*;
use error::{Error, TypeError};
use span::{Span, IntoSpan};

type Typed = (Ty, ir::Tree);

fn error<T>(span: &Span, err: TypeError) -> Result<T, Error> {
    Err(Error::semantic(*span, err))
}

pub struct Checker {
    done: Vec<Unit>,
    data: Vec<ir::Static>,
    loops: Vec<Label>,
    frames: Vec<Frame>,
    vc: VarContext,
    tc: TypeContext,
}

impl Checker {

    pub fn check(ast: &Exp) -> Result<Vec<Unit>, Error> {
        let main = Frame::new(
            Label::from_fixed("main"),
            Vec::new(),
        );

        let mut checker = Checker {
            done: Vec::new(),
            data: Vec::new(),
            loops: Vec::new(),
            frames: vec![main],
            vc: VarContext::default(),
            tc: TypeContext::default(),
        };

        let (_, main_exp) = checker.check_exp(ast)?;
        let main_frame = checker.frames.pop()
            .expect("Internal error: missing frame");

        let main_unit = main_frame.wrap(main_exp);
        checker.done.push(main_unit);

        Ok(checker.done)
    }

    fn check_var(&mut self, var: &Var) -> Result<Typed, Error> {

        match var {
        | Var::Simple(name, span) => {

            let var_ty = self.vc.get_var(span, name)?;
            let exp = translate_simple_var(&self.frames, name);
            Ok((var_ty, exp))

        },
        | Var::Field(rec, field, field_span, _) => {

            let (rec_ty, rec_exp) = self.check_var(&*rec)?;

            // Must be bound to record type
            match rec_ty {
            | Ty::Rec(fields, _) => {

                // Find corresponding field
                let field = fields.iter()
                    .enumerate()
                    .find(|(i, (name, _))| field == name)
                    .map(|(i, (_, ty))| (i, self.tc.trace_full(field_span, ty)));

                // Check field type
                match field {
                | Some((index, ty)) => Ok((ty?, translate_field_var(rec_exp, index))),
                | None     => error(field_span, TypeError::UnboundField),
                }
            },
            | _ => error(&rec.into_span(), TypeError::NotRecord),
            }
        },
        | Var::Index(arr, index, _) => {

            let (index_ty, index_exp) = self.check_exp(index)?;

            // Index must be integer
            if !index_ty.is_int() {
                return error(&index.into_span(), TypeError::IndexMismatch)
            }

            let (arr_ty, arr_exp) = self.check_var(&*arr)?;

            // Get element type
            if let Ty::Arr(ele_ty, _) = arr_ty {
                Ok((
                    *ele_ty.clone(),
                    translate_index_var(arr_exp, index_exp),
                ))
            } else {
                error(&arr.into_span(), TypeError::NotArr)
            }
        },
        }
    }

    fn check_exp(&mut self, exp: &Exp) -> Result<Typed, Error> {

        match exp {
        | Exp::Nil(_)      => Ok((Ty::Nil, translate_nil())),
        | Exp::Int(n, _)   => Ok((Ty::Int, translate_int(*n))),
        | Exp::Str(s, _)   => Ok((Ty::Str, translate_str(&mut self.data, s))),
        | Exp::Var(var, _) => self.check_var(var),
        | Exp::Break(span) => {
            if self.loops.is_empty() {
                error(span, TypeError::Break)
            } else {
                Ok((Ty::Unit, translate_break(&self.loops)))
            }
        },
        | Exp::Call{name, name_span, args, span} => {

            // Get function header
            let binding = self.vc.get_fun(name_span, name)?;

            let (arg_tys, ret_ty) = match &binding {
            | Binding::Fun(arg_tys, ret_ty, _)
            | Binding::Ext(arg_tys, ret_ty, _) => (arg_tys, ret_ty),
            | _                                => panic!("Internal error: not function"),
            };

            // Check number of arguments
            if args.len() != arg_tys.len() {
                return error(name_span, TypeError::CallCountMismatch)
            }

            let mut arg_exps = Vec::new();

            // Check that each argument subtypes formal parameter type
            for (arg, ty) in args.iter().zip(arg_tys) {

                let (arg_ty, arg_exp) = self.check_exp(arg)?;

                if !arg_ty.subtypes(&ty) {
                    return error(&arg.into_span(), TypeError::CallTypeMismatch)
                }

                arg_exps.push(arg_exp);
            }

            Ok((ret_ty.clone(), translate_call(&binding, arg_exps)))
        },
        | Exp::Neg(neg, span) => {

            let (neg_ty, neg_exp) = self.check_exp(neg)?;

            // Unary negation only works on integers
            if !neg_ty.is_int() {
                return error(&exp.into_span(), TypeError::Neg)
            }

            Ok((Ty::Int, translate_neg(neg_exp)))
        },
        | Exp::Bin{lhs, op, op_span, rhs, span} => {

            let (lhs_ty, lhs_exp) = self.check_exp(lhs)?;
            let (rhs_ty, rhs_exp) = self.check_exp(rhs)?;

            // No binary operators work on unit
            if lhs_ty == Ty::Unit {
                return error(&lhs.into_span(), TypeError::BinaryUnit)
            } else if rhs_ty == Ty::Unit {
                return error(&rhs.into_span(), TypeError::BinaryUnit)
            }

            // Equality checking is valid for any L<>R, L=R where R: L
            if op.is_equality() && (lhs_ty.subtypes(&rhs_ty) || rhs_ty.subtypes(&lhs_ty)) {
                return if lhs_ty == Ty::Nil && rhs_ty == Ty::Nil {
                    error(span, TypeError::BinaryNil)
                } else {
                    Ok((Ty::Int, translate_bin(lhs_exp, *op, rhs_exp)))
                }
            }

            // Comparisons are valid for
            // - Str and Str
            // - Int and Int
            if op.is_comparison() && (lhs_ty == Ty::Int || lhs_ty == Ty::Str) && lhs_ty == rhs_ty {
                return Ok((Ty::Int, translate_bin(lhs_exp, *op, rhs_exp)))
            }

            // Arithmetic is valid for
            // - Int and Int
            if lhs_ty == Ty::Int && rhs_ty == Ty::Int {
                return Ok((Ty::Int, translate_bin(lhs_exp, *op, rhs_exp)))
            }

            error(op_span, TypeError::BinaryMismatch)
        },
        | Exp::Rec{name, name_span, fields, span} => {

            let rec_ty = self.tc.get_full(name_span, name)?;

            let field_exps = match &rec_ty {
            | Ty::Rec(field_tys, _) => {

                if fields.len() != field_tys.len() {
                    return error(span, TypeError::FieldCountMismatch)
                }

                // Make sure all record fields are fully resolved
                let field_tys = field_tys.iter()
                    .map(|(name, ty)| (name, self.tc.trace_full(span, ty)))
                    .collect::<Vec<_>>();

                let mut field_exps = Vec::new();

                // Check all field name - value pairs
                for (field, (field_name, field_ty)) in fields.iter().zip(field_tys) {

                    let (field_exp_ty, field_exp) = self.check_exp(&*field.exp)?;

                    if &field.name != field_name {
                        return error(&field.name_span, TypeError::FieldNameMismatch)
                    }

                    if !field_exp_ty.subtypes(&field_ty?) {
                        return error(&field.exp.into_span(), TypeError::FieldTypeMismatch)
                    }

                    field_exps.push(field_exp);
                }

                field_exps
            },
            | _ => return error(name_span, TypeError::NotRecord),
            };

            Ok((rec_ty, translate_rec(field_exps)))
        },
        | Exp::Seq(statements, _) => {

            // Empty sequence is just unit
            if statements.len() == 0 { return Ok((Ty::Unit, translate_nil())) }

            let mut statement_exps = Vec::new();

            // Check intermediate expressions
            for i in 0..statements.len() - 1 {
                let (_, statement_exp) = self.check_exp(&statements[i])?;
                statement_exps.push(statement_exp);
            }

            // Result is type of last exp
            let (result_ty, result_exp) = self.check_exp(&statements.last().unwrap())?;

            statement_exps.push(result_exp);

            Ok((result_ty, translate_seq(statement_exps)))

        },
        | Exp::Ass{name, exp, ..} => {

            let (lhs_ty, lhs_exp) = self.check_var(name)?;
            let (rhs_ty, rhs_exp) = self.check_exp(exp)?;

            if !rhs_ty.subtypes(&lhs_ty) {
                return error(&exp.into_span(), TypeError::VarMismatch)
            }

            Ok((Ty::Unit, translate_ass(lhs_exp, rhs_exp)))
        },
        | Exp::If{guard, then, or, span} => {

            let (guard_ty, guard_exp) = self.check_exp(guard)?;
            let (then_ty, then_exp) = self.check_exp(then)?;

            // Guard must be boolean
            if !guard_ty.is_int() {
                return error(&guard.into_span(), TypeError::GuardMismatch)
            }

            if let Some(exp) = or {

                // For if-else, both branches must return the same type
                let (or_ty, or_exp) = self.check_exp(&*exp)?;

                if !then_ty.subtypes(&or_ty) && !or_ty.subtypes(&then_ty) {
                    return error(&exp.into_span(), TypeError::BranchMismatch)
                }

                Ok((then_ty, translate_if(guard_exp, then_exp, Some(or_exp))))

            } else {

                // For if, branch must have no expression
                if then_ty != Ty::Unit {
                    return error(&then.into_span(), TypeError::UnusedBranch)
                }

                Ok((Ty::Unit, translate_if(guard_exp, then_exp, None)))
            }
        },
        | Exp::While{guard, body, ..} => {

            let (guard_ty, guard_exp) = self.check_exp(guard)?;

            // Guard must be boolean
            if !guard_ty.is_int() {
                return error(&guard.into_span(), TypeError::GuardMismatch)
            }

            // Enter loop body
            let s_label = Label::from_str("START_WHILE");
            self.loops.push(s_label);
            let (body_ty, body_exp) = self.check_exp(body)?;
            self.loops.pop().expect("Internal error: missing loop");

            // Body must be unit
            if !body_ty.is_unit() {
                return error(&body.into_span(), TypeError::UnusedWhileBody)
            }

            Ok((Ty::Unit, translate_while(s_label, guard_exp, body_exp)))
        },
        | Exp::For{name, escape, lo, hi, body, ..} => {

            let (lo_ty, lo_exp) = self.check_exp(lo)?;
            let (hi_ty, hi_exp) = self.check_exp(hi)?;

            if !lo_ty.is_int() {
                return error(&lo.into_span(), TypeError::ForBound)
            }

            if !hi_ty.is_int() {
                return error(&hi.into_span(), TypeError::ForBound)
            }

            // Enter loop body with new environment and binding
            let label = Label::from_str("START_FOR");
            let index_exp = translate_for_index(&mut self.frames, *name, *escape);

            self.vc.push();
            self.vc.insert(*name, Binding::Var(Ty::Int));
            self.loops.push(label);

            // Check body with updated VarContext
            let (body_ty, body_exp) = self.check_exp(&*body)?;

            if !body_ty.is_unit() {
                return error(&body.into_span(), TypeError::UnusedForBody)
            }

            // Pop environment
            self.vc.pop();
            self.loops.pop().expect("Internal error: missing loop");

            Ok((Ty::Unit, translate_for(label, index_exp, lo_exp, hi_exp, body_exp)))
        },
        | Exp::Let{decs, body, ..} => {

            // Enter let body with new environment and binding
            self.vc.push();
            self.tc.push();

            let mut dec_exps = Vec::new();

            for dec in decs {
                if let Some(dec_exp) = self.check_dec(&*dec)? {
                    dec_exps.push(dec_exp);
                }
            }

            let (body_ty, body_exp) = self.check_exp(&*body)?;

            self.vc.pop();
            self.tc.pop();

            Ok((body_ty, translate_let(dec_exps, body_exp)))
        },
        | Exp::Arr{name, name_span, size, init, ..} => {

            // Look up element type
            let elem = match self.tc.get_full(name_span, name)? {
            | Ty::Arr(elem, _) => *elem,
            | _                => return error(name_span, TypeError::NotArr),
            };

            let (size_ty, size_exp) = self.check_exp(&*size)?;

            // Size must be integer
            if !size_ty.is_int() {
                return error(&size.into_span(), TypeError::ArrSize)
            }

            let (init_ty, init_exp) = self.check_exp(&*init)?;

            // Initialization expression must subtype element type
            if !init_ty.subtypes(&elem) {
                return error(&init.into_span(), TypeError::ArrMismatch)
            }

            Ok((self.tc.get_full(name_span, name)?, translate_arr(size_exp, init_exp)))
        },
        }
    }

    fn check_unique(names: impl Iterator<Item = (Symbol, Span)>) -> Result<(), Error> {
        let mut unique = FnvHashSet::default();
        for (name, name_span) in names {
            if unique.contains(&name) { return error(&name_span, TypeError::DecConflict) }
            unique.insert(name);
        }
        Ok(())
    }

    fn check_dec(&mut self, dec: &Dec) -> Result<Option<ir::Tree>, Error> {
        match dec {
        | Dec::Fun(funs, _) => {

            // Make sure all top-level names are unique
            Self::check_unique(funs.iter().map(|fun| (fun.name, fun.name_span)))?;

            let mut labels = FnvHashMap::default();

            // Initialize top-level bindings
            for fun in funs {

                let label = Label::from_symbol(fun.name);
                labels.insert(fun.name, label);
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
                self.vc.insert(fun.name, Binding::Fun(args, ret, label));
            }

            // Evaluate bodies with all function headers
            for fun in funs {

                let label = labels.get(&fun.name)
                    .expect("Internal error: missing label");

                self.vc.push();
                self.frames.push(
                    translate_frame(label, &fun.args)
                );

                // Add parameter bindings to body context
                for arg in &fun.args {
                    let arg_ty = self.tc.get_full(&arg.name_span, &arg.ty)?;
                    self.vc.insert(arg.name, Binding::Var(arg_ty));
                }


                // Evaluate body with updated context
                let (body_ty, body_exp) = self.check_exp(&fun.body)?;

                self.vc.pop();
                let frame = self.frames.pop()
                    .expect("Internal error: missing frame");

                // Get return type
                let ret_ty = match &fun.rets {
                | None      => Ty::Unit,
                | Some(ret) => self.tc.get_full(&fun.rets_span.unwrap(), ret)?,
                };

                // Make sure body expression subtypes return
                if !body_ty.subtypes(&ret_ty) {
                    return error(&fun.body.into_span(), TypeError::ReturnMismatch)
                }

                self.done.push(translate_fun_dec(frame, body_exp));
            }

            Ok(None)
        },
        | Dec::Var{name, name_span, escape, ty, ty_span, init, span} => {

            // Initialization expression type
            let (init_ty, init_exp) = self.check_exp(&init)?;

            // Can't assign nil without type annotation
            if init_ty == Ty::Nil && ty.is_none() {
                return error(name_span, TypeError::UnknownNil)
            }

            // Type annotation on variable
            match ty {
            | None     => self.vc.insert(*name, Binding::Var(init_ty.clone())),
            | Some(id) => {

                // Make sure initialization matches annotation
                let name_ty = self.tc.get_full(&ty_span.unwrap(), id)?;
                if !init_ty.subtypes(&name_ty) {
                    return error(&init.into_span(), TypeError::VarMismatch)
                }

                self.vc.insert(*name, Binding::Var(name_ty));
            },
            };

            Ok(Some(translate_var_dec(&mut self.frames, *name, *escape, init_exp)))
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

            Ok(None)
        },
        }
    }

    fn check_type(&self, ty: &Type) -> Result<Ty, Error> {

        match ty {
        | Type::Name(name, span) => self.tc.get_partial(span, name),
        | Type::Arr(name, name_span, span) => {

            // Look up array element type
            let elem_ty = Box::new(self.tc.get_partial(name_span, name)?);
            Ok(Ty::Arr(elem_ty, TyID::next()))

        },
        | Type::Rec(decs, _) => {

            let mut fields = Vec::new();

            // Look up each field type
            for dec in decs {
                fields.push((dec.name, self.tc.get_partial(&dec.ty_span, &dec.ty)?));
            }

            Ok(Ty::Rec(fields, TyID::next()))

        },
        }
    }
}
