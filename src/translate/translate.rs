use ast;
use ir;

use ty::Ty;
use check::TypeContext;
use translate::FrameContext;

pub struct Translator {
    fc: Vec<FrameContext>,
    tc: Vec<TypeContext>,
}

impl Translator {

    pub fn translate(ast: &ast::Exp) -> ir::Tree {
        unimplemented!()
    }

    pub fn translate_var(&mut self, var: &ast::Var) -> ir::Tree {

        unimplemented!()
    }

    pub fn translate_exp(&mut self, ast: &ast::Exp) -> ir::Tree {

        unimplemented!()
    }

    pub fn translate_dec(&mut self, dec: &ast::Dec) {

    }

    pub fn translate_type(&mut self, ty: &Ty) {

    }
}
