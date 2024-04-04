//! # Contains [GraphInterface]
//! GraphInterface is a trait for basic "read and write" operations on a graph; core operations needed to change a graph and some derived helper functions.

use crate::{Edge, EdgeID, GraphError, Node, NodeID};

/// GraphInterface is a trait for basic "read and write" operations on a graph; core operations needed to change a graph and some derived helper functions.
pub trait GraphInterface {
    type NodeData;
    type EdgeData;
    
    fn node_count(&self) -> usize;
    
    fn node(&self, id: NodeID) -> Result<&Node<Self::NodeData>, GraphError>;
    fn node_mut(&mut self, id: NodeID) -> Result<&mut Node<Self::NodeData>, GraphError>;

    fn edge(&self, id: EdgeID) -> Result<&Edge<Self::EdgeData>, GraphError>;
    fn edge_mut(&mut self, id: EdgeID) -> Result<&mut Edge<Self::EdgeData>, GraphError>;

    fn add_node(&mut self, data: Self::NodeData) -> NodeID;
    fn add_nodes(&mut self, data: &[Self::NodeData]) -> Vec<NodeID>
    where
        Self::NodeData: Clone;

    fn add_edge(&mut self, from: NodeID, to: NodeID, data: Self::EdgeData) -> EdgeID;

    fn remove_node(&mut self, id: NodeID) -> Result<(), GraphError>;
    fn remove_edge(&mut self, id: EdgeID) -> Result<(), GraphError>;

    fn add_edges(&mut self, data: &[(NodeID, NodeID)]) -> Vec<EdgeID>
    where
        Self::EdgeData: Default + Clone,
        Self::NodeData: Clone;

    fn remove_nodes(&mut self, ids: &[NodeID]) -> Result<(), GraphError> {
        for id in ids {
            self.remove_node(*id)?;
        }
        Ok(())
    }

    fn add_edges_with_data(&mut self, data: &[(NodeID, NodeID, Self::EdgeData)]) -> Vec<EdgeID>
    where
        Self::EdgeData: Clone,
    {
        let mut edges = Vec::new();
        for (from, to, data) in data {
            let edge = self.add_edge(*from, *to, data.clone());
            edges.push(edge);
        }
        edges
    }

    fn add_nodes_and_edges(&mut self, data: Vec<(Self::NodeData, Vec<NodeID>)>) -> (Vec<NodeID>, Vec<EdgeID>)
    where
        Self::EdgeData: Default + Clone,
        Self::NodeData: Default + Clone,
    {
        let with_data: Vec<(Self::NodeData, Vec<(NodeID, Self::EdgeData)>)> = data
            .iter()
            .map(|(data, edges)| {
                (
                    data.clone(),
                    edges.iter().map(|id| (*id, Self::EdgeData::default())).collect(),
                )
            })
            .collect();
        self.add_nodes_and_edges_with_data(with_data)
    }

    fn add_nodes_and_edges_with_data(
        &mut self,
        node_data: Vec<(Self::NodeData, Vec<(NodeID, Self::EdgeData)>)>,
    ) -> (Vec<NodeID>, Vec<EdgeID>)
    where
        Self::NodeData: Default + Clone,
        Self::EdgeData: Clone,
    {
        let mut added_nodes = Vec::new();
        let mut added_edges: Vec<EdgeID> = Vec::new();
        for (data, connections) in node_data {
            let node_id = {
                let node = self.add_node(data);
                added_nodes.push(node);
                node
            };
            let edges: Vec<(NodeID, NodeID, Self::EdgeData)> = connections
                .iter()
                .map(|(to, edge_data)| (node_id, *to, edge_data.clone()))
                .collect();
            added_edges = self.add_edges_with_data(&edges);
        }
        (added_nodes, added_edges)
    }
}
