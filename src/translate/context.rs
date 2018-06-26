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
    map: FnvHashMap<Symbol, Access>,
    offset: i32,
}

impl Frame {
    pub fn new(name: ir::Label, args: Vec<(Symbol, bool)>) -> Self {
        unimplemented!()
    }

    pub fn name(&self) -> ir::Label {
        self.name
    }

    pub fn allocate(&mut self, name: Symbol, escape: bool) -> Access {
        unimplemented!()
    }

    pub fn get(&self, name: Symbol) -> Access {
        self.map[&name]
    }
}

#[derive(Default)]
pub struct FnContext {
    map: FnvHashMap<Symbol, ir::Label>,
}

impl FnContext {
    pub fn contains(&self, name: &Symbol) -> bool {
        self.map.contains_key(name)
    }

    pub fn get(&self, name: &Symbol) -> Option<ir::Label> {
        self.map.get(name).cloned()
    }

    pub fn insert(&mut self, name: Symbol) -> ir::Label {
        let label = ir::Label::with_symbol(name);
        self.map.insert(name, label);
        label
    }
}
