use ir::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Commute {
    Yes,
    No,
}

impl Commute {
    fn and(self, other: Commute) -> Commute {
        match (self, other) {
        | (Commute::Yes, Commute::Yes) => Commute::Yes,
        | _                            => Commute::No,
        }
    }
}

pub fn canonize_ast(exp: Stm) -> Stm {
    unimplemented!()
}

fn canonize_exp(exp: Exp) -> (Commute, Exp) {

    match exp {
    | Exp::Const(_)
    | Exp::Name(_)
    | Exp::Temp(_) => (Commute::Yes, exp),
    | Exp::Binop(left, op, right) => {

        let (left_commutes, left) = canonize_exp(*left);
        let (right_commutes, right) = canonize_exp(*right);
        let both_commute = left_commutes.and(right_commutes);

       unimplemented!()
    }

    | _ => unimplemented!(),

    }
}
