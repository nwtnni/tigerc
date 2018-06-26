use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};
use sym::{store, Symbol};

use ast::*;
use ir;

use check::TypeContext;
use config::WORD_SIZE;
use operand::Reg;
use translate::{Frame, FnContext};
use ty::Ty;

pub struct Translator {
    data: Vec<ir::Static>,
    functions: Vec<Vec<ir::Stm>>,
    loops: Vec<ir::Label>,
    frames: Vec<Frame>,
    fc: FnContext,
    tc: TypeContext,
}

impl Translator {

    pub fn translate(ast: &Exp) -> ir::Tree {
        unimplemented!()
    }

    fn translate_var(&mut self, var: &Var) -> (ir::Tree, Ty) {
        match var {
        | Var::Simple(name, span) => {

            // Start off at current frame's base pointer
            let rbp = ir::Exp::Temp(ir::Temp::Reg(Reg::RBP));
            let link = store("STATIC_LINK");

            // Retrieve variable type
            let var_ty = self.tc.get_full(span, name)
                .expect("Internal error: unbounb variable");

            // Follow static links
            let var_exp = self.frames.iter().fold_while(rbp, |acc, frame| {
                if frame.contains(*name) {
                    Done(frame.get(*name, acc))
                } else {
                    Continue(frame.get(link, acc))
                }
            }).into_inner();

            (var_exp.into(), var_ty)
        },
        | Var::Field(record, field, _, _) => {

            // Translate record l-value
            let (record_exp, record_type) = self.translate_var(&**record);

            // Find field-type associations
            let fields = match record_type {
            | Ty::Rec(fields, _) => fields,
            | _                  => panic!("Internal error: not a record")
            };

            // Calculate index and type of resulting expression
            let (index, field_ty) = fields.iter()
                .enumerate()
                .find(|(_, (name, _))| field == name)
                .map(|(index, (_, ty))| (index as i32, ty))
                .expect("Internal error: missing field");

            // Calculate memory address offset from record pointer
            let address_exp = ir::Exp::Mem(
                Box::new(
                    ir::Exp::Binop(
                        Box::new(record_exp.into()),
                        ir::Binop::Add,
                        Box::new(ir::Exp::Const(index * WORD_SIZE)),
                    )
                )
            );

            (address_exp.into(), field_ty.clone())
        },
        | Var::Index(array, index, _) => {

            // Translate array l-value
            let (array_exp, array_ty) = self.translate_var(&**array);

            // Find array element type
            let element_ty = match array_ty {
            | Ty::Arr(ty, _) => ty,
            | _              => panic!("Internal error: not an array"),
            };

            // Translate index
            let index_exp = self.translate_exp(&**index);

            // Multiply offset by word size
            let offset_exp = ir::Exp::Binop(
                Box::new(index_exp.into()),
                ir::Binop::Mul,
                Box::new(ir::Exp::Const(WORD_SIZE)),
            );

            // Calculate memory address offset from array pointer
            let address_exp = ir::Exp::Mem(
                Box::new(
                    ir::Exp::Binop(
                        Box::new(array_exp.into()),
                        ir::Binop::Add,
                        Box::new(offset_exp),
                    )
                )
            );

            (address_exp.into(), *element_ty.clone())
        },
        }
    }

    fn translate_exp(&mut self, ast: &Exp) -> ir::Tree {
        match ast {
        | Exp::Break(_) => {

            // Find latest loop exit label on stack
            let label = *self.loops.last()
                .expect("Internal error: break without enclosing loop");

            // Jump to exit label
            ir::Stm::Jump(
                ir::Exp::Name(label),
                vec![label],
            ).into()

        },
        | Exp::Nil(_) => ir::Exp::Const(0).into(),
        | Exp::Var(var, _) => self.translate_var(var).0,
        | Exp::Int(n, _) => ir::Exp::Const(*n).into(),
        | Exp::Str(s, _) => {
            let data = ir::Static::new(s.to_string());
            let label = data.label();
            self.data.push(data);
            ir::Exp::Name(label).into()
        },
        | Exp::Call{name, args, ..} => {

            // Find label from context
            let label = self.fc.get(name);

            // Translate args sequentially
            let exps: Vec<ir::Exp> = args.iter()
                .map(|arg| self.translate_exp(arg))
                .map(|arg| arg.into())
                .collect();

            // Call function
            ir::Exp::Call(
                Box::new(ir::Exp::Name(label)),
                exps,
            ).into()
        },
        | Exp::Neg(exp, _) => {

            // Subtract sub-expression from 0
            ir::Exp::Binop(
                Box::new(ir::Exp::Const(0)),
                ir::Binop::Sub,
                Box::new(self.translate_exp(exp).into()),
            ).into()

        },
        | Exp::Bin{lhs, op, rhs, ..} => {

            let lexp = self.translate_exp(lhs).into();
            let rexp = self.translate_exp(rhs).into();

            // Straightforward arithmetic operation
            if let Some(binop) = Self::translate_binop(op) {
                ir::Exp::Binop(
                    Box::new(lexp), binop, Box::new(rexp)
                ).into()
            }

            // Conditional operation
            else if let Some(relop) = Self::translate_relop(op) {
                ir::Tree::Cx(
                    Box::new(move |t, f| {
                        ir::Stm::CJump(lexp.clone(), relop, rexp.clone(), t, f)
                    })
                )
            }

            // All operations must be covered
            else {
                panic!("Internal error: non-exhaustive binop check");
            }
        },
        | Exp::Rec{name, fields, ..} => {

            // TODO: do field lookup
            unimplemented!()

        },
        | Exp::Seq(exps, _) => {

            // Unit is a no-op
            if exps.is_empty() {
                return ir::Exp::Const(0).into()
            }

            let (last, rest) = exps.split_last().unwrap();

            // Translate last exp into an ir::Exp
            let last_exp = self.translate_exp(last).into();

            // Translate rest of exps into ir::Stm
            let rest_stm = rest.iter()
                .map(|stm| self.translate_exp(stm))
                .map(|stm| stm.into())
                .collect();

            ir::Exp::ESeq(
                Box::new(ir::Stm::Seq(rest_stm)),
                Box::new(last_exp),
            ).into()
        },
        | Exp::Ass{name, exp, ..} => {

            // TODO: handle static links and frames
            unimplemented!()

        },
        | Exp::If{guard, then, or, ..} => {


            if let Some(or_exp) = or {

                let t_label = ir::Label::with_name("TRUE_BRANCH");
                let f_label = ir::Label::with_name("FALSE_BRANCH");
                let e_label = ir::Label::with_name("EXIT_IF_ELSE");
                let result = ir::Temp::with_name("IF_ELSE_RESULT");

                ir::Exp::ESeq(
                    Box::new(ir::Stm::Seq(vec![

                        // Evaluate guard expression and jump to correct branch
                        ir::Stm::CJump(
                            self.translate_exp(guard).into(),
                            ir::Relop::Eq,
                            ir::Exp::Const(0),
                            f_label,
                            t_label,
                        ),

                        // Move result of true branch
                        ir::Stm::Label(t_label),
                        ir::Stm::Move(
                            self.translate_exp(then).into(),
                            ir::Exp::Temp(result),
                        ),
                        ir::Stm::Jump(
                            ir::Exp::Name(e_label),
                            vec![e_label],
                        ),

                        // Move result of false branch
                        ir::Stm::Label(f_label),
                        ir::Stm::Move(
                            self.translate_exp(or_exp).into(),
                            ir::Exp::Temp(result),
                        ),
                        ir::Stm::Jump(
                            ir::Exp::Name(e_label),
                            vec![e_label],
                        ),

                        // Exit branch
                        ir::Stm::Label(e_label),
                    ])),
                    Box::new(ir::Exp::Temp(result)),
                ).into()

            } else {

                let t_label = ir::Label::with_name("TRUE_BRANCH");
                let e_label = ir::Label::with_name("EXIT_IF");

                ir::Stm::Seq(vec![

                    // Evaluate guard expression and jumpt to exit if false
                    ir::Stm::CJump(
                        self.translate_exp(guard).into(),
                        ir::Relop::Eq,
                        ir::Exp::Const(0),
                        e_label,
                        t_label,
                    ),

                    // Execute branch
                    ir::Stm::Label(t_label),
                    self.translate_exp(then).into(),
                    ir::Stm::Jump(
                        ir::Exp::Name(e_label),
                        vec![e_label],
                    ),

                    // Skip branch
                    ir::Stm::Label(e_label),
                ]).into()

            }
        },
        | Exp::While{guard, body, ..} => {

            let s_label = ir::Label::with_name("START_WHILE");
            let t_label = ir::Label::with_name("TRUE_BRANCH");
            let e_label = ir::Label::with_name("EXIT_WHILE");

            let guard_exp = self.translate_exp(guard).into();

            // Push exit label of enclosing loop onto context
            self.loops.push(e_label);
            let body_stm = self.translate_exp(body).into();
            self.loops.pop().expect("Internal error: loop mismatch");

            ir::Stm::Seq(vec![

                // While loop header
                ir::Stm::Label(s_label),

                // Evaluate guard expression and jump to exit if false
                ir::Stm::CJump(
                    guard_exp,
                    ir::Relop::Eq,
                    ir::Exp::Const(0),
                    e_label,
                    t_label,
                ),

                // Execute loop body and repeat
                ir::Stm::Label(t_label),
                body_stm,
                ir::Stm::Jump(
                    ir::Exp::Name(s_label),
                    vec![s_label],
                ),

                // Exit loop
                ir::Stm::Label(e_label),

            ]).into()
        },
        _ => unimplemented!(),
        }
    }

    fn translate_binop(op: &Binop) -> Option<ir::Binop> {
        match op {
        | Binop::Add  => Some(ir::Binop::Add),
        | Binop::Sub  => Some(ir::Binop::Sub),
        | Binop::Mul  => Some(ir::Binop::Mul),
        | Binop::Div  => Some(ir::Binop::Div),
        | Binop::LAnd => Some(ir::Binop::And),
        | Binop::LOr  => Some(ir::Binop::Or),
        _ => None,
        }
    }

    fn translate_relop(op: &Binop) -> Option<ir::Relop> {
        match op {
        | Binop::Eq  => Some(ir::Relop::Eq),
        | Binop::Neq => Some(ir::Relop::Ne),
        | Binop::Lt  => Some(ir::Relop::Lt),
        | Binop::Le  => Some(ir::Relop::Le),
        | Binop::Gt  => Some(ir::Relop::Gt),
        | Binop::Ge  => Some(ir::Relop::Ge),
        _ => None,
        }
    }

    fn translate_dec(&mut self, dec: &Dec) {

    }

    fn translate_type(&mut self, ty: &Ty) {

    }
}
