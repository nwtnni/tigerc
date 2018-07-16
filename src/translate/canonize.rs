use ir::*;
use operand::Temp;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Purity {
    Pure,
    Impure,
}

impl Purity {
    fn and(self, other: Purity) -> Purity {
        if (self, other) == (Purity::Pure, Purity::Pure) {
            Purity::Pure
        } else {
            Purity::Impure
        }
    }

    fn is_affected_by(self, other: Purity) -> bool {
        (self, other) == (Purity::Impure, Purity::Impure)
    }
}

pub fn canonize(ast: Stm) -> Stm {
    let (_, statements) = canonize_stm(ast);
    Stm::Seq(statements)
}

fn canonize_exp(exp: Exp) -> (Purity, Exp, Vec<Stm>) {

    match exp {
    | Exp::Const(_)
    | Exp::Name(_)
    | Exp::Temp(_) => (Purity::Pure, exp, vec![]),
    | Exp::Binop(lhs_exp, op, rhs_exp) => {

        let (lhs_purity, lhs_exp, mut lhs_stms) = canonize_exp(*lhs_exp);
        let (rhs_purity, rhs_exp, mut rhs_stms) = canonize_exp(*rhs_exp);

        // Evaluation of RHS might affect evaluation of LHS
        if lhs_purity.is_affected_by(rhs_purity) {

            let protect = Temp::from_str("CANONIZE_BINOP_LHS");

            // Move LHS result into temp before evaluating RHS
            lhs_stms.push(Stm::Move(
                lhs_exp,
                Exp::Temp(protect),
            ));

            lhs_stms.append(&mut rhs_stms);

            let canonized = Exp::Binop(
                Box::new(Exp::Temp(protect)),
                op,
                Box::new(rhs_exp),
            );

            (Purity::Impure, canonized, lhs_stms)
        }

        // Evaluation of RHS doesn't affect evaluation of LHS
        else {

            // Safe to evaluate all statements before expression
            lhs_stms.append(&mut rhs_stms);

            let canonized = Exp::Binop(
                Box::new(lhs_exp),
                op,
                Box::new(rhs_exp),
            );

            (Purity::Pure, canonized, lhs_stms)
        }
    }
    | Exp::Mem(addr_exp) => {

        // Memory access is pure if address expression is pure
        let (purity, exp, stms) = canonize_exp(*addr_exp);

        let canonized = Exp::Mem(
            Box::new(exp)
        );

        (purity, canonized, stms)

    },
    | Exp::Call(name_exp, arg_exps) => {

        let mut all_purity = Purity::Pure;
        let mut all_stms = vec![];
        let mut all_exps = vec![];

        // Impure args may affect all args before them
        for arg_exp in arg_exps.into_iter().rev() {

            let (arg_purity, arg_exp, mut arg_stms) = canonize_exp(arg_exp);

            // Arg possibly conflicts with later arg
            if arg_purity.is_affected_by(all_purity) {

                // Move arg into temp
                let arg_temp = Temp::from_str("CANONIZE_CALL_ARG");
                arg_stms.push(Stm::Move(arg_exp, Exp::Temp(arg_temp)));

                // Append later arg statements
                arg_stms.append(&mut all_stms);
                all_stms = arg_stms;
                all_exps.push(Exp::Temp(arg_temp));
                all_purity = Purity::Impure;

            } else {
                arg_stms.append(&mut all_stms);
                all_stms = arg_stms;
                all_exps.push(arg_exp);
            }
        }

        let (name_purity, name_exp, mut name_stms) = canonize_exp(*name_exp);

        if name_purity.is_affected_by(all_purity) {

            let name_temp = Temp::from_str("CANONIZE_CALL_NAME");
            let call_temp = Temp::from_str("CANONIZE_CALL");

            name_stms.push(Stm::Move(
                name_exp,
                Exp::Temp(name_temp)
            ));

            name_stms.append(&mut all_stms);

            // Move result of call into temp to prevent clobbering
            name_stms.push(Stm::Move(
                Exp::Call(
                    Box::new(Exp::Temp(name_temp)),
                    all_exps,
                ),
                Exp::Temp(call_temp),
            ));

            (Purity::Impure, Exp::Temp(call_temp), name_stms)

        } else {

            let call_temp = Temp::from_str("CANONIZE_CALL");

            name_stms.append(&mut all_stms);

            // Move result of call into temp to prevent clobbering
            name_stms.push(Stm::Move(
                Exp::Call(
                    Box::new(name_exp),
                    all_exps,
                ),
                Exp::Temp(call_temp),
            ));

            (Purity::Impure, Exp::Temp(call_temp), name_stms)

        }
    },
    | Exp::ESeq(stm, exp) => {

        let (exp_purity, exp_exp, mut exp_stms) = canonize_exp(*exp);
        let (stm_purity, mut stm_stms) = canonize_stm(*stm);
        stm_stms.append(&mut exp_stms);

        // ESeq is pure if both statement and expression are pure
        (exp_purity.and(stm_purity), exp_exp, stm_stms)

    },
    }
}

fn canonize_stm(stm: Stm) -> (Purity, Vec<Stm>) {

    match stm {
    | Stm::Label(_)
    | Stm::Comment(_) => (Purity::Pure, vec![stm]),
    | Stm::Move(src_exp, dst_exp) => {

        let (_, src_exp, mut src_stms) = canonize_exp(src_exp);
        let (_, dst_exp, mut dst_stms) = canonize_exp(dst_exp);

        src_stms.append(&mut dst_stms);

        src_stms.push(Stm::Move(
            src_exp,
            dst_exp,
        ));

        (Purity::Impure, src_stms)

    },
    | Stm::Exp(exp) => {

        let (exp_purity, _, exp_stms) = canonize_exp(exp);
        (exp_purity, exp_stms)

    },
    | Stm::Jump(addr_exp, labels) => {

        let (_, addr_exp, mut addr_stms) = canonize_exp(addr_exp);

        addr_stms.push(Stm::Jump(
            addr_exp,
            labels
        ));

        (Purity::Impure, addr_stms)

    },
    | Stm::CJump(lhs_exp, op, rhs_exp, t, f) => {

        let (_, lhs_exp, mut lhs_stms) = canonize_exp(lhs_exp);
        let (_, rhs_exp, mut rhs_stms) = canonize_exp(rhs_exp);

        lhs_stms.append(&mut rhs_stms);
        lhs_stms.push(Stm::CJump(lhs_exp, op, rhs_exp, t, f));

        (Purity::Impure, lhs_stms)

    },
    | Stm::Seq(stms) => {

        stms.into_iter()
            .map(|stm| canonize_stm(stm))
            .fold((Purity::Pure, vec![]), |(all_purity, mut all_stms), (stm_purity, mut stm_stms)| {
                all_stms.append(&mut stm_stms);
                (stm_purity.and(all_purity), all_stms)
            })

    },
    }
}
