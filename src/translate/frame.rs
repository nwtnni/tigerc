use fnv::FnvHashMap;
use sym::Symbol;

use ir;

#[derive(Clone, Copy)]
pub enum Access {
    Frame(i32),
    Reg(ir::Temp),
}

pub struct Frame {
    name: ir::Label,
    stack: i32,
    map: FnvHashMap<Symbol, Access>,
}

impl Frame {

    pub fn new(name: ir::Label, args: Vec<(Symbol, bool)>) -> Frame {
        unimplemented!()
    }

    pub fn name(&self) -> ir::Label {
        self.name
    }

    pub fn push(&mut self, escape: bool) -> Access {
        unimplemented!()
    }

    pub fn get(&self, name: Symbol) -> Option<Access> {
        self.map.get(&name).cloned()
    }
}
