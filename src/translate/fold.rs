use ir::*;
use operand::Label;

pub fn fold_ast(ast: Stm) -> Stm {
    unimplemented!()
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

    if let (Exp::Const(lhs), Exp::Const(rhs)) = (&lhs_exp, &rhs_exp) {
        let result = match op {
        | Binop::Add => lhs + rhs,
        | Binop::Sub => lhs - rhs,
        | Binop::Mul => lhs * rhs,
        | Binop::Div => lhs / rhs,
        | Binop::And => lhs & rhs,
        | Binop::Or  => lhs | rhs,
        | Binop::LShift => lhs << rhs,
        | Binop::RShift => (*lhs as u32 >> rhs) as i32,
        | Binop::ARShift => lhs >> rhs,
        | Binop::XOr => lhs ^ rhs,
        };

        return Exp::Const(result)
    }

    Exp::Binop(
        Box::new(lhs_exp),
        *op,
        Box::new(rhs_exp),
    )
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
    unimplemented!()
}