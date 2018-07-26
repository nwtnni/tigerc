use fnv::{FnvHashSet, FnvHashMap};
use simple_symbol::Symbol;
use petgraph::prelude::*;
use petgraph::dot::*;

use ir::*;
use util::Void;
use operand::Label;

#[derive(Debug)]
pub struct Flow {
    start: Symbol,
    graph: DiGraphMap<Symbol, Void>,
    blocks: FnvHashMap<Symbol, Vec<Stm>>,
}

impl Flow {

    pub fn new(start: Label, ir: Vec<Stm>) -> Flow {
        let mut graph = DiGraphMap::default();
        let mut blocks = FnvHashMap::default();

        let mut header: Option<Symbol> = Some(start.into());
        let mut block = Vec::new();

        for stm in ir {

            match stm {
            | Stm::Label(label) => {
                let symbol = label.into();
                header = Some(symbol);
                block.push(stm);
            },
            | Stm::Jump(Exp::Name(label), _) => {

                let current = header
                    .expect("Internal error: missing header for block");

                graph.add_edge(current, label.into(), Void {});
                block.push(stm);
                blocks.insert(current, block);

                block = Vec::new();
                header = None;
            },
            | Stm::CJump(_, _, _, t_label, f_label) => {

                let current = header
                    .expect("Internal error: missing header for block");

                graph.add_edge(current, t_label.into(), Void {});
                graph.add_edge(current, f_label.into(), Void {});
                block.push(stm);
                blocks.insert(current, block);

                block = Vec::new();
                header = None;
            },
            _ => block.push(stm),
            }

        }

        blocks.insert(
            header.expect("Internal error: missing final header"),
            block
        );

        Flow { start: start.into(), graph, blocks }
    }

    pub fn export(&self) -> String {
        format!("{}", Dot::with_config(&self.graph, &[Config::EdgeNoLabel]))
    }

    fn trace(&self, node: Symbol, map: &mut FnvHashMap<Symbol, usize>, seen: &mut FnvHashSet<Symbol>) -> usize {
        seen.insert(node);

        let neighbors = self.graph.neighbors(node)
            .filter(|node| !seen.contains(node))
            .collect::<Vec<_>>();

        let height = neighbors.into_iter()
            .map(|node| self.trace(node, map, seen) + 1)
            .max()
            .unwrap_or(0);

        seen.remove(&node);
        map.insert(node, height);
        height
    }

    fn remove(&mut self, node: Symbol) -> Option<Vec<Stm>> {
        self.graph.remove_node(node);
        self.blocks.remove(&node)
    }
}

pub fn reorder(unit: Unit) -> Unit {

    let mut flow = Flow::new(unit.label, unit.body);
    let mut height = FnvHashMap::default();
    let mut seen = FnvHashSet::default();
    let mut reordered = Vec::new();
    flow.trace(flow.start, &mut height, &mut seen);

    while !flow.blocks.is_empty() {

        let mut node_symbol = flow.blocks.keys()
            .max_by_key(|symbol| height[symbol])
            .cloned()
            .expect("Impossible: blocks is non-empty");

        let mut node_block = flow.remove(node_symbol)
            .expect("Impossible: start symbol is from keys iterator");
        
        reordered.append(&mut node_block);

        while let Some(symbol) = flow.graph
            .neighbors(node_symbol)
            .max_by_key(|symbol| height[symbol]) {
            
            node_symbol = symbol;
            node_block = flow.remove(node_symbol)
                .expect("Internal error: inconsistent state between blocks and graph");

            reordered.append(&mut node_block);
        }
    }

    Unit {
        data: unit.data,
        label: unit.label,
        body: reordered,
        escapes: unit.escapes,
    }
}

pub fn condense(unit: Unit) -> Unit {

    let mut condense = FnvHashSet::default();

    for i in 0..unit.body.len() {
        match (unit.body.get(i), unit.body.get(i + 1)) {
        | (Some(Stm::Jump(Exp::Name(target), _)), Some(Stm::Label(label))) if target == label => {},
        | _ => { condense.insert(i); },
        }
    }

    unit.map(|body| {
        body.into_iter()
            .enumerate()
            .filter(|(i, _)| condense.contains(i))
            .map(|(_, stm)| stm)
            .collect()
    })
}
