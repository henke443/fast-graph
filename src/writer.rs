//! # Contains [GraphWriter]
//! GraphWriter is a trait for basic "write" operations on a graph; core operations needed to change a graph and some derived helper functions.

use crate::{Edge, EdgeID, GraphError, Node, NodeID};

/// GraphWriter is a trait for basic "write" operations on a graph; core operations needed to change a graph and some derived helper functions.
pub trait GraphWriter<N: Clone, E: Clone> {
    fn add_node(&mut self, data: N) -> &Node<N>;
    fn add_nodes(&mut self, data: &[N]) -> Vec<NodeID>;

    fn add_edge(&mut self, from: NodeID, to: NodeID, data: E) -> &mut Edge<E>;

    fn remove_node(&mut self, id: NodeID) -> Result<(), GraphError>;
    fn remove_edge(&mut self, id: EdgeID) -> Result<(), GraphError>;

    fn add_edges(&mut self, data: &[(NodeID, NodeID)]) -> Vec<EdgeID>
    where
        E: Default;

    fn remove_nodes(&mut self, ids: &[NodeID]) -> Result<(), GraphError> {
        for id in ids {
            self.remove_node(*id)?;
        }
        Ok(())
    }

    fn add_edges_with_data(&mut self, data: &[(NodeID, NodeID, E)]) -> Vec<EdgeID> {
        let mut edges = Vec::new();
        for (from, to, data) in data {
            let edge: &Edge<E> = self.add_edge(*from, *to, data.clone());
            edges.push(edge.id);
        }
        edges
    }

    fn add_nodes_and_edges(&mut self, data: Vec<(N, Vec<NodeID>)>) -> (Vec<NodeID>, Vec<EdgeID>)
    where
        E: Default,
    {
        let with_data = data
            .iter()
            .map(|(data, edges)| {
                (
                    data.clone(),
                    edges.iter().map(|id| (*id, E::default())).collect(),
                )
            })
            .collect();
        self.add_nodes_and_edges_with_data(with_data)
    }

    fn add_nodes_and_edges_with_data(
        &mut self,
        node_data: Vec<(N, Vec<(NodeID, E)>)>,
    ) -> (Vec<NodeID>, Vec<EdgeID>) {
        let mut added_nodes = Vec::new();
        let mut added_edges: Vec<EdgeID> = Vec::new();
        for (data, connections) in node_data {
            let node_id = {
                let node = self.add_node(data);
                added_nodes.push(node.id);
                node.id
            };
            let edges: Vec<(NodeID, NodeID, E)> = connections
                .iter()
                .map(|(to, edge_data)| (node_id, *to, edge_data.clone()))
                .collect();
            added_edges = self.add_edges_with_data(&edges);
        }
        (added_nodes, added_edges)
    }
}
