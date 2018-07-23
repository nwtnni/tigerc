use fnv::FnvHashMap;

use ir::*;
use operand::*;

const MEM_SIZE: usize = 1024;

pub struct Debugger<'ir> {
    ir: &'ir [Stm],
    pc: usize,
    env: FnvHashMap<Temp, i32>,
    stack: Vec<i32>,
}

impl <'ir> Debugger<'ir> {

    pub fn new(ir: &'ir [Stm]) -> Self {
        Debugger {
            ir,
            pc: 0,
            env: hashmap! {
                Temp::Reg(Reg::RBP) => 0,            
                Temp::Reg(Reg::RSP) => 0
            },
            stack: vec![0; MEM_SIZE],
        }
    }
}
