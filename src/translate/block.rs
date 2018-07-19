use fnv::FnvHashMap;
use simple_symbol::Symbol;
use petgraph::prelude::*;
use petgraph::dot::*;

use ir::*;
use util::Void;

pub type FlowGraph = DiGraphMap<Symbol, Void>;

#[derive(Debug)]
pub struct Flow<'ir> {
    graph: FlowGraph,
    blocks: FnvHashMap<Symbol, Vec<&'ir Stm>>,
}

impl <'ir> Flow<'ir> {

    pub fn new(ir: &[Stm]) -> Flow {
        let mut graph = DiGraphMap::default();
        let mut blocks = FnvHashMap::default();

        let mut header: Option<Symbol> = None;
        let mut block = Vec::new();

        for stm in ir {

            match stm {
            | Stm::Label(label) => header = Some(label.into()),
            | Stm::Jump(Exp::Name(label), _) => {
                
                let current = header
                    .expect("Internal error: missing header for block");

                graph.add_edge(current, label.into(), Void {});
                blocks.insert(current, block);

                block = Vec::new();
                header = None;
            },
            | Stm::CJump(_, _, _, t_label, f_label) => {

                let current = header
                    .expect("Internal error: missing header for block");

                graph.add_edge(current, t_label.into(), Void {});
                graph.add_edge(current, f_label.into(), Void {});
                blocks.insert(current, block);

                block = Vec::new();
                header = None;
            },
            _ => (),
            }

            block.push(stm);
        }

        Flow { graph, blocks }
    }

    pub fn export(&self) -> String {
        format!("{}", Dot::with_config(&self.graph, &[Config::EdgeNoLabel]))
    }
}

