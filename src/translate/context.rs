use fnv::FnvHashMap;
use sym::Symbol;

use ir;

#[derive(Clone, Copy)]
pub enum Access {
    Frame(i32),
    Reg(ir::Temp),
}

pub struct FrameContext {
    name: ir::Label,
    map: FnvHashMap<Symbol, Access>,
    offset: i32,
}

impl FrameContext {

    pub fn new(name: ir::Label, args: Vec<(Symbol, bool)>) -> Self {
        unimplemented!()
    }

    pub fn name(&self) -> ir::Label {
        self.name
    }

    pub fn push(&mut self, name: Symbol, escape: bool) -> Access {
        unimplemented!()
    }

    pub fn get(&self, name: Symbol) -> Access {
        self.map[&name]
    }
}
