use std::fmt;

use fnv::{FnvHashSet, FnvHashMap};
use petgraph::prelude::*;
use petgraph::dot::*;

use ir::*;
use util::Void;
use operand::Label;

#[derive(Debug)]
pub struct Flow {
    start: Label,
    end: Label,
    escapes: usize,
    graph: DiGraphMap<Label, Void>,
    blocks: FnvHashMap<Label, Vec<Stm>>,
}

impl Flow {

    pub fn new(ir: Function) -> Flow {
        let mut graph = DiGraphMap::default();
        let mut blocks = FnvHashMap::default();

        let mut header: Option<Label> = Some(ir.label);
        let mut block = Vec::new();

        for stm in ir.body {

            match stm {
            | Stm::Label(label) => {
                let symbol = label;
                header = Some(symbol);
                block.push(stm);
            },
            | Stm::Jump(Exp::Name(label), _) => {

                let current = header
                    .expect("Internal error: missing header for block");

                graph.add_edge(current, label, Void {});
                block.push(stm);
                blocks.insert(current, block);
                block = Vec::new();
                header = None;
            },
            | Stm::CJump(_, _, _, t_label, f_label) => {

                let current = header
                    .expect("Internal error: missing header for block");

                graph.add_edge(current, t_label, Void {});
                graph.add_edge(current, f_label, Void {});
                block.push(stm);
                blocks.insert(current, block);
                block = Vec::new();
                header = Some(f_label);
            },
            _ => block.push(stm),
            }
        }

        let end = header.expect("Internal error: missing end label");
        blocks.insert(end, block);

        let mut height = FnvHashMap::default();
        let mut seen = FnvHashSet::default();
        let mut flow = Flow {
            start: ir.label,
            end,
            escapes: ir.escapes,
            graph,
            blocks
        };
        flow.trace(flow.start(), &mut height, &mut seen);

        // Remove unreachable nodes
        for label in flow.blocks.keys() {
            if !height.contains_key(label) {
                flow.graph.remove_node(*label);
            }
        }

        flow.blocks.retain(|label, _| height.contains_key(label));

        flow
    }

    pub fn linearize(mut self) -> Function {

        let mut height = FnvHashMap::default();
        let mut seen = FnvHashSet::default();
        let mut reordered = Vec::new();
        self.trace(self.start(), &mut height, &mut seen);

        while !self.blocks.is_empty() {

            let mut node_symbol = self.blocks.keys()
                .max_by_key(|symbol| height[symbol])
                .cloned()
                .expect("Impossible: blocks is non-empty");

            let mut node_block = self.remove(node_symbol)
                .expect("Impossible: start symbol is from keys iterator");
            
            reordered.append(&mut node_block);

            while let Some(symbol) = self.graph
                .neighbors(node_symbol)
                .max_by_key(|symbol| height[symbol]) {
                
                node_symbol = symbol;
                node_block = self.remove(node_symbol)
                    .expect("Internal error: inconsistent state between blocks and graph");

                reordered.append(&mut node_block);
            }
        }

        Function {
            label: self.start,
            body: reordered,
            escapes: self.escapes,
        }
    }

    pub fn start(&self) -> Label {
        self.start
    }

    pub fn end(&self) -> Label {
        self.end
    }

    pub fn trace(&self, node: Label, map: &mut FnvHashMap<Label, usize>, seen: &mut FnvHashSet<Label>) -> usize {
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

    pub fn remove(&mut self, node: Label) -> Option<Vec<Stm>> {
        self.graph.remove_node(node);
        self.blocks.remove(&node)
    }
}

impl fmt::Display for Flow {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", Dot::with_config(&self.graph, &[Config::EdgeNoLabel]))
    }
}
