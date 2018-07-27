use std::mem;
use simple_symbol::Symbol;
use fnv::FnvHashMap;

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

    fn store_temps(&mut self, asm: &mut Vec<Asm<Reg>>);

    fn load_temp(&mut self, asm: &mut Vec<Asm<Reg>>, temp: Temp) -> Reg;

    fn load_mem(&mut self, asm: &mut Vec<Asm<Reg>>, mem: Mem<Temp>) -> Mem<Reg> {
        match mem {
        | Mem::R(temp)          => Mem::R(self.load_temp(asm, temp)),
        | Mem::RO(temp, offset) => Mem::RO(self.load_temp(asm, temp), offset),
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
            self.allocated.push(stm);
            self.assigner.store_temps(&mut self.allocated);
        }

        let stack_size = self.assigner.get_stack_size();
        let stack_size = if stack_size % 2 == 0 { stack_size } else { stack_size + 1 };
        let stack_op = Binary::IR(Imm(stack_size as i32 * WORD_SIZE), Reg::RSP);

        self.allocated = mem::replace(&mut self.allocated, Vec::with_capacity(0))
            .into_iter()
            .map(|stm| {
                match stm {
                | Asm::Comment(sym) if sym == sub_rsp => Asm::Bin(Binop::Sub, stack_op),
                | Asm::Comment(sym) if sym == add_rsp => Asm::Bin(Binop::Add, stack_op),
                | stm => stm,
                }
            })
            .collect()
    }

    fn load_temp(&mut self, temp: Temp) -> Reg {
        self.assigner.load_temp(&mut self.allocated, temp)
    }

    fn load_mem(&mut self, mem: Mem<Temp>) -> Mem<Reg> {
        self.assigner.load_mem(&mut self.allocated, mem)
    }

    fn allocate_unary(&mut self, unary: &Unary<Temp>) -> Unary<Reg> {
        match unary {
        | Unary::R(temp) => Unary::R(self.load_temp(*temp)),
        | Unary::M(mem)  => Unary::M(self.load_mem(*mem)),
        }
    }

    fn allocate_binary(&mut self, binary: &Binary<Temp>) -> Binary<Reg> {
        match binary {
        | Binary::IR(imm, temp)      => Binary::IR(*imm,                    self.load_temp(*temp)),
        | Binary::IM(imm, mem)       => Binary::IM(*imm,                    self.load_mem(*mem)),
        | Binary::RM(temp, mem)      => Binary::RM(self.load_temp(*temp),   self.load_mem(*mem)),
        | Binary::MR(mem, temp)      => Binary::MR(self.load_mem(*mem),     self.load_temp(*temp)),
        | Binary::LR(label, temp)    => Binary::LR(*label,                  self.load_temp(*temp)),
        | Binary::RR(temp_a, temp_b) => Binary::RR(self.load_temp(*temp_a), self.load_temp(*temp_b)),
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
        | Asm::Lea(mem, temp)  => Asm::Lea(self.load_mem(*mem), self.load_temp(*temp)),
        | Asm::Cmp(binary)     => Asm::Cmp(self.allocate_binary(binary)),
        | stm                  => (*stm).into(),
        }
    }
}

pub struct Trivial {
    temps: FnvHashMap<Temp, i32>,
    stack_size: usize,
    stores: Vec<Asm<Reg>>,
}

impl Assigner for Trivial {

    fn new(stack_size: usize) -> Self {
        Trivial {
            temps: FnvHashMap::default(),
            stack_size,
            stores: Vec::new(),
        }
    }

    fn get_stack_size(&self) -> usize {
        self.stack_size
    }

    fn store_temps(&mut self, asm: &mut Vec<Asm<Reg>>) {
        asm.append(&mut self.stores);
    }

    fn load_temp(&mut self, asm: &mut Vec<Asm<Reg>>, temp: Temp) -> Reg {

        if let Temp::Reg(fixed) = temp { return fixed }
        
        if !self.temps.contains_key(&temp) {
            self.stack_size += 1;
            self.temps.insert(temp, self.stack_size as i32);
        }

        let mem = Mem::RO(Reg::RBP, -(self.temps[&temp] * WORD_SIZE));
        
        let reg = if self.stores.is_empty() {
            Reg::R10
        } else {
            Reg::R11
        };
        
        self.stores.push(Asm::Mov(Binary::RM(reg, mem)));
        asm.push(Asm::Mov(Binary::MR(mem, reg)));
        reg
    }
}
