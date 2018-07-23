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

        use ir::Exp::{Binop, Const, Name, Temp};

        match exp {
        | Const(n) => Value::Imm(Imm(*n)),
        | Name(l)  => Value::Label(*l),
        | Temp(t)  => Value::Temp(*t),
        | Exp::ESeq(_, _) => panic!("Internal error: non-canonical IR"),

        // BRSO memory addressing
        | Exp::Mem(box Binop(box Binop(box Temp(b), ir::Binop::Add, box Binop(box Temp(r), ir::Binop::Mul, box Const(s))), ir::Binop::Add, box Const(o)))
        | Exp::Mem(box Binop(box Binop(box Temp(b), ir::Binop::Add, box Binop(box Const(s), ir::Binop::Mul, box Temp(r))), ir::Binop::Add, box Const(o)))
        | Exp::Mem(box Binop(box Binop(box Binop(box Temp(r), ir::Binop::Mul, box Const(s)), ir::Binop::Add, box Temp(b)), ir::Binop::Add, box Const(o)))
        | Exp::Mem(box Binop(box Binop(box Binop(box Const(s), ir::Binop::Mul, box Temp(r)), ir::Binop::Add, box Temp(b)), ir::Binop::Add, box Const(o)))
        | Exp::Mem(box Binop(box Const(o), ir::Binop::Add, box Binop(box Temp(b), ir::Binop::Add, box Binop(box Temp(r), ir::Binop::Mul, box Const(s)))))
        | Exp::Mem(box Binop(box Const(o), ir::Binop::Add, box Binop(box Temp(b), ir::Binop::Add, box Binop(box Const(s), ir::Binop::Mul, box Temp(r)))))
        | Exp::Mem(box Binop(box Const(o), ir::Binop::Add, box Binop(box Binop(box Temp(r), ir::Binop::Mul, box Const(s)), ir::Binop::Add, box Temp(b))))
        | Exp::Mem(box Binop(box Const(o), ir::Binop::Add, box Binop(box Binop(box Const(s), ir::Binop::Mul, box Temp(r)), ir::Binop::Add, box Temp(b)))) => {
            Value::Mem(Mem::BRSO(*b, *r, Scale::try_from(*s), *o))
        },
        | Exp::Mem(box Binop(box Binop(box Temp(b), ir::Binop::Add, box Binop(box Temp(r), ir::Binop::Mul, box Const(s))), ir::Binop::Sub, box Const(o)))
        | Exp::Mem(box Binop(box Binop(box Temp(b), ir::Binop::Add, box Binop(box Const(s), ir::Binop::Mul, box Temp(r))), ir::Binop::Sub, box Const(o)))
        | Exp::Mem(box Binop(box Binop(box Binop(box Temp(r), ir::Binop::Mul, box Const(s)), ir::Binop::Add, box Temp(b)), ir::Binop::Sub, box Const(o)))
        | Exp::Mem(box Binop(box Binop(box Binop(box Const(s), ir::Binop::Mul, box Temp(r)), ir::Binop::Add, box Temp(b)), ir::Binop::Sub, box Const(o))) => {
            Value::Mem(Mem::BRSO(*b, *r, Scale::try_from(*s), -*o))
        },

        // RSO memory addressing
        | Exp::Mem(box Binop(box Binop(box Temp(r), ir::Binop::Mul, box Const(s)), ir::Binop::Add, box Const(o)))
        | Exp::Mem(box Binop(box Binop(box Const(s), ir::Binop::Mul, box Temp(r)), ir::Binop::Add, box Const(o)))
        | Exp::Mem(box Binop(box Const(o), ir::Binop::Add, box Binop(box Temp(r), ir::Binop::Mul, box Const(s))))
        | Exp::Mem(box Binop(box Const(o), ir::Binop::Add, box Binop(box Const(s), ir::Binop::Mul, box Temp(r)))) => {
            Value::Mem(Mem::RSO(*r, Scale::try_from(*s), *o))
        }
        | Exp::Mem(box Binop(box Binop(box Temp(r), ir::Binop::Mul, box Const(s)), ir::Binop::Sub, box Const(o)))
        | Exp::Mem(box Binop(box Binop(box Const(s), ir::Binop::Mul, box Temp(r)), ir::Binop::Sub, box Const(o))) => {
            Value::Mem(Mem::RSO(*r, Scale::try_from(*s), -*o))
        }

        // RO memory addressing
        | Exp::Mem(box Binop(box Temp(r), ir::Binop::Add, box Const(o)))
        | Exp::Mem(box Binop(box Const(o), ir::Binop::Add, box Temp(r))) => {
            Value::Mem(Mem::RO(*r, *o))
        },
        | Exp::Mem(box Binop(box Temp(r), ir::Binop::Sub, box Const(o))) => {
            Value::Mem(Mem::RO(*r, -*o))
        },

        // R memory addressing
        | Exp::Mem(box Temp(r)) => {
            Value::Mem(Mem::R(*r))
        }

        // General memory 
        | Exp::Mem(box mem) => {
            let value = self.tile_exp(mem);

            Value::Mem(Mem::R(
                self.into_temp(value)
            ))
        }


        | _ => unimplemented!(),
        }

    }

}
