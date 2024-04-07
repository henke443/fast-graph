//! # A graph with category nodes.
//!
//! The [CategorizedGraph] struct uses a hash map to map category names ([String]) to a category node ([NodeID]) (where the node's edges are the nodes belonging to the category).
//! There's also some useful extra functions to query categories and their nodes, and a [Categorized] trait that can be implemented for a custom struct if needed.
//!
//! In other words a simple extension to the graph that allows for efficient and easy grouping of nodes by strings.
//!
//! # Example
//! ```
//! use fast_graph::*;
//!
//! #[derive(Clone, Debug, Default, PartialEq)]
//! #[cfg_attr(feature = "serde", derive(serde::Serialize))]
//! enum NodeData {
//!     String(String),
//!     CategoryName(String),
//!     #[default]
//!     None,
//! }
//!
//! let mut graph: CategorizedGraph<NodeData, ()> = CategorizedGraph::new();
//!
//! let node1 = graph.add_node(NodeData::String("Node 1".into()));
//! let node2 = graph.add_node(NodeData::String("Node 2".into()));
//! let node3 = graph.add_node(NodeData::String("Node 3".into()));
//!
//! let category1 = graph.create_category("Category 1", vec![node1, node2], NodeData::CategoryName("Category 1".into())).unwrap();
//! let category2 = graph.add_to_category("Category 2", vec![node3]);
//!
//! assert_eq!(graph.all_categories().len(), 2);
//! ```

use crate::*;
#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;
#[cfg(feature = "hashbrown")]
use hashbrown::HashSet;

#[cfg(not(feature = "hashbrown"))]
use std::collections::HashMap;
#[cfg(not(feature = "hashbrown"))]
use std::collections::HashSet;

use thiserror::Error;

/// A graph with category nodes (where the nodes contain an ID of the category and a list of nodes in that category) and a hash map that maps category names to category nodes efficiently.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CategorizedGraph<N, E> {
    pub nodes: SlotMap<NodeID, Node<N>>,
    pub edges: SlotMap<EdgeID, Edge<E>>,
    pub categories: HashMap<String, NodeID>,
}

impl<N, E> GraphInterface for CategorizedGraph<N, E> {
    type NodeData = N;
    type EdgeData = E;

    fn nodes(&self) -> impl Iterator<Item = NodeID> {
        self.nodes.keys()
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn remove_node(&mut self, id: NodeID) -> Result<(), GraphError> {
        let node = self
            .nodes
            .remove(id)
            .map_or(Err(GraphError::NodeNotFound), |n| Ok(n))?;

        for edge_id in node.connections.iter() {
            self.remove_edge(*edge_id).or_else(|e| Ok(()))?;
        }

        Ok(())
    }

    fn remove_edge(&mut self, id: EdgeID) -> Result<(), GraphError> {
        let edge = self.edge(id)?;
        let from = edge.from;
        let to = edge.to;

        if let Ok(node) = self.node_mut(from) {
            node.connections.retain(|&x| x != id)
        }

        if let Ok(node) = self.node_mut(to) {
            node.connections.retain(|&x| x != id)
        }

        self.edges
            .remove(id)
            .map_or(Err(GraphError::EdgeNotFound), |_| Ok(()))?;

        Ok(())
    }

    fn add_node(&mut self, data: N) -> NodeID {
        let id = self.nodes.insert_with_key(|id| Node::new(id, data));
        id
    }

    fn add_nodes(&mut self, data: &[N]) -> Vec<NodeID>
    where
        N: Clone,
    {
        let mut nodes = Vec::new();
        for data in data {
            let node = self.add_node(data.clone());
            nodes.push(node);
        }
        nodes
    }

    fn add_edges(&mut self, data: &[(NodeID, NodeID)]) -> Vec<EdgeID>
    where
        E: Default + Clone,
        N: Clone,
    {
        let with_data: Vec<(NodeID, NodeID, E)> = data
            .iter()
            .map(|(from, to)| (*from, *to, E::default()))
            .collect();

        self.add_edges_with_data(&with_data)
    }

    fn add_edge(&mut self, from: NodeID, to: NodeID, data: E) -> EdgeID {
        let id = self
            .edges
            .insert_with_key(|id| Edge::new(id, from, to, data));
        if let Some(node) = self.nodes.get_mut(from) {
            node.add_connection(id);
        }
        if let Some(node) = self.nodes.get_mut(to) {
            node.add_connection(id);
        }
        id
    }

    fn node(&self, id: NodeID) -> Result<&Node<N>, GraphError> {
        self.nodes.get(id).ok_or(GraphError::NodeNotFound)
    }

    fn node_mut(&mut self, id: NodeID) -> Result<&mut Node<N>, GraphError> {
        self.nodes.get_mut(id).ok_or(GraphError::NodeNotFound)
    }

    fn edge(&self, id: EdgeID) -> Result<&Edge<E>, GraphError> {
        self.edges.get(id).ok_or(GraphError::EdgeNotFound)
    }

    fn edge_mut(&mut self, id: EdgeID) -> Result<&mut Edge<E>, GraphError> {
        self.edges.get_mut(id).ok_or(GraphError::EdgeNotFound)
    }
}

impl<N, E> CategorizedGraph<N, E> {
    pub fn new() -> Self {
        CategorizedGraph {
            edges: SlotMap::with_key(),
            nodes: SlotMap::with_key(),
            categories: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CategorizedGraphError {
    #[error("Category identified by `{0}` already exists")]
    CategoryAlreadyExists(String),
    #[error("Category identified by `{0}` does not exists")]
    CategoryNotFound(String),
}

/// Methods for a graph with categories.
pub trait Categorized<N, E, C>: GraphInterface<NodeData = N, EdgeData = E> {
    /// Returns the category ID by name. In the standard implementation this is a hashmap lookup.
    fn category_id_by_name(&self, category_name: &str) -> Option<&NodeID>;

    /// Checks if the category exists by name.
    fn category_exists(&self, category_name: &str) -> bool {
        self.category_id_by_name(category_name).is_some()
    }

    /// Adds a list of nodes to a category by ID. Returns `Ok(())` if successful, otherwise returns Error([CategorizedGraphError::CategoryNotFound]).
    fn add_to_category_by_id(
        &mut self,
        category_id: NodeID,
        nodes: Vec<NodeID>,
    ) -> Result<(), CategorizedGraphError>
    where
        E: Default + Clone,
        N: Clone,
    {
        let category_node = self.node(category_id).map_or(
            Err(CategorizedGraphError::CategoryNotFound(format!(
                "NodeID({:?})",
                category_id
            ))),
            |node| Ok(node),
        )?;

        let edges: Vec<(NodeID, NodeID)> = nodes
            .iter()
            .map(|node: &NodeID| (category_node.id, *node))
            .collect();

        self.add_edges(&edges);

        Ok(())
    }

    /// In the default implementation this is used to insert the category ID into the hashmap.
    fn insert_category_id_by_name(&mut self, category_name: &str, category_id: NodeID) {
        // Default implementation (optional logic)
        // You can leave this empty or provide some default behavior
    }

    /// If the category does not exist, it is created. Returns the [NodeID] of the category.
    fn add_to_category(&mut self, category_name: &str, nodes: Vec<NodeID>) -> NodeID
    where
        E: Default + Clone,
        N: Clone + Default,
    {
        let existing: Option<&NodeID> = self.category_id_by_name(category_name);
        let category_node: NodeID;

        if existing.is_some() {
            category_node = *existing.unwrap();
            self.add_to_category_by_id(category_node, nodes).unwrap();
        } else {
            category_node = self.add_node(N::default());
            self.add_to_category_by_id(category_node, nodes).unwrap();
            self.insert_category_id_by_name(category_name, category_node)
        }

        category_node
    }

    /// Creates a new category [Node] with the given name, nodes, and (optionally) data.
    ///
    /// Returns the [NodeID] of the category if successful, otherwise returns Error(CategorizedGraphError::CategoryAlreadyExists).
    ///
    /// An empty vector of nodes can be passed.
    fn create_category(
        &mut self,
        category: &str,
        nodes: Vec<NodeID>,
        data: C,
    ) -> Result<NodeID, String>
    where
        E: Default + Clone,
        N: Clone + Default;

    /// Returns a list of all categories.
    fn all_categories(&self) -> Vec<(&String, NodeID)>;

    /// Returns the category node by name.
    fn category(&self, category: &str) -> Option<&Node<N>>;

    /// Checks if the category exists by ID.
    fn category_exists_by_id(&self, category: NodeID) -> bool {
        self.category_by_id(category).is_ok()
    }

    /// Returns the category node by ID.
    fn category_by_id(&self, category: NodeID) -> Result<&Node<N>, GraphError>;

    /// Returns a list of nodes in the category by ID.
    fn nodes_by_category_id(&self, category: NodeID) -> Vec<NodeID>;

    /// Returns a list of nodes in the category by name.
    fn nodes_by_category(&self, category: &str) -> Vec<NodeID>;

    /// Returns a list of nodes in the categories by name.
    fn nodes_by_categories(&self, categories: Vec<&str>) -> Vec<NodeID> {
        categories
            .iter()
            .map(|category| self.nodes_by_category(category))
            .flatten()
            .collect()
    }

    /// Returns a list of nodes in the categories by ID.
    fn nodes_by_category_ids(&self, categories: Vec<NodeID>) -> Vec<NodeID> {
        categories
            .iter()
            .map(|category| self.nodes_by_category_id(*category))
            .flatten()
            .collect()
    }
}

impl<N, E> Categorized<N, E, N> for CategorizedGraph<N, E>
where
    Self: GraphInterface<NodeData = N, EdgeData = E>,
{
    fn category_id_by_name(&self, category_name: &str) -> Option<&NodeID> {
        self.categories.get(category_name)
    }

    fn insert_category_id_by_name(&mut self, category_name: &str, category_id: NodeID) {
        self.categories
            .insert(category_name.to_string(), category_id);
    }

    fn create_category(
        &mut self,
        category: &str,
        nodes: Vec<NodeID>,
        data: N,
    ) -> Result<NodeID, String>
    where
        E: Default + Clone,
        N: Clone + Default,
    {
        let existing_category: Option<&NodeID> = self.categories.get(category);
        if existing_category.is_some() {
            return Err(format!("Category {} already exists", category));
        }
        let category_node = self.add_node(data);
        self.add_to_category(category, nodes);
        Ok(category_node)
    }

    fn all_categories(&self) -> Vec<(&String, NodeID)> {
        self.categories
            .iter()
            .map(|(cat, node)| (cat, *node))
            .collect()
    }

    fn category(&self, category: &str) -> Option<&Node<N>> {
        self.categories
            .get(category)
            .map(|id| self.node(*id).unwrap())
    }

    fn category_by_id(&self, category: NodeID) -> Result<&Node<N>, GraphError> {
        self.node(category)
    }

    fn nodes_by_category_id(&self, category: NodeID) -> Vec<NodeID> {
        self.node(category)
            .and_then(|category_node| {
                category_node
                    .connections
                    .iter()
                    .filter_map(|edge_id| self.edge(*edge_id).map_or(None, |edge| Some(edge)))
                    .map(|edge| Ok(edge.to))
                    .collect()
            })
            .unwrap_or(Vec::new())
    }

    fn nodes_by_category(&self, category: &str) -> Vec<NodeID> {
        self.categories
            .get(category)
            .map(|id| self.nodes_by_category_id(*id))
            .unwrap_or(Vec::new())
    }
}
