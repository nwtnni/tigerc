use fnv::FnvHashMap;
use sym::Symbol;

use ir;
use config::WORD_SIZE;
use operand::{Label, Temp, Reg};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Access {
    Frame(i32),
    Reg(Temp),
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

#[derive(Debug)]
pub struct Frame {
    label: Label,
    prologue: Vec<ir::Stm>,
    body: Option<ir::Stm>,
    epilogue: Vec<ir::Stm>,
    map: FnvHashMap<Symbol, Access>,
    offset: i32,
    size: usize,
}

impl Frame {
    pub fn new(label: Label, args: Vec<(Symbol, bool)>) -> Self {
        let rbp = ir::Exp::Temp(Temp::Reg(Reg::RBP));
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
                Access::Reg(Temp::from_str("ARG"))
            };

            prologue.push(ir::Stm::Move(from, to.from_base(rbp.clone())));
            map.insert(*name, to);
        }

        Frame {
            label,
            prologue,
            body: None,
            epilogue: Vec::new(),
            map,
            offset,
            size,
        }
    }

    pub fn label(&self) -> Label {
        self.label
    }

    pub fn allocate(&mut self, name: Symbol, escape: bool) -> ir::Exp {
        let rbp = ir::Exp::Temp(Temp::Reg(Reg::RBP));
        let access = if escape {
            self.offset -= 1;
            self.size += 1;
            Access::Frame(self.offset)
        } else {
            Access::Reg(
                Temp::from_str("LOCAL")
            )
        };

        self.map.insert(name, access);;
        access.from_base(rbp)
    }

    pub fn contains(&self, name: Symbol) -> bool {
        self.map.contains_key(&name)
    }

    pub fn get(&self, name: Symbol, base: ir::Exp) -> ir::Exp {
        self.map[&name].from_base(base)
    }

    pub fn wrap(mut self, body: ir::Tree) -> Self {
        self.body = Some(body.into());
        self
    }

    fn get_argument(i: usize) -> ir::Exp {
        if i < 6 {
            ir::Exp::Temp(
                Temp::from_reg(
                    Reg::get_argument(i)
                )
            )
        } else {
            let fp = ir::Exp::Temp(
                Temp::from_reg(Reg::RBP)
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
