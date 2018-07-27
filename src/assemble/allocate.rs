use std::mem;
use simple_symbol::Symbol;

use config::WORD_SIZE;
use asm::*;
use operand::*;

pub fn allocate<A: Assigner>(asm: Unit<Temp>) -> Unit<Reg> {
    let mut allocator = Allocator {
        assigner: A::new(asm.stack_info.0),
        allocated: Vec::new(),
    };

    allocator.allocate(&asm.asm, asm.stack_info.1, asm.stack_info.2);

    Unit {
        asm: allocator.allocated,

        rodata: asm.rodata.into_iter()
            .map(|stm| stm.into())
            .collect(),

        stack_info: asm.stack_info,
    }
}

pub trait Assigner {
    fn new(stack_size: usize) -> Self;

    fn get_stack_size(&self) -> usize;

    fn store(&mut self, stm: &Asm<Reg>);

    fn get_temp(&mut self, temp: Temp) -> Reg;

    fn get_mem(&mut self, mem: Mem<Temp>) -> Mem<Reg> {
        match mem {
        | Mem::R(temp)          => Mem::R(self.get_temp(temp)),
        | Mem::RO(temp, offset) => Mem::RO(self.get_temp(temp), offset),
        }
    }
}

struct Allocator<A: Assigner> {
    assigner: A,
    allocated: Vec<Asm<Reg>>,
}

impl <A: Assigner> Allocator<A> {

    fn allocate(&mut self, asm: &[Asm<Temp>], sub_rsp: Symbol, add_rsp: Symbol) {
        for stm in asm {
            let stm = self.allocate_stm(stm);
            self.assigner.store(&stm);
            self.allocated.push(stm);
        }

        self.allocated = mem::replace(&mut self.allocated, Vec::with_capacity(0))
            .into_iter()
            .map(|stm| {
                match stm {
                | Asm::Comment(sym) if sym == sub_rsp => {
                    Asm::Bin(Binop::Sub, Binary::IR(Imm(WORD_SIZE * self.assigner.get_stack_size() as i32), Reg::RSP))
                }
                | Asm::Comment(sym) if sym == add_rsp => {
                    Asm::Bin(Binop::Add, Binary::IR(Imm(WORD_SIZE * self.assigner.get_stack_size() as i32), Reg::RSP))
                },
                | stm => stm,
                }
            })
            .collect()
    }

    fn get_temp(&mut self, temp: Temp) -> Reg {
        self.assigner.get_temp(temp)
    }

    fn get_mem(&mut self, mem: Mem<Temp>) -> Mem<Reg> {
        self.assigner.get_mem(mem)
    }

    fn allocate_unary(&mut self, unary: &Unary<Temp>) -> Unary<Reg> {
        match unary {
        | Unary::R(temp) => Unary::R(self.get_temp(*temp)),
        | Unary::M(mem)  => Unary::M(self.get_mem(*mem)),
        }
    }

    fn allocate_binary(&mut self, binary: &Binary<Temp>) -> Binary<Reg> {
        match binary {
        | Binary::IR(imm, temp)      => Binary::IR(*imm, self.get_temp(*temp)),
        | Binary::IM(imm, mem)       => Binary::IM(*imm, self.get_mem(*mem)),
        | Binary::RM(temp, mem)      => Binary::RM(self.get_temp(*temp), self.get_mem(*mem)),
        | Binary::MR(mem, temp)      => Binary::MR(self.get_mem(*mem), self.get_temp(*temp)),
        | Binary::LR(label, temp)    => Binary::LR(*label, self.get_temp(*temp)),
        | Binary::RR(temp_a, temp_b) => Binary::RR(self.get_temp(*temp_a), self.get_temp(*temp_b)),
        }
    }

    fn allocate_stm(&mut self, stm: &Asm<Temp>) -> Asm<Reg> {
        match stm {
        | Asm::Mov(binary)     => Asm::Mov(self.allocate_binary(binary)),
        | Asm::Bin(op, binary) => Asm::Bin(*op, self.allocate_binary(binary)),
        | Asm::Mul(unary)      => Asm::Mul(self.allocate_unary(unary)),
        | Asm::Div(unary)      => Asm::Div(self.allocate_unary(unary)),
        | Asm::Un(op, unary)   => Asm::Un(*op, self.allocate_unary(unary)),
        | Asm::Pop(unary)      => Asm::Pop(self.allocate_unary(unary)),
        | Asm::Push(unary)     => Asm::Push(self.allocate_unary(unary)),
        | Asm::Lea(mem, temp)  => Asm::Lea(self.get_mem(*mem), self.get_temp(*temp)),
        | Asm::Cmp(binary)     => Asm::Cmp(self.allocate_binary(binary)),
        | stm                  => (*stm).into(),
        }
    }
}
