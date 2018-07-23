use asm;
use ir;
use ir::*;
use operand::*;

pub fn tile(ir: &[Stm]) -> Vec<asm::Asm<Temp>> {

    unimplemented!()
}

pub enum Value {
    Temp(Temp),
    Mem(Mem<Temp>),
    Imm(Imm),
    Label(Label),
}

struct Tiler {
    asm: Vec<asm::Asm<Temp>>,
}

impl Tiler {

    fn into_temp(&mut self, value: Value) -> Temp {
        match value {
        | Value::Temp(temp) => temp,
        | Value::Mem(mem) => {
            let temp = Temp::from_str("TILE_MEM");
            self.asm.push(asm::Asm::Mov(asm::Binary::MR(mem, temp)));
            temp
        }
        | Value::Imm(imm) => {
            let temp = Temp::from_str("TILE_IMM");
            self.asm.push(asm::Asm::Mov(asm::Binary::IR(imm, temp)));
            temp
        }
        | _ => unimplemented!(),
        }
    }

    fn tile_exp(&mut self, exp: &Exp) -> Value {

        use ir::Exp::{Binop, Const};

        match exp {
        | Exp::Const(n) => Value::Imm(Imm(*n)),
        | Exp::Name(l)  => Value::Label(*l),
        | Exp::Temp(t)  => Value::Temp(*t),
        | Exp::ESeq(_, _) => panic!("Internal error: non-canonical IR"),

        // BRSO memory addressing
        | Exp::Mem(box Binop(box Binop(box b, ir::Binop::Add, box Binop(box r, ir::Binop::Mul, box Const(s))), ir::Binop::Add, box Const(o)))
        | Exp::Mem(box Binop(box Binop(box b, ir::Binop::Add, box Binop(box Const(s), ir::Binop::Mul, box r)), ir::Binop::Add, box Const(o)))
        | Exp::Mem(box Binop(box Binop(box Binop(box r, ir::Binop::Mul, box Const(s)), ir::Binop::Add, box b), ir::Binop::Add, box Const(o)))
        | Exp::Mem(box Binop(box Binop(box Binop(box Const(s), ir::Binop::Mul, box r), ir::Binop::Add, box b), ir::Binop::Add, box Const(o)))
        | Exp::Mem(box Binop(box Const(o), ir::Binop::Add, box Binop(box b, ir::Binop::Add, box Binop(box r, ir::Binop::Mul, box Const(s)))))
        | Exp::Mem(box Binop(box Const(o), ir::Binop::Add, box Binop(box b, ir::Binop::Add, box Binop(box Const(s), ir::Binop::Mul, box r))))
        | Exp::Mem(box Binop(box Const(o), ir::Binop::Add, box Binop(box Binop(box r, ir::Binop::Mul, box Const(s)), ir::Binop::Add, box b)))
        | Exp::Mem(box Binop(box Const(o), ir::Binop::Add, box Binop(box Binop(box Const(s), ir::Binop::Mul, box r), ir::Binop::Add, box b))) => {
            let b = self.tile_exp(b);
            let r = self.tile_exp(r);
            Value::Mem(Mem::BRSO(
                self.into_temp(b),
                self.into_temp(r),
                Scale::try_from(*s),
                *o,
            ))
        },
        | Exp::Mem(box Binop(box Binop(box b, ir::Binop::Add, box Binop(box r, ir::Binop::Mul, box Const(s))), ir::Binop::Sub, box Const(o)))
        | Exp::Mem(box Binop(box Binop(box b, ir::Binop::Add, box Binop(box Const(s), ir::Binop::Mul, box r)), ir::Binop::Sub, box Const(o)))
        | Exp::Mem(box Binop(box Binop(box Binop(box r, ir::Binop::Mul, box Const(s)), ir::Binop::Add, box b), ir::Binop::Sub, box Const(o)))
        | Exp::Mem(box Binop(box Binop(box Binop(box Const(s), ir::Binop::Mul, box r), ir::Binop::Add, box b), ir::Binop::Sub, box Const(o))) => {
            let b = self.tile_exp(b);
            let r = self.tile_exp(r);
            Value::Mem(Mem::BRSO(
                self.into_temp(b),
                self.into_temp(r),
                Scale::try_from(*s),
                -*o,
            ))
        },

        // RSO memory addressing
        | Exp::Mem(box Binop(box Binop(box r, ir::Binop::Mul, box Const(s)), ir::Binop::Add, box Const(o)))
        | Exp::Mem(box Binop(box Binop(box Const(s), ir::Binop::Mul, box r), ir::Binop::Add, box Const(o)))
        | Exp::Mem(box Binop(box Const(o), ir::Binop::Add, box Binop(box r, ir::Binop::Mul, box Const(s))))
        | Exp::Mem(box Binop(box Const(o), ir::Binop::Add, box Binop(box Const(s), ir::Binop::Mul, box r))) => {
            let r = self.tile_exp(r);
            Value::Mem(Mem::RSO(
                self.into_temp(r),
                Scale::try_from(*s),
                *o
            ))
        }
        | Exp::Mem(box Binop(box Binop(box r, ir::Binop::Mul, box Const(s)), ir::Binop::Sub, box Const(o)))
        | Exp::Mem(box Binop(box Binop(box Const(s), ir::Binop::Mul, box r), ir::Binop::Sub, box Const(o))) => {
            let r = self.tile_exp(r);
            Value::Mem(Mem::RSO(
                self.into_temp(r),
                Scale::try_from(*s),
                -*o
            ))
        }

        // RO memory addressing
        | Exp::Mem(box Binop(box r, ir::Binop::Add, box Const(o)))
        | Exp::Mem(box Binop(box Const(o), ir::Binop::Add, box r)) => {
            let r = self.tile_exp(r);
            Value::Mem(Mem::RO(
                self.into_temp(r),
                *o
            ))
        },
        | Exp::Mem(box Binop(box r, ir::Binop::Sub, box Const(o))) => {
            let r = self.tile_exp(r);
            Value::Mem(Mem::RO(
                self.into_temp(r),
                -*o
            ))
        },

        // General memory
        | Exp::Mem(box r) => {
            let r = self.tile_exp(r);
            Value::Mem(Mem::R(
                self.into_temp(r)
            ))
        }

        // Negation
        | Exp::Binop(box Const(0), ir::Binop::Sub, box neg) => {

            let neg_tile = self.tile_exp(neg);

            match neg_tile {
            | Value::Mem(mem) => {
                self.asm.push(asm::Asm::Un(asm::Unop::Neg, asm::Unary::M(mem)));
                Value::Mem(mem)
            },
            | temp => {
                let temp = self.into_temp(temp);
                self.asm.push(asm::Asm::Un(asm::Unop::Neg, asm::Unary::R(temp)));
                Value::Temp(temp)
            },
            }
        }

        | _ => unimplemented!(),
        }
    }
}
