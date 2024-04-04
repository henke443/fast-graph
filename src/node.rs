//! # [Node] is a struct representing a node in the graph,
//! --- which has a generic data field and a list of [EdgeID]s.
//!
//! A [NodeID] is a key to the node in the slotmap.
//!
//! # Why is there no "NodeTrait"?
//!
//! The [Node] struct is very simple and doesn't need a trait.
//! If you want to add more functionality or data to the Node you can probably just add it to the data field, or add a node as a field to your custom type.

use super::*;
use slotmap::{new_key_type, KeyData};

new_key_type! {
    /// A key to the node in the slotmap.
    pub struct NodeID;
}

impl NodeID {
    pub fn to_u64(&self) -> u64 {
        self.0.as_ffi()
    }
    pub fn from_u64(id: u64) -> Self {
        NodeID::from(KeyData::from_ffi(id))
    }
}

/* -------------------------------------------------------------------------- */
/*                                    Node                                    */
/* -------------------------------------------------------------------------- */

/// # A struct representing a node/vertex in the graph.
/// Has a generic data field and a list of [EdgeID]s.
///
/// A [NodeID] is a key to the node in the slotmap.
///
/// ## Why is there no "NodeTrait"?
///
/// The [Node] struct is very simple and doesn't need a trait.
/// If you want to add more functionality or data to the Node you can probably just add it to the data field, or add a node as a field to your custom type.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Node<T> {
    pub id: NodeID,
    pub data: T,
    pub connections: Vec<EdgeID>,
}

/// Implements PartialEQ for Node<T> so only the ID is used for comparison.
impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// Implements Hash for Node<T> so only the ID is used for hashing.
impl<T: std::hash::Hash> std::hash::Hash for Node<T> {
    fn hash<H: std::hash::Hasher>(&self, ra_expand_state: &mut H) {
        self.id.hash(ra_expand_state);
    }
}

/* ---------------------------------- Debug --------------------------------- */

impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node {{ id: {:#?}, data: {:#?}, connections: {:#?} }}",
            self.id, self.data, self.connections
        )
    }
}

impl<T> Node<T> {
    pub fn new(id: NodeID, data: T) -> Node<T> {
        Node {
            id,
            data,
            connections: Vec::new(),
        }
    }

    pub fn add_connection(&mut self, edge: EdgeID) {
        self.connections.push(edge);
    }
}
