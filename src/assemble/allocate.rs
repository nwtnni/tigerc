use std::mem;
use simple_symbol::Symbol;
use fnv::FnvHashMap;

use config::WORD_SIZE;
use asm::*;
use operand::*;

pub fn allocate<A: Assigner>(unit: Unit<Temp>) -> Unit<Reg> {
    Unit {
        data: unit.data.into_iter()
            .map(|directive| directive.into())
            .collect(),
        
        functions: unit.functions.into_iter()
            .map(allocate_function::<A>)
            .collect(),
    }
}

pub fn allocate_function<A: Assigner>(asm: Function<Temp>) -> Function<Reg> {
    let mut allocator = Allocator {
        assigner: A::new(asm.stack_info.0),
        allocated: Vec::new(),
    };

    allocator.allocate(&asm.body, asm.stack_info.1, asm.stack_info.2);

    Function {
        body: allocator.allocated,
        stack_info: asm.stack_info,
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Dir { R, W, RW, }

pub trait Assigner {
    fn new(stack_size: usize) -> Self;

    fn get_stack_size(&self) -> usize;

    fn store_temps(&mut self, asm: &mut Vec<Asm<Reg>>);

    fn load_temps(&mut self, asm: &mut Vec<Asm<Reg>>);

    fn get_temp(&mut self, temp: Temp, dir: Dir) -> Reg;

    fn get_mem(&mut self, mem: Mem<Temp>) -> Mem<Reg> {
        match mem {
        | Mem::R(temp)          => Mem::R(self.get_temp(temp, Dir::R)),
        | Mem::RO(temp, offset) => Mem::RO(self.get_temp(temp, Dir::R), offset),
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
            self.assigner.load_temps(&mut self.allocated);
            self.allocated.push(stm);
            self.assigner.store_temps(&mut self.allocated);
        }

        let stack_size = self.assigner.get_stack_size();
        let stack_size = if stack_size % 2 == 0 { stack_size } else { stack_size + 1 };
        let stack_op = Binary::IR(Imm::Int(stack_size as i32 * WORD_SIZE), Reg::RSP);

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

    fn load_temp(&mut self, temp: Temp, dir: Dir) -> Reg {
        self.assigner.get_temp(temp, dir)
    }

    fn load_mem(&mut self, mem: Mem<Temp>) -> Mem<Reg> {
        self.assigner.get_mem(mem)
    }

    fn allocate_unary(&mut self, unary: &Unary<Temp>, dir: Dir) -> Unary<Reg> {
        match unary {
        | Unary::R(temp) => Unary::R(self.load_temp(*temp, dir)),
        | Unary::M(mem)  => Unary::M(self.load_mem(*mem)),
        }
    }

    fn allocate_binary(&mut self, binary: &Binary<Temp>, dest_dir: Dir) -> Binary<Reg> {
        match binary {
        | Binary::IR(imm, temp)      => Binary::IR(*imm,                            self.load_temp(*temp, dest_dir)),
        | Binary::IM(imm, mem)       => Binary::IM(*imm,                            self.load_mem(*mem)),
        | Binary::RM(temp, mem)      => Binary::RM(self.load_temp(*temp, Dir::R),   self.load_mem(*mem)),
        | Binary::MR(mem, temp)      => Binary::MR(self.load_mem(*mem),             self.load_temp(*temp, dest_dir)),
        | Binary::RR(temp_a, temp_b) => Binary::RR(self.load_temp(*temp_a, Dir::R), self.load_temp(*temp_b, dest_dir)),
        }
    }

    fn allocate_stm(&mut self, stm: &Asm<Temp>) -> Asm<Reg> {
        match stm {
        | Asm::Mov(binary)     => Asm::Mov(self.allocate_binary(binary, Dir::W)),
        | Asm::Bin(op, binary) => Asm::Bin(*op, self.allocate_binary(binary, Dir::RW)),
        | Asm::Mul(unary)      => Asm::Mul(self.allocate_unary(unary, Dir::R)),
        | Asm::Div(div, unary) => Asm::Div(*div, self.allocate_unary(unary, Dir::R)),
        | Asm::Un(op, unary)   => Asm::Un(*op, self.allocate_unary(unary, Dir::RW)),
        | Asm::Pop(unary)      => Asm::Pop(self.allocate_unary(unary, Dir::W)),
        | Asm::Push(unary)     => Asm::Push(self.allocate_unary(unary, Dir::R)),
        | Asm::Lea(mem, temp)  => Asm::Lea(self.load_mem(*mem), self.load_temp(*temp, Dir::W)),
        | Asm::Cmp(binary)     => Asm::Cmp(self.allocate_binary(binary, Dir::R)),
        | stm                  => (*stm).into(),
        }
    }
}

pub struct Trivial {
    temps: FnvHashMap<Temp, i32>,
    stack_size: usize,
    loads: Vec<Asm<Reg>>,
    stores: Vec<Asm<Reg>>,
}

impl Assigner for Trivial {

    fn new(stack_size: usize) -> Self {
        Trivial {
            temps: FnvHashMap::default(),
            stack_size,
            loads: Vec::new(),
            stores: Vec::new(),
        }
    }

    fn get_stack_size(&self) -> usize {
        self.stack_size
    }

    fn store_temps(&mut self, asm: &mut Vec<Asm<Reg>>) {
        asm.append(&mut self.stores);
    }

    fn load_temps(&mut self, asm: &mut Vec<Asm<Reg>>) {
        asm.append(&mut self.loads);
    }

    fn get_temp(&mut self, temp: Temp, dir: Dir) -> Reg {

        if let Temp::Reg(fixed) = temp { return fixed }

        if !self.temps.contains_key(&temp) {
            self.stack_size += 1;
            self.temps.insert(temp, self.stack_size as i32);
        }

        // Temp offset from stack
        let mem = Mem::RO(Reg::RBP, -(self.temps[&temp] * WORD_SIZE));

        // Use neither caller nor callee saved registers
        let reg = if self.stores.len() + self.loads.len() == 0 { Reg::R10 } else { Reg::R11 };

        // Load temp from stack position
        if dir == Dir::R || dir == Dir::RW {
            self.loads.push(Asm::Mov(Binary::MR(mem, reg)));
        }

        // Write temp back to stack position
        if dir == Dir::W || dir == Dir::RW {
            self.stores.push(Asm::Mov(Binary::RM(reg, mem)));
        }

        reg
    }
}
