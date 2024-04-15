#[cfg(feature = "hashbrown")]
use hashbrown::HashSet;

#[cfg(not(feature = "hashbrown"))]
use std::collections::HashSet;

//#[cfg(feature = "std")]
use std::collections::VecDeque;

// #[cfg(not(feature = "std"))]
// use alloc::collections::VecDeque;


use crate::{EdgeID, GraphInterface, NodeID};



// pub struct BreadthFirstSearch<'a, G: GraphInterface> {
//     graph: &'a G,
//     start: NodeID,
//     visited: HashSet<NodeID>,
//     queue: VecDeque<NodeID>,
//     visited_edges: Vec<(NodeID, NodeID)>
// }

// impl<'a, G: GraphInterface> BreadthFirstSearch<'a, G> {
//     pub fn new(graph: &'a G, start: NodeID) -> Self {
//         Self {
//             graph,
//             start,
//             visited: HashSet::new(),
//             queue: VecDeque::from(vec![start]),
//             visited_edges: Vec::new(),
//         }
//     }
// }

// impl <'a, G: GraphInterface> Iterator for BreadthFirstSearch<'a, G> {
//     type Item = NodeID;

//     fn next(&mut self) -> Option<Self::Item> {
        
//     }
// }

// impl<'a, G: GraphInterface> BreadthFirstSearch<'a, G> {
//     pub fn visited_edges(&self) -> &Vec<(NodeID, NodeID)> {
//         &self.visited_edges
//     }

//     pub fn visited(&self) -> &HashSet<NodeID> {
//         &self.visited
//     }
// }

pub trait IterBreadthFirst<'a, G: GraphInterface> {
    fn iter_breadth_first<'b>(&'b self, start: NodeID) -> Box<impl Iterator<Item = EdgeID>>;
}

// impl<'a, G: GraphInterface> IterBreadthFirst<'a, G> for G {
//     fn iter_breadth_first<'b>(&'b self, start: NodeID) -> Box<impl Iterator<Item = EdgeID>> {
//         let node = self.node(start).unwrap();
//         Box::new(node.connections).iter().fold(Box::new(node.connections.iter()), |acc, e| {
//             let edge = self.edge(*e).unwrap();
//             let child_node = self.node(edge.to).unwrap();
//             return self.iter_breadth_first(edge.to)
//         })
//     }
// }