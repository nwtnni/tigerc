use ir::*;
use operand::Label;

pub fn fold(ir: Vec<Stm>) -> Vec<Stm> {
    ir.into_iter()
        .map(|stm| fold_stm(&stm))
        .collect()
}

fn fold_exp(exp: &Exp) -> Exp {
    match exp {
    | Exp::Const(_)
    | Exp::Name(_)
    | Exp::Temp(_) => exp.clone(),
    | Exp::Binop(lhs_exp, op, rhs_exp) => fold_binop(lhs_exp, op, rhs_exp),
    | Exp::Mem(addr_exp) => {
        Exp::Mem(
            Box::new(fold_exp(addr_exp))
        )
    },
    | Exp::Call(name_exp, arg_exps) => {
        Exp::Call(
            Box::new(fold_exp(name_exp)),
            arg_exps.into_iter()
                .map(|arg_exp| fold_exp(arg_exp))
                .collect()
        )
    },
    | Exp::ESeq(stm, exp) => {
        Exp::ESeq(
            Box::new(fold_stm(stm)),
            Box::new(fold_exp(exp)),
        )
    },
    }
}

fn fold_binop(lhs_exp: &Exp, op: &Binop, rhs_exp: &Exp) -> Exp {

    let lhs_exp = fold_exp(lhs_exp);
    let rhs_exp = fold_exp(rhs_exp);

    match (lhs_exp, op, rhs_exp) {
    | (Exp::Const(0),   Binop::Add,     rhs          )
    | (Exp::Const(0),   Binop::Or,      rhs          ) => rhs,
    | (lhs,             Binop::Add,     Exp::Const(0))
    | (lhs,             Binop::Sub,     Exp::Const(0))
    | (lhs,             Binop::Or ,     Exp::Const(0))
    | (lhs,             Binop::LShift,  Exp::Const(0))
    | (lhs,             Binop::RShift,  Exp::Const(0))
    | (lhs,             Binop::ARShift, Exp::Const(0)) => lhs,
    | (Exp::Const(0),   Binop::Mul,     _            )
    | (_            ,   Binop::Mul,     Exp::Const(0))
    | (Exp::Const(0),   Binop::And,     _            )
    | (_            ,   Binop::And,     Exp::Const(0)) => Exp::Const(0),
    | (Exp::Const(lhs), op,             Exp::Const(rhs)) => {

        let result = match op {
        | Binop::Add => lhs + rhs,
        | Binop::Sub => lhs - rhs,
        | Binop::Mul => lhs * rhs,
        | Binop::Div => lhs / rhs,
        | Binop::And => lhs & rhs,
        | Binop::Or  => lhs | rhs,
        | Binop::LShift => lhs << rhs,
        | Binop::RShift => (lhs as u32 >> rhs) as i32,
        | Binop::ARShift => lhs >> rhs,
        | Binop::XOr => lhs ^ rhs,
        };

        Exp::Const(result)
    },
    | (lhs_exp, op, rhs_exp) => Exp::Binop(Box::new(lhs_exp), *op, Box::new(rhs_exp)),
    }
}

fn fold_stm(stm: &Stm) -> Stm {

    match stm {
    | Stm::Label(_)
    | Stm::Comment(_) => stm.clone(),
    | Stm::Move(src_exp, dst_exp) => {
        Stm::Move(
            fold_exp(src_exp),
            fold_exp(dst_exp),
        )
    },
    | Stm::Exp(exp) => {
        Stm::Exp(
            fold_exp(exp),
        )
    },
    | Stm::Jump(dst_exp, labels) => {
        Stm::Jump(
            fold_exp(dst_exp),
            labels.clone(),
        )
    },
    | Stm::CJump(lhs_exp, op, rhs_exp, t, f) => {
        fold_cjump(lhs_exp, op, rhs_exp, t, f)
    },
    | Stm::Seq(stms) => {
        Stm::Seq(
            stms.into_iter()
                .map(|stm| fold_stm(stm))
                .collect()
        )
    },
    }

}

fn fold_cjump(lhs_exp: &Exp, op: &Relop, rhs_exp: &Exp, t: &Label, f: &Label) -> Stm {

    let lhs_exp = fold_exp(lhs_exp);
    let rhs_exp = fold_exp(rhs_exp);

    if let (Exp::Const(lhs), Exp::Const(rhs)) = (&lhs_exp, &rhs_exp) {

        let result = match op {
        | Relop::Eq  => lhs == rhs,
        | Relop::Ne  => lhs != rhs,
        | Relop::Lt  => lhs <  rhs,
        | Relop::Gt  => lhs >  rhs,
        | Relop::Le  => lhs <= rhs,
        | Relop::Ge  => lhs >= rhs,
        | Relop::Ult => (*lhs as u32) <  (*rhs as u32),
        | Relop::Ule => (*lhs as u32) <= (*rhs as u32),
        | Relop::Ugt => (*lhs as u32) >  (*rhs as u32),
        | Relop::Uge => (*lhs as u32) >= (*rhs as u32),
        };

        let target = if result { *t } else { *f };

        return Stm::Jump(
            Exp::Name(target),
            vec![target],
        )
    }

    Stm::CJump(
        lhs_exp,
        *op,
        rhs_exp,
        *t,
        *f,
    )
}
