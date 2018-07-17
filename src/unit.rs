use ir::*;
use translate::Frame;
use operand::*;

#[derive(Debug)]
pub struct Unit {
    pub label: Label,
    pub prologue: Vec<Stm>,
    pub body: Vec<Stm>,
    pub epilogue: Vec<Stm>,
    pub size: usize,
}

impl Unit {
    pub fn new(frame: Frame, body: Tree) -> Self {
        let return_temp = Temp::from_str("RETURN");
        let return_reg = Temp::Reg(Reg::get_return());

        Unit {
            label: frame.label,
            prologue: frame.prologue,
            epilogue: vec![
                Stm::Move(
                    Exp::Temp(return_temp),
                    Exp::Temp(return_reg),
                )
            ],
            size: frame.size,
            body: vec![
                Stm::Move(
                    body.into(),
                    Exp::Temp(return_temp)
                )
            ],
        }
    }
}
