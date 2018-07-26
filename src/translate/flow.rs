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
                header = Some(f_label.into());
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

    let body_len = unit.body.len();
    let mut condensed = Vec::new();

    for i in 0..body_len {
        if i == body_len - 1 {
            condensed.push(unit.body[i].clone()); 
            break
        }

        match (&unit.body[i], &unit.body[i + 1]) {
        | (Stm::Jump(Exp::Name(j_label), _), Stm::Label(label)) if j_label == label => (),
        | (Stm::CJump(_, _, _, _, f_label), Stm::Label(label)) if f_label == label => {
            condensed.push(unit.body[i].clone())  
        }
        | (Stm::CJump(l, op, r, t_label, f_label), Stm::Label(label)) if t_label == label => {
            condensed.push(Stm::CJump(l.clone(), op.negate(), r.clone(), *f_label, *t_label));
        }
        | (Stm::CJump(l, op, r, t_label, f_label), _) => {
            let label = Label::from_str("CONDENSE_CJUMP");
            condensed.push(Stm::CJump(l.clone(), *op, r.clone(), *t_label, label));
            condensed.push(Stm::Label(label));
            condensed.push(Stm::Jump(Exp::Name(*f_label), vec![*f_label]));
        }
        | _ => {
            condensed.push(unit.body[i].clone())  
        },
        }
    }

    Unit {
        data: unit.data,
        label: unit.label,
        body: condensed,
        escapes: unit.escapes,
    }
}

pub fn clean(unit: Unit) -> Unit {

    let mut used = FnvHashSet::default();

    for stm in &unit.body {
        match stm {
        | Stm::Jump(Exp::Name(label), _) => { used.insert(*label); },
        | Stm::CJump(_, _, _, label, _)  => { used.insert(*label); },
        | _ => (),
        }
    }

    unit.map(|body| {
        body.into_iter()
            .filter(|stm| {
                match stm {
                | Stm::Label(label) => used.contains(&label),
                | _ => true,
                }
            })
            .collect()
    })
}
