use asm::*;
use operand::*;

pub fn coalesce<T: Operand>(unit: Unit<T>) -> Unit<T> {
    Unit {
        data: unit.data,
        functions: unit.functions.into_iter()
            .map(coalesce_function)
            .collect()
    }
}

pub fn coalesce_function<T: Operand>(asm: Function<T>) -> Function<T> {

    use self::Binary::*;

    let mut coalesced = Vec::new();
    let mut i = 0;
    let len = asm.body.len();

    while i < len {
        
        if i == len - 1 {
            coalesced.push(asm.body[i]);
            break;
        }

        match (asm.body[i], asm.body[i + 1]) {
        | (Asm::Mov(IR(imm, reg_a)), Asm::Mov(RM(reg_b, mem))) if reg_a == reg_b => {
            coalesced.push(Asm::Mov(IM(imm, mem)));
        }
        | (Asm::Mov(IR(imm, reg_a)), Asm::Mov(RR(reg_b, reg_c))) if reg_a == reg_b => {
            coalesced.push(Asm::Mov(IR(imm, reg_c)));
        }
        | (Asm::Mov(IM(imm, mem_a)), Asm::Mov(MR(mem_b, reg))) if mem_a == mem_b => {
            coalesced.push(Asm::Mov(IR(imm, reg)));
        }
        | (Asm::Mov(MR(mem, reg_a)), Asm::Mov(RR(reg_b, reg_c))) if reg_a == reg_b => {
            coalesced.push(Asm::Mov(MR(mem, reg_c)));
        }
        | (Asm::Mov(RM(reg_a, mem_a)), Asm::Mov(MR(mem_b, reg_b))) if mem_a == mem_b => {
            coalesced.push(Asm::Mov(RR(reg_a, reg_b)));
        }
        | (Asm::Mov(RR(reg_a, reg_b)), Asm::Mov(RR(reg_c, reg_d))) if reg_b == reg_c => {
            coalesced.push(Asm::Mov(RR(reg_a, reg_d)));
        }
        | (Asm::Mov(RR(reg_a, reg_b)), Asm::Mov(RM(reg_c, mem))) if reg_b == reg_c => {
            coalesced.push(Asm::Mov(RM(reg_a, mem)));
        }
        | _ => {
            coalesced.push(asm.body[i]);
            i += 1;
            continue
        }
        }
        
        // Skip past coalesced move
        i += 2;
    }

    Function {
        body: coalesced,
        stack_info: asm.stack_info,
    }
}
