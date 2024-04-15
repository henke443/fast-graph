#[cfg(feature = "hashbrown")]
use hashbrown::HashSet;
#[cfg(not(feature = "hashbrown"))]
use std::collections::HashSet;



use crate::{GraphInterface, NodeID};
use crate::Edge;

/// Under development
#[derive(Clone)]
pub struct DepthFirstSearch<'a, G: GraphInterface> {
    graph: &'a G,
    visited: HashSet<NodeID>,
    stack: Vec<NodeID>,
    cyclic: bool,
}

impl<'a, G: GraphInterface> DepthFirstSearch<'a, G> {
    pub fn new(graph: &'a G, start: NodeID) -> Self {
        Self {
            graph,
            visited: HashSet::new(),
            stack: vec![start],
            cyclic: false,
        }
    }
}


impl<'a, G: GraphInterface> Iterator for DepthFirstSearch<'a, G> {
    type Item = NodeID;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node_id) = self.stack.pop() {
            if !self.visited.contains(&node_id) {
                self.visited.insert(node_id);
                let node = self.graph.node(node_id).unwrap();
                let connections = &node.connections;
                for edge_id in connections.iter().rev() {
                    let to_id = self.graph.edge(*edge_id).unwrap().to;
                    if !self.visited.contains(&to_id) {
                        self.stack.push(to_id);
                    }
                }
                return Some(node_id);
            } else {
                self.cyclic = true;
            }
        }
        None
        // if let Some(node) = self.stack.pop() {
        //     if self.visited.contains(&node) {
        //         self.cyclic = true;
        //         return self.next();
        //     }
        //     self.visited.insert(node);

        //     let node = self.graph.node(node);
        //     if node.is_err() {
        //         return self.next();
        //     }
        //     let node = node.unwrap();
        //     for edge in node.connections.iter().rev() {
        //         let edge = self.graph.edge(*edge).unwrap();
        //         if !self.visited.contains(&edge.to) {
        //             self.stack.push(edge.to);
        //         }
        //     }

        //     return Some(node.id);
        // }
        // None
    }
}

// impl<'a, G: GraphInterface> std::iter::FusedIterator for DepthFirstSearch<'a, G> {}

/// Under development
pub trait IterDepthFirst<'a, G: GraphInterface> {
    /// Returns a *depth first search* iterator starting from a given node
    fn iter_depth_first(&'a self, start: NodeID) -> DepthFirstSearch<'a, G>;

    /// Returns a vector of sets of node IDs, where each set is a connected component. \
    /// Starts a DFS at every node (except if it's already been visited) and marks all reachable nodes as being part of the same component.
    fn connected_components(&'a self) -> Vec<HashSet<NodeID>>;
}

impl<'a, G: GraphInterface> IterDepthFirst<'a, G> for G {
    fn iter_depth_first(&'a self, start: NodeID) -> DepthFirstSearch<'a, G> {
        DepthFirstSearch::new(self, start)
    }

    /// Returns a vector of sets of node IDs, where each set is a connected component. \
    /// Starts a DFS at every node (except if it's already been visited) and marks all reachable nodes as being part of the same component.
    fn connected_components(&'a self) -> Vec<HashSet<NodeID>> {
        let mut visited = HashSet::new();
        let mut components = Vec::new();
        let mut current_component = 0usize;

        // Starts a DFS at every node
        for node_id in self.nodes() {
            // (except if it's already been visited)
            if visited.contains(&node_id) {
                continue;
            }
            for node in self.iter_depth_first(node_id) {
                visited.insert(node);

                // and marks all reachable nodes as being part of the same component.
                if current_component >= components.len() {
                    components.push(HashSet::new());
                }
                components[current_component].insert(node);
            }
            current_component += 1;
        }

        components
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Graph;

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

    macro_rules! get_graph {
        ($graph:ident, $n:expr) => {{
            let mut nodes = Vec::new();
            for i in 0..$n {
                nodes.push(NodeData::Int64(i));
            }
            let nodes = $graph.add_nodes(&nodes);
            if nodes.len() != $n {
                panic!("Failed to add nodes");
            }
            nodes[..].try_into().unwrap()
        }};
    }

    #[test]
    fn test_dfs_connected_components() {
        let mut graph: Graph<NodeData, ()> = Graph::new();
        let [node0, node1, node2, node3, node4] = get_graph!(graph, 5);

        let mut components = graph.connected_components();
        println!(
            "Connected components 1 ({}): {:#?}",
            components.len(),
            components
        );
        assert_eq!(components.len(), 5);
        assert_eq!(components[0].len(), 1);

        graph.add_edges(&[(node0, node1), (node1, node0)]);

        components = graph.connected_components();
        println!(
            "Connected components 2 ({}): {:#?}",
            components.len(),
            components
        );
        assert_eq!(components.len(), 4);
        assert_eq!(components[0].len(), 2);

        graph.add_edges(&[(node2, node3), (node3, node4)]);

        components = graph.connected_components();
        println!(
            "Connected components 3 ({}): {:#?}",
            components.len(),
            components
        );

        assert_eq!(components.len(), 2);
        assert_eq!(components[1].len(), 3);
    }

    #[test]
    fn test_dfs_iter() {
        let mut graph1: Graph<NodeData, ()> = Graph::new();
        let [node0, node1, node2, node3, node4] = get_graph!(graph1, 5);

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
        let [node0_1, node1_1, node2_1, node3_1, node4_1] = get_graph!(graph2, 5);

        graph2.add_edges(&[
            (node0_1, node3_1),
            (node0_1, node2_1),
            (node1_1, node0_1),
            (node2_1, node3_1),
            (node4_1, node2_1),
        ]);

        let mut visited = Vec::new();
        let depth_first = graph1.iter_depth_first(node0);
        for node in depth_first {
            let node = graph1.node(node).unwrap();
            visited.push(node);
        }

        assert_eq!(visited.len(), graph1.node_count());
        assert_eq!(visited.len(), 5);

        println!(
            "Depth First Search 2 (node count: {}):",
            graph1.node_count()
        );
        println!("Edges: {:#?}", graph1.edges.len());

        let mut visited = Vec::new();
        for node in graph1.iter_depth_first(node0) {
            let node = graph1.node(node).unwrap();
            //println!("{:?}", node.data);
            visited.push(node);

            if node.data == NodeData::Int64(4) {
                break;
            }
        }

        assert_eq!(visited.len(), graph1.node_count());
        assert_eq!(visited.len(), 5);

        println!(
            "Depth First Search 3 (node count: {}):",
            graph2.node_count()
        );
        println!("Edges: {:#?}", graph2.edges.len());
        let mut visited2 = Vec::new();
        for node in graph2.iter_depth_first(node0_1) {
            let node = graph2.node(node).unwrap();
            //println!("{:?}", node.data);
            visited2.push(node);

            if node.data == NodeData::Int64(4) {
                break;
            }
        }

        assert_ne!(visited.len(), visited2.len());
    }
}
