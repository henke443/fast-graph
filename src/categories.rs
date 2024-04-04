//! # A graph with category nodes.
//!
//! The [CategoryGraph] struct uses a hash map to map category names ([String]) to a category node ([NodeID]) (where the node's edges are the nodes belonging to the category).
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
//! let mut graph: CategoryGraph<NodeData, ()> = CategoryGraph::new();
//!
//! let node1 = graph.add_node(NodeData::String("Node 1".into())).id;
//! let node2 = graph.add_node(NodeData::String("Node 2".into())).id;
//! let node3 = graph.add_node(NodeData::String("Node 3".into())).id;
//!
//! let category1 = graph.create_category("Category 1", vec![node1, node2], NodeData::CategoryName("Category 1".into())).unwrap();
//! let category2 = graph.add_to_category("Category 2", vec![node3]);
//!
//! assert_eq!(graph.all_categories().len(), 2);
//! ```

use crate::*;
#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;
#[cfg(not(feature = "hashbrown"))]
use std::collections::HashMap;
use thiserror::Error;

/// A graph with category nodes (where the nodes contain an ID of the category and a list of nodes in that category) and a hash map that maps category names to category nodes efficiently.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CategoryGraph<N: Clone, E: Clone> {
    pub nodes: SlotMap<NodeID, Node<N>>,
    pub edges: SlotMap<EdgeID, Edge<E>>,
    pub categories: HashMap<String, NodeID>,
}

impl<N: Clone, E: Clone> SlotMapGraph<N, E> for CategoryGraph<N, E> {
    fn nodes(&self) -> &SlotMap<NodeID, Node<N>> {
        &self.nodes
    }

    fn nodes_mut(&mut self) -> &mut SlotMap<NodeID, Node<N>> {
        &mut self.nodes
    }

    fn edges(&self) -> &SlotMap<EdgeID, Edge<E>> {
        &self.edges
    }

    fn edges_mut(&mut self) -> &mut SlotMap<EdgeID, Edge<E>> {
        &mut self.edges
    }
}

impl<N: Clone, E: Clone> CategoryGraph<N, E> {
    pub fn new() -> Self {
        CategoryGraph {
            edges: SlotMap::with_key(),
            nodes: SlotMap::with_key(),
            categories: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CategoryGraphError {
    #[error("Category identified by `{0}` already exists")]
    CategoryAlreadyExists(String),
    #[error("Category identified by `{0}` does not exists")]
    CategoryNotFound(String),
}

/// Methods for a graph with categories.
pub trait Categorized<N: Clone + Default, E: Clone + Default, C: Clone>:
    SlotMapGraph<N, E>
{
    /// Returns the category ID by name. In the standard implementation this is a hashmap lookup.
    fn category_id_by_name(&self, category_name: &str) -> Option<&NodeID>;

    /// Checks if the category exists by name.
    fn category_exists(&self, category_name: &str) -> bool {
        self.category_id_by_name(category_name).is_some()
    }

    /// Adds a list of nodes to a category by ID. Returns `Ok(())` if successful, otherwise returns Error([CategoryGraphError::CategoryNotFound]).
    fn add_to_category_by_id(
        &mut self,
        category_id: NodeID,
        nodes: Vec<NodeID>,
    ) -> Result<(), CategoryGraphError> {
        let category_node = self.node(category_id).map_or(
            Err(CategoryGraphError::CategoryNotFound(format!(
                "NodeID({:?})",
                category_id
            ))),
            |node| Ok(node),
        )?;

        let edges: Vec<(NodeID, NodeID)> = nodes
            .iter()
            .map(|node: &NodeID| (category_node.id, node.clone()))
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
    fn add_to_category(&mut self, category_name: &str, nodes: Vec<NodeID>) -> NodeID {
        let existing: Option<&NodeID> = self.category_id_by_name(category_name);
        let category_node: NodeID;

        if existing.is_some() {
            category_node = *existing.unwrap();
            self.add_to_category_by_id(category_node, nodes).unwrap();
        } else {
            category_node = self.add_node(N::default()).id;
            self.add_to_category_by_id(category_node, nodes).unwrap();
            self.insert_category_id_by_name(category_name, category_node)
        }

        category_node
    }

    /// Creates a new category [Node] with the given name, nodes, and (optionally) data.
    ///
    /// Returns the [NodeID] of the category if successful, otherwise returns Error(CategoryGraphError::CategoryAlreadyExists).
    ///
    /// An empty vector of nodes can be passed.
    fn create_category(
        &mut self,
        category: &str,
        nodes: Vec<NodeID>,
        data: C,
    ) -> Result<NodeID, String>;

    /// Returns a list of all categories.
    fn all_categories(&self) -> Vec<(&String, NodeID)>;

    /// Returns the category node by name.
    fn category(&self, category: &str) -> Option<&Node<N>>;

    /// Checks if the category exists by ID.
    fn category_exists_by_id(&self, category: NodeID) -> bool {
        self.category_by_id(category).is_some()
    }

    /// Returns the category node by ID.
    fn category_by_id(&self, category: NodeID) -> Option<&Node<N>>;

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

impl<N: Clone + Default, E: Clone + Default> Categorized<N, E, N> for CategoryGraph<N, E>
where
    Self: GraphWriter<N, E>,
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
    ) -> Result<NodeID, String> {
        let existing_category: Option<&NodeID> = self.categories.get(category);
        if existing_category.is_some() {
            return Err(format!("Category {} already exists", category));
        }
        let category_node = self.add_node(data).id;
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
            .map(|id| SlotMapGraph::node(self, *id).unwrap())
    }

    fn category_by_id(&self, category: NodeID) -> Option<&Node<N>> {
        SlotMapGraph::node(self, category)
    }

    fn nodes_by_category_id(&self, category: NodeID) -> Vec<NodeID> {
        SlotMapGraph::node(self, category)
            .and_then(|category_node| {
                category_node
                    .connections
                    .iter()
                    .filter_map(|edge_id| SlotMapGraph::edge(self, *edge_id))
                    .map(|edge| Some(edge.to.clone()))
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
