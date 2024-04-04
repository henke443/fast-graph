//! # Contains [SlotMapGraph]
//! [SlotMapGraph] provides the most basic functionality of a graph (getting mutable and immutable references to nodes and edges) using the [slotmap] crate.
//! Also implements the [GraphWriter] trait for any type that implements [SlotMapGraph].

use slotmap::SlotMap;

use crate::{Edge, EdgeID, GraphError, Node, NodeID};

pub use crate::GraphWriter;

/// Provides the most basic functionality of a graph (getting mutable and immutable references to nodes and edges) using the [slotmap] crate.
/// The [GraphWriter] trait is implemented for any type that implements [SlotMapGraph].
pub trait SlotMapGraph<N: Clone, E: Clone> {
    fn nodes(&self) -> &SlotMap<NodeID, Node<N>>;
    fn nodes_mut(&mut self) -> &mut SlotMap<NodeID, Node<N>>;
    fn node(&self, id: NodeID) -> Option<&Node<N>> {
        self.nodes().get(id)
    }
    fn node_mut(&mut self, id: NodeID) -> Option<&mut Node<N>> {
        self.nodes_mut().get_mut(id)
    }

    fn edges(&self) -> &SlotMap<EdgeID, Edge<E>>;
    fn edges_mut(&mut self) -> &mut SlotMap<EdgeID, Edge<E>>;
    fn edge(&self, id: EdgeID) -> Option<&Edge<E>> {
        self.edges().get(id)
    }
    fn edge_mut(&mut self, id: EdgeID) -> Option<&mut Edge<E>> {
        self.edges_mut().get_mut(id)
    }
}

impl<T: ?Sized + SlotMapGraph<N, E>, N: Clone, E: Clone> GraphWriter<N, E> for T {
    fn remove_node(&mut self, id: NodeID) -> Result<(), GraphError> {
        let node = self
            .nodes_mut()
            .remove(id)
            .map_or(Err(GraphError::NodeNotFound), |n| Ok(n))?;
        for edge_id in node.connections.iter() {
            self.edges_mut()
                .remove(*edge_id)
                .map_or(Err(GraphError::EdgeNotFound), |_| Ok(()))?;
        }
        Ok(())
    }

    fn remove_edge(&mut self, id: EdgeID) -> Result<(), GraphError> {
        self.edges_mut()
            .remove(id)
            .map_or(Err(GraphError::EdgeNotFound), |_| Ok(()))?;
        Ok(())
    }

    fn add_node(&mut self, data: N) -> &Node<N> {
        let id = self.nodes_mut().insert_with_key(|id| Node::new(id, data));
        &mut self.nodes().get(id).unwrap()
    }

    fn add_nodes(&mut self, data: &[N]) -> Vec<NodeID> {
        let mut nodes = Vec::new();
        for data in data {
            let node = self.add_node(data.clone());
            nodes.push(node.id);
        }
        nodes
    }

    fn add_edges(&mut self, data: &[(NodeID, NodeID)]) -> Vec<EdgeID>
    where
        E: Default,
    {
        let with_data: Vec<(NodeID, NodeID, E)> = data
            .iter()
            .map(|(from, to)| (*from, *to, E::default()))
            .collect();

        self.add_edges_with_data(&with_data)
    }

    fn add_edge(&mut self, from: NodeID, to: NodeID, data: E) -> &mut Edge<E> {
        let id = self
            .edges_mut()
            .insert_with_key(|id| Edge::new(id, from, to, data));
        if let Some(node) = self.nodes_mut().get_mut(from) {
            node.add_connection(id);
        }
        if let Some(node) = self.nodes_mut().get_mut(to) {
            node.add_connection(id);
        }
        self.edges_mut().get_mut(id).unwrap()
    }
}
