use sym::store;

use ir::*;
use translate::frame::Frame;

enum Node {
    Ex(Exp),
    Nx(Stm),
    Cx(Box<Fn(Label, Label) -> Stm>),
}

impl Node {
    fn to_ex(self) -> Exp {
        match self {
        | Node::Ex(exp) => exp,
        | Node::Nx(stm) => {
            Exp::ESeq(
                Box::new(stm),
                Box::new(Exp::Const(0)),
            )
        },
        | Node::Cx(gen_stm) => {
            let r = Temp::with_name(store("COND_EXP"));
            let t = Label::with_name(store("TRUE_BRANCH"));
            let f = Label::with_name(store("FALSE_BRANCH"));
            Exp::ESeq(
                Box::new(Stm::Seq(vec![
                    Stm::Move(Exp::Const(1), Exp::Temp(r)),
                    gen_stm(t, f),
                    Stm::Label(f),
                    Stm::Move(Exp::Const(0), Exp::Temp(r)),
                    Stm::Label(t),
                ])),
                Box::new(Exp::Temp(r)),
            )
        },
        }
    }

    fn to_nx(self) -> Stm {
        match self {
        | Node::Nx(stm) => stm,
        | Node::Ex(exp) => Stm::Exp(exp),
        | Node::Cx(gen_stm) => {
            let t = Label::with_name(store("TRUE_BRANCH"));
            let f = Label::with_name(store("FALSE_BRANCH"));
            gen_stm(t, f)
        },
        }
    }

    fn to_cx(self) -> Box<Fn(Label, Label) -> Stm> {
        match self {
        | Node::Nx(_) => panic!("Internal compiler error: converting statement to conditional"),
        | Node::Cx(gen_stm) => gen_stm,
        | Node::Ex(exp) => {
            Box::new(move |t, f| Stm::CJump(
                Exp::Const(0),     
                Relop::Eq,
                exp.clone(),
                t,
                f,
            ))
        },
        }
    }
}

pub struct Translator {
    frames: Vec<Frame>,
}

impl Translator {


}
