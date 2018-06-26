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
    label: ir::Label,
    prologue: Vec<ir::Stm>,
    epilogue: Vec<ir::Stm>,
    map: FnvHashMap<Symbol, Access>,
    offset: i32,
    size: usize,
}

impl Frame {
    pub fn new(label: ir::Label, args: Vec<(Symbol, bool)>) -> Self {
        let mut map = FnvHashMap::default();
        let mut prologue = Vec::new();
        let mut offset = 0;
        let mut size = 0;

        for (i, (name, escape)) in args.iter().enumerate() {
            let from = Frame::get_argument(i);
            let to = if *escape {
                offset -= 1;
                size += 1;
                Access::Frame(offset)
            } else {
                Access::Reg(ir::Temp::with_name("ARG")) 
            };

            prologue.push(ir::Stm::Move(from, to.into()));
            map.insert(*name, to);
        }

        Frame {
            label,
            prologue,
            epilogue: Vec::new(),
            map,
            offset,
            size,
        }
    }

    pub fn label(&self) -> ir::Label {
        self.label
    }

    pub fn allocate(&mut self, name: Symbol, escape: bool) -> ir::Exp {
        let access = if escape {
            self.offset -= 1;
            self.size += 1;
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

    fn get_argument(i: usize) -> ir::Exp {
        if i < 6 {
            ir::Exp::Temp(
                ir::Temp::with_reg(
                    Reg::get_argument(i)
                )
            )
        } else {
            let fp = ir::Exp::Temp(
                ir::Temp::with_reg(Reg::RBP)
            );

            let offset = ir::Exp::Const(
                i as i32 - 6
            );

            ir::Exp::Mem(
                Box::new(
                    ir::Exp::Binop(
                        Box::new(fp),
                        ir::Binop::Add,
                        Box::new(offset),
                    )
                )
            )
        }
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
