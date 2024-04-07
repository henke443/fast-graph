#![crate_name = "fast_graph"]

//! # A fast, lightweight and extensible implementation of a graph data structure.
//!
//! ## Lightweight & fast.
//!
//! By default, [SlotMaps](`slotmap`) are used to store the nodes and edges which solves the [ABA problem] while also providing O(1) insertion, deletion and lookup times. Additionally, and optionally,
//! [HashBrown](hashbrown) is used instead of [`std::HashMap`] to map category names to ids in the [`CategorizedGraph`] struct.
//!
//! [ABA problem]: https://en.wikipedia.org/wiki/ABA_problem
//!
//! ## Extensible & Generic
//!
//! The [Graph] is generic over the node and edge data types, which can be any type that implements [Clone]. There's also traits for making even more customized graph-like data structures if the need arises.
//!
//! [`std::HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
//!
//! ## Serde & Specta
//!
//! There's optional features to enable [serde] & [specta] support.
//!
//! ## Categories
//!
//! The [CategorizedGraph] struct uses a hash map to map category names ([String]) to a category node ([NodeID]) (where the node's edges are the nodes belonging to the category).
//! There's also some useful extra functions to query categories and their nodes, and a [Categorized] trait that can be implemented for a custom struct if needed.
//!
//! In other words a simple extension to the graph that allows for efficient and easy grouping of nodes by strings.
//!
//! # Structure
//! [Node] - Struct representing a node in the graph. Contains a [NodeID] which is a key to the node in the slotmap, which has a generic data field and a list of edges.
//!
//! [Edge] - Struct representing an edge in the graph. Contains an [EdgeID] which is a key to the edge in the slotmap, and two [NodeID]s which are the nodes the edge connects (from & to). An edge can also have "data", which could be anything or nothing; for example the weight of the connection or a struct or enum representing something else.
//!
//! [GraphInterface] - Trait defining methods to alter a graph, i.e. adding, removing, and editing nodes and edges.
//!
//!
//! [Graph] - The default graph struct which implements [GraphInterface]. It only contains two slotmaps, one for nodes and one for edges.
//!
//! [Categorized] - Trait that extends the [Graph] with category specific methods.
//!
//! [CategorizedGraph] - A graph with categories. Categories are normal nodes (which can contain edges & data), but the graph also contains a hashmap that maps category names to category nodes for easy access.
//!
//!
//! # Examples
//!
//! ## Simple [Graph] and the ABA problem.
//!
//! ```
//! use fast_graph::{Graph, Node, Edge};
//! /* We need to have this trait in scope: */
//! use fast_graph::{GraphInterface};
//!
//! #[derive(Debug, Clone)]
//! struct EdgeData(String);
//!
//! #[derive(Debug, Clone)]
//! struct NodeData(String);
//!
//! let mut graph: Graph<NodeData, EdgeData> = Graph::new();
//!
//! let node1 = graph.add_node(NodeData("Node 1".into()));
//! let node2 = graph.add_node(NodeData("Node 2".into()));
//! let edge1 = graph.add_edge(node1, node2, EdgeData("Edge 1".into()));
//!
//! assert_eq!(graph.node(node1).unwrap().id, node1);
//! assert_eq!(graph.edge(edge1).unwrap().id, edge1);
//!
//! graph.remove_node(node1).unwrap();
//!
//! // Since we just removed node 1, it should be None now.
//! assert!(graph.node(node1).is_err());
//! // And node 2 still points to node 2.
//! assert_eq!(graph.node(node2).unwrap().id, node2);
//!
//! println!("{:#?}", graph);
//!
//! ```
//!
//! ## [CategorizedGraph] example
//! ```
//! use fast_graph::*;
//!
//! #[derive(Clone, Debug, Default, PartialEq)]
//! #[cfg_attr(feature = "serde", derive(serde::Serialize))]
//! enum NodeData {
//!     Number(u32),
//!     CategoryData(String),
//!     #[default]
//!     None,
//! }
//!
//! let mut graph: CategorizedGraph<NodeData, ()> = CategorizedGraph::new();
//!
//! let node1 = graph.add_node(NodeData::Number(1));
//! let node2 = graph.add_node(NodeData::Number(2));
//! let node3 = graph.add_node(NodeData::Number(3));
//!
//! let category1 = graph.create_category("Category 1", vec![node1, node2],
//!     NodeData::CategoryData("Category 1".into())
//! ).unwrap();
//!
//! assert_eq!(graph.category("Category 1").unwrap().connections.len(), 2);
//!
//! // The category node should have the same data as the one we passed in.
//! let category1_data = graph.category("Category 1").unwrap().data.clone();
//! if let NodeData::CategoryData(category1_name) = category1_data {
//!    assert_eq!(category1_name, "Category 1".to_string());
//! }
//!
//! // Adding to a category that doesn't exist will create it.
//! let category2 = graph.add_to_category("Category 2", vec![node2]);
//! assert_eq!(graph.all_categories().len(), 2);
//!
//! // Adding to the same category twice will return the same category node.
//! let category2_1 = graph.add_to_category("Category 2", vec![node3]);
//! assert_eq!(graph.all_categories().len(), 2);
//! assert_eq!(category2, category2_1);
//!
//! // The "Category 2" node should have two connections, one to node2 and one to node3.
//! let category2_node = graph.category("Category 2").unwrap();
//! assert_eq!(
//! // this:
//!     category2_node.connections.iter()
//!         .map(|edge_id|
//!             graph.edge(*edge_id).unwrap().to
//!         )
//!         .collect::<Vec<NodeID>>(),
//! // should equal:
//!     vec![node2, node3]
//! );
//!
//! // Creating a category twice will error.
//! assert!(
//!     graph.create_category("Category 1",
//!         vec![node3], NodeData::CategoryData("Category 1".into())
//!     ).is_err()
//! );
//! ```

#[cfg(feature = "specta")]
pub use specta_derives::*;

use core::fmt;
use std::fmt::Formatter;


pub use slotmap::SlotMap;
use thiserror::Error;

#[cfg(feature = "categories")]
pub mod categories;

#[cfg(feature = "categories")]
pub use categories::*;


pub mod algorithms;

mod edge;
mod interface;
mod node;
mod specta_derives;

pub use edge::{Edge, EdgeID};
pub use interface::GraphInterface;
pub use node::{Node, NodeID};

#[cfg(test)]
#[path = "./tests.rs"]
mod tests;

/* -------------------------------------------------------------------------- */
/*                 Simple very performant graph implementation                */
/* -------------------------------------------------------------------------- */

/* ---------------------------------- Graph --------------------------------- */
/// The default Graph struct which implements the [GraphInterface] trait.
///
///
/// # Examples
/// ```
/// use fast_graph::{Graph, Node, Edge};
/// /* We need to have this trait in scope: */
/// use fast_graph::{GraphInterface};
///
/// #[derive(Clone, Debug)]
/// struct EdgeData(String);
/// #[derive(Clone, Debug)]
/// struct NodeData(String);
///
/// let mut graph: Graph<NodeData, EdgeData> = Graph::new();
///
/// let node1 = graph.add_node(NodeData("Node 1".into()));
/// let node2 = graph.add_node(NodeData("Node 2".into()));
/// let edge1 = graph.add_edge(node1, node2, EdgeData("Edge 1".into()));
///
/// assert_eq!(graph.node(node1).unwrap().id, node1);
/// assert_eq!(graph.edge(edge1).unwrap().id, edge1);
///
/// graph.remove_node(node1).unwrap();
///
/// assert!(graph.node(node1).is_err());
/// assert_eq!(graph.node(node2).unwrap().id, node2);
///
/// println!("{:#?}", graph);
///
/// ```
pub struct Graph<N, E> {
    pub nodes: SlotMap<NodeID, Node<N>>,
    pub edges: SlotMap<EdgeID, Edge<E>>,
}

impl<N, E> GraphInterface for Graph<N, E> {
    type NodeData = N;
    type EdgeData = E;
    
    fn nodes(&self) -> impl Iterator<Item = NodeID> {
        self.nodes.keys()
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
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

    fn remove_node(&mut self, id: NodeID) -> Result<(), GraphError> {
        let node = self
            .nodes
            .remove(id)
            .map_or(Err(GraphError::NodeNotFound), |n| Ok(n))?;
        for edge_id in node.connections.iter() {
            self.edges
                .remove(*edge_id)
                .map_or(Err(GraphError::EdgeNotFound), |_| Ok(()))?;
        }
        Ok(())
    }

    fn remove_edge(&mut self, id: EdgeID) -> Result<(), GraphError> {
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
}

impl<N: fmt::Debug + Clone, E: fmt::Debug + Clone> fmt::Debug for Graph<N, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Graph {{ nodes: {:#?}, edges: {:#?} }}",
            self.nodes, self.edges
        )
    }
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Graph<N, E> {
        Graph {
            nodes: SlotMap::with_key(),
            edges: SlotMap::with_key(),
        }
    }
}

#[derive(Debug, Clone, Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GraphError {
    #[error("Edge not found")]
    EdgeNotFound,
    #[error("Node not found")]
    NodeNotFound,
}
