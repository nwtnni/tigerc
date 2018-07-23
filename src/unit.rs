use std::fmt;

use ir::*;
use translate::Frame;
use operand::*;

#[derive(Debug)]
pub struct Unit {
    pub data: Vec<Static>,
    pub label: Label,
    pub prologue: Vec<Stm>,
    pub body: Vec<Stm>,
    pub epilogue: Vec<Stm>,
    pub size: usize,
}

impl Unit {
    pub fn new(frame: Frame, data: Vec<Static>, body: Tree) -> Self {
        let return_temp = Temp::from_str("RETURN");
        let return_reg = Temp::Reg(Reg::get_return());

        let prologue_label = Label::from_str("PROLOGUE");
        let body_label = Label::from_str("BODY");
        let epilogue_label = Label::from_str("EPILOGUE");

        Unit {
            data,
            label: frame.label,
            prologue: vec![
                Stm::Label(prologue_label),
                Stm::Seq(frame.prologue),
                Stm::Jump(
                    Exp::Name(body_label),
                    vec![body_label]
                ),
            ],
            size: frame.size,
            body: vec![
                Stm::Label(body_label),
                Stm::Move(
                    body.into(),
                    Exp::Temp(return_temp)
                ),
                Stm::Jump(
                    Exp::Name(epilogue_label),
                    vec![epilogue_label]
                ),
            ],
            epilogue: vec![
                Stm::Label(epilogue_label),
                Stm::Move(
                    Exp::Temp(return_temp),
                    Exp::Temp(return_reg),
                )
            ],
        }
    }

    pub fn map(self, f: impl Fn(Vec<Stm>) -> Vec<Stm>) -> Self {
        Unit {
            data: self.data,
            label: self.label,
            prologue: f(self.prologue),
            body: f(self.body),
            epilogue: f(self.epilogue),
            size: self.size,
        }
    }
}

impl Into<Vec<Stm>> for Unit {
    fn into(self) -> Vec<Stm> {
        self.prologue.into_iter()
            .chain(self.body.into_iter())
            .chain(self.epilogue.into_iter())
            .collect()
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {

        let combined = self.prologue.iter()
            .chain(self.body.iter())
            .chain(self.epilogue.iter());

        write!(fmt, "{}", self.label)?;

        for stm in combined {
            write!(fmt, "\n    {}", stm)?;
        }
        
        Ok(())
    }
}
