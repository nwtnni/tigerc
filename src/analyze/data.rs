use std::hash::Hash;

use fnv::FnvHashSet;
use petgraph::{Directed, Direction::*};
use petgraph::graphmap::{DiGraphMap, NodeTrait, NeighborsDirected};

pub trait Direction {
    fn neighbors<'graph, N: NodeTrait>(graph: &'graph DiGraphMap<N, Directed>, node: N) -> NeighborsDirected<'graph, N, Directed>;
}

pub struct Forward {}

pub struct Backward {}

impl Direction for Forward {
    fn neighbors<'graph, N: NodeTrait>(graph: &'graph DiGraphMap<N, Directed>, node: N) -> NeighborsDirected<'graph, N, Directed> {
        graph.neighbors_directed(node, Outgoing)
    }
}

impl Direction for Backward {
    fn neighbors<'graph, N: NodeTrait>(graph: &'graph DiGraphMap<N, Directed>, node: N) -> NeighborsDirected<'graph, N, Directed> {
        graph.neighbors_directed(node, Incoming)
    }
}
