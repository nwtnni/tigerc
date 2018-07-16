use ir::*;
use operand::Temp;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Purity {
    Pure,
    Impure,
}

impl Purity {
    fn and(self, other: Purity) -> bool {
        match (self, other) {
        | (Purity::Pure, Purity::Impure) => true,
        | _                              => false,
        }
    }
}

pub fn canonize_ast(ast: Stm) {
    unimplemented!()
}

fn canonize_exp(exp: Exp) -> (Purity, Exp, Vec<Stm>) {

    match exp {
    | Exp::Const(_)
    | Exp::Name(_)
    | Exp::Temp(_) => (Purity::Pure, exp, vec![]),
    | Exp::Binop(lhs_exp, op, rhs_exp) => {

        let (_,          lhs_exp, mut lhs_stms) = canonize_exp(*lhs_exp);
        let (rhs_purity, rhs_exp, mut rhs_stms) = canonize_exp(*rhs_exp);

        // Evaluation of RHS doesn't affect evaluation of LHS
        if let Purity::Pure = rhs_purity {

            // Safe to evaluate all statements before expression
            lhs_stms.append(&mut rhs_stms);

            let canonized = Exp::Binop(
                Box::new(lhs_exp),
                op,
                Box::new(rhs_exp),
            );
            
            (Purity::Pure, canonized, lhs_stms)
        } 
        
        // Evaluation of RHS might affect evaluation of LHS
        else {
            
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
    }
    | Exp::Mem(addr_exp) => {
        
        let (purity, exp, stms) = canonize_exp(*addr_exp);
        
        let canonized = Exp::Mem(
            Box::new(exp)
        );

        (purity, canonized, stms)

    },
    | Exp::Call(name_exp, arg_exps) => {

        unimplemented!() 

    },
    | Exp::ESeq(stm, exp) => {
        
        unimplemented!() 

    },
    }
}

fn canonize_stm(stm: Stm) -> (Purity, Vec<Stm>) {

    unimplemented!()

}
