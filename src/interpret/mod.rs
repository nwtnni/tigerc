use fnv::FnvHashMap;

use ir::*;
use operand::*;

pub struct Debugger<'ir> {
    ir: &'ir [Stm],
    pc: usize,
    env: FnvHashMap<Temp, i32>,
    stack: FnvHashMap<usize, i32>,
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
            stack: FnvHashMap::default(),
        }
    }
}
