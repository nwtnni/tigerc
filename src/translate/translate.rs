use ast::*;
use ir;

use ty::Ty;
use check::TypeContext;
use translate::{Access, Frame, FnContext};

pub struct Translator {
    loops: Vec<ir::Label>,
    frames: Vec<Frame>,
    fc: Vec<FnContext>,
    tc: Vec<TypeContext>,
}

impl Translator {

    pub fn translate(ast: &Exp) -> ir::Tree {
        unimplemented!()
    }

    pub fn translate_var(&mut self, var: &Var) -> ir::Tree {

        unimplemented!()
    }

    pub fn translate_exp(&mut self, ast: &Exp) -> ir::Tree {
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
        | Exp::Var(var, _) => self.translate_var(var),
        | Exp::Int(n, _) => ir::Exp::Const(*n).into(),
        | Exp::Str(s, _) => {

            // TODO: figure out how to represent string literals
            unimplemented!()

        },
        | Exp::Call{name, args, ..} => {

            // Find label from context
            let label = self.fc.iter().rev()
                .find(|context| context.contains(name))
                .expect("Internal error: function not found")
                .get(name)
                .unwrap();

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
        _ => unimplemented!(),
        }
    }

    pub fn translate_dec(&mut self, dec: &Dec) {

    }

    pub fn translate_type(&mut self, ty: &Ty) {

    }
}
