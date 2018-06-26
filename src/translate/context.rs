use fnv::FnvHashMap;
use sym::Symbol;

use config::WORD_SIZE;
use ir;
use operand::Reg;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Access {
    Frame(i32),
    Reg(ir::Temp),
}

impl From<Access> for ir::Exp {
    fn from(access: Access) -> ir::Exp {
        match access {
        | Access::Reg(temp) => ir::Exp::Temp(temp),
        | Access::Frame(n) => {

            let fp = ir::Exp::Temp(
                ir::Temp::with_reg(Reg::RBP)
            );

            let offset = ir::Exp::Const(
                n * WORD_SIZE
            );

            ir::Exp::Binop(
                Box::new(fp),
                ir::Binop::Sub,
                Box::new(offset)
            )
        },
        }
    }
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

    pub fn allocate(&mut self, name: Symbol, escape: bool) -> ir::Exp {
        let access = if escape {
            self.offset -= 1;
            Access::Frame(self.offset)
        } else {
            Access::Reg(
                ir::Temp::with_name("LOCAL")
            )
        };
        
        self.map.insert(name, access);;
        access.into()
    }

    pub fn get(&self, name: Symbol) -> Option<ir::Exp> {
        self.map.get(&name).map(|&access| access.into())
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
