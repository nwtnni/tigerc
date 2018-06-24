use ast::*;
use ir;

use ty::Ty;
use check::TypeContext;
use translate::{Access, FrameContext};

pub struct Translator {
    loops: Vec<ir::Label>,
    fc: Vec<FrameContext>,
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
        _ => unimplemented!(),
        }
    }

    pub fn translate_dec(&mut self, dec: &Dec) {

    }

    pub fn translate_type(&mut self, ty: &Ty) {

    }
}
