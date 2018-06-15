use ir;

#[derive(Clone)]
pub enum Escape { Y, N }

#[derive(Clone)]
pub enum Access {
    Stack(usize),
    Reg(ir::Temp),
}

trait Frame {

    fn new(name: ir::Label, escapes: Vec<Escape>) -> Self;

    fn name(&self) -> ir::Label;

    fn arguments(&self) -> Vec<Access>;

    fn allocate(&self, escape: Escape) -> Access;

}
