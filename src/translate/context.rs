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

impl Access {
    fn from_base(self, base: ir::Exp) -> ir::Exp {
        match self {
        | Access::Reg(temp) => ir::Exp::Temp(temp),
        | Access::Frame(n) => {

            let offset = ir::Exp::Const(
                n * WORD_SIZE
            );

            ir::Exp::Binop(
                Box::new(base),
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
        let rbp = ir::Exp::Temp(ir::Temp::Reg(Reg::RBP));
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

            prologue.push(ir::Stm::Move(from, to.from_base(rbp.clone())));
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
        let rbp = ir::Exp::Temp(ir::Temp::Reg(Reg::RBP));
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
        access.from_base(rbp)
    }

    pub fn get(&self, name: Symbol, base: ir::Exp) -> Option<ir::Exp> {
        self.map.get(&name).map(|&access| access.from_base(base))
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
