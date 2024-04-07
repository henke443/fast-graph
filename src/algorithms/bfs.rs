#[cfg(feature = "hashbrown")]
use hashbrown::HashSet;

#[cfg(not(feature = "hashbrown"))]
use std::collections::HashSet;

//#[cfg(feature = "std")]
use std::collections::VecDeque;

// #[cfg(not(feature = "std"))]
// use alloc::collections::VecDeque;


use crate::{GraphInterface, NodeID};



pub struct BreadthFirstSearch<'a, G: GraphInterface> {
    graph: &'a G,
    start: NodeID,
    visited: HashSet<NodeID>,
    queue: VecDeque<NodeID>,
    visited_edges: Vec<(NodeID, NodeID)>
}

impl<'a, G: GraphInterface> BreadthFirstSearch<'a, G> {
    pub fn new(graph: &'a G, start: NodeID) -> Self {
        Self {
            graph,
            start,
            visited: HashSet::new(),
            queue: VecDeque::from(vec![start]),
            visited_edges: Vec::new(),
        }
    }
}

impl <'a, G: GraphInterface> Iterator for BreadthFirstSearch<'a, G> {
    type Item = NodeID;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.queue.pop_front() {
            if self.visited.contains(&node) {
                return self.next();
            }
            self.visited.insert(node);

            let node = self.graph.node(node).unwrap();
            for edge in &node.connections {
                let edge = self.graph.edge(*edge).unwrap();
                if !self.visited.contains(&edge.to) {
                    self.queue.push_back(edge.to);
                    self.visited_edges.push((edge.from, edge.to));
                }
            }

            return Some(node.id);
        }
        None
    }
}

impl<'a, G: GraphInterface> BreadthFirstSearch<'a, G> {
    pub fn visited_edges(&self) -> &Vec<(NodeID, NodeID)> {
        &self.visited_edges
    }

    pub fn visited(&self) -> &HashSet<NodeID> {
        &self.visited
    }
}
