use asm::*;
use operand::*;

pub enum Location {
    M(Mem<Reg>),
    R(Reg), 
}

pub trait Allocator {
    fn get(&mut self, temp: Temp) -> Reg;
}

pub fn allocate<A: Allocator>(allocator: A, unit: Unit<Temp>) -> Unit<Reg> {
    unimplemented!()
}

fn allocate_mem<A: Allocator>(allocator: &mut A, mem: Mem<Temp>) -> Mem<Reg> {
    match mem {
    | Mem::R(temp)          => Mem::R(allocator.get(temp)),
    | Mem::RO(temp, offset) => Mem::RO(allocator.get(temp), offset),
    }
}
