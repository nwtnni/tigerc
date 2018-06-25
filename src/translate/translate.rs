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

            let label = *self.loops.last()
                .expect("Internal error: break without enclosing loop");

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
        _ => unimplemented!(),
        }
    }

    pub fn translate_dec(&mut self, dec: &Dec) {

    }

    pub fn translate_type(&mut self, ty: &Ty) {

    }
}
