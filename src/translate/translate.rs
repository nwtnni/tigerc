use ast;
use ir;
use ty::TypeContext;
use translate::frame::Frame;

pub struct Translator {
    frames: Vec<Frame>,
    tc: Vec<TypeContext>,
}

impl Translator {

    pub fn translate(ast: &ast::Exp) -> ir::Tree {

        unimplemented!()

    }


}
