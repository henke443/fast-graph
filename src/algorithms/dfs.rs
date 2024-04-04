//! # Under development
use hashbrown::HashSet;

use crate::{
    Edge, Graph, GraphInterface, NodeID
};


/// Under development
pub struct DepthFirstSearch<'a, G: GraphInterface> {
    graph: &'a G,
    start: NodeID,
    visited: HashSet<NodeID>,
    stack: Vec<NodeID>,
    cyclic: bool,
    visited_edges: Vec<(NodeID, NodeID)>
}

impl<'a, G: GraphInterface> DepthFirstSearch<'a, G> {
    pub fn new(graph: &'a G, start: NodeID) -> Self {
        Self {
            graph,
            start,
            visited: HashSet::new(),
            stack: vec![start],
            cyclic: false,
            visited_edges: Vec::new(),
        }
    }
}

impl <'a, G: GraphInterface> Iterator for DepthFirstSearch<'a, G> {
    type Item = NodeID;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            if self.visited.contains(&node) {
                println!("Cyclic graph detected, visited edges: {:#?}", self.visited_edges);
                self.cyclic = true;
                return self.next();
            }
            self.visited.insert(node);

            let node = self.graph.node(node).unwrap();
            for edge in &node.connections {
                let edge = self.graph.edge(*edge).unwrap();
                if (edge.to != self.start) && !self.visited.contains(&edge.to) {
                    self.stack.push(edge.to);
                    self.visited_edges.push((edge.from, edge.to));
                }
                // else if (edge.from != self.start) && !self.visited.contains(&edge.from){
                //     self.stack.push(edge.from)
                // }
            }

            return Some(node.id);
        }
        None
    }
}

impl<'a, G: GraphInterface> std::iter::FusedIterator for DepthFirstSearch<'a, G> {}

impl<'a, G: GraphInterface> std::iter::ExactSizeIterator for DepthFirstSearch<'a, G> {
    fn len(&self) -> usize {
        self.graph.node_count() - self.visited.len()
    }
}

/// Under development
pub trait IterDepthFirst<'a, G: GraphInterface> {
    fn iter_depth_first(&'a self, start: NodeID) -> DepthFirstSearch<'a, G>;
}

impl<'a, G: GraphInterface> IterDepthFirst<'a, G> for G {
    /// Under development
    fn iter_depth_first(&'a self, start: NodeID) -> DepthFirstSearch<'a, G> {
        DepthFirstSearch::new(self, start)
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::GraphInterface;

    #[derive(Clone, Debug)]
    enum NodeData {
        Int64(i64),
    }
    impl PartialEq for NodeData {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (NodeData::Int64(a), NodeData::Int64(b)) => a == b,
            }
        }
    }

    #[test]
    fn test_dfs() {
        let mut graph1: Graph<NodeData, ()> = Graph::new();
        let [node0, node1, node2, node3, node4] = graph1.add_nodes(&[
            NodeData::Int64(0),
            NodeData::Int64(1),
            NodeData::Int64(2),
            NodeData::Int64(3),
            NodeData::Int64(4),
        ])[..] else { panic!("Failed to add nodes") };
        
        
        graph1.add_edges(&[
            (node0, node1),
            (node0, node3),
            (node0, node2),
            (node1, node0),
            (node2, node3),
            (node2, node0),
            (node2, node4),
            (node4, node2),
        ]);


        let mut graph2: Graph<NodeData, ()> = Graph::new();
        let [node02, node12, node22, node32, node42] = graph2.add_nodes(&[
            NodeData::Int64(0),
            NodeData::Int64(1),
            NodeData::Int64(2),
            NodeData::Int64(3),
            NodeData::Int64(4),
        ])[..] else { panic!("Failed to add nodes") };
        
        graph2.add_edges(&[
            (node02, node32),
            (node02, node22),
            (node12, node02),
            (node22, node32),
            (node42, node22),
        ]);



        println!("Depth First Search 1 (node count: {}):", graph1.node_count());
        println!("Edges: {:#?}", graph1.edges.len());
        let mut visited = Vec::new();
        let depth_first = graph1.iter_depth_first(node0);
        for node in  depth_first{
            let node = graph1.node(node).unwrap();
            println!("{:?}", node.data);
            visited.push(node);
        }

        assert_eq!(visited.len(), graph1.node_count());
        assert_eq!(visited.len(), 5);


        println!("Depth First Search 2 (node count: {}):", graph1.node_count());
        println!("Edges: {:#?}", graph1.edges.len());

        let mut visited = Vec::new();
        for node in graph1.iter_depth_first(node0) {
            let node = graph1.node(node).unwrap();
            println!("{:?}", node.data);
            visited.push(node);

            if node.data == NodeData::Int64(4) {
                break
            }
        }

        assert_ne!(visited.len(), graph1.node_count());
        assert_eq!(visited.len(), 3);

        println!("Depth First Search 3 (node count: {}):", graph2.node_count());
        println!("Edges: {:#?}", graph2.edges.len());
        let mut visited2 = Vec::new();
        for node in graph2.iter_depth_first(node02) {
            let node = graph2.node(node).unwrap();
            println!("{:?}", node.data);
            visited2.push(node);

            if node.data == NodeData::Int64(4) {
                break
            }
        }
        

        assert_eq!(visited.len(), visited2.len());


    }
}