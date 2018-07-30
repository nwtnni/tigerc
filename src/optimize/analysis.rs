use std::hash::Hash;

use fnv::FnvHashSet;
use petgraph::{Directed, Direction::*};
use petgraph::graphmap::{DiGraphMap, NodeTrait, NeighborsDirected};

pub trait Value: Clone + Eq + Hash {}
impl <N: Clone + Eq + Hash> Value for N {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Set<V: Value>(FnvHashSet<V>);

impl <V: Value> Set<V> {
    pub fn default() -> Self {
        Set(FnvHashSet::default())
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Set(
            self.0.union(&other.0)
                .cloned()
                .collect()
        )
    }

    pub fn union(&self, other: &Self) -> Self {
        Set(
            self.0.intersection(&other.0)
                .cloned()
                .collect()
        )
    }
}

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

pub trait Meet {
    fn meet<T: Clone + Hash + Eq, I: Iterator<Item = Set<T>>>(sets: I) -> Set<T>;
}

pub struct Union {}

impl Meet for Union {
    fn meet<V: Value, I: Iterator<Item = Set<V>>>(sets: I) -> Set<V> {
        sets.fold(Set::default(), |a, b| a.union(&b))
    }
}

pub struct Intersection {}

impl Meet for Intersection {
    fn meet<V: Value, I: Iterator<Item = Set<V>>>(mut sets: I) -> Set<V> {
        if let Some(set) = sets.next() {
            sets.fold(set.clone(), |a, b| a.intersection(&b))
        } else {
            Set::default()
        }
    }
}

pub trait Transfer<V: Value> {


}

pub trait Analysis<D: Direction, M: Meet, V: Value> {


}
