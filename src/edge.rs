//! # A struct representing an edge in the graph.
//!
//! Contains an [EdgeID] which is a key to the edge in the slotmap, and two [NodeID]s which are the nodes the edge connects (from & to).
//!
//! An edge can also have “data”, which could be anything or nothing; for example the weight of the connection or a struct or enum representing something else.
//!
//! # Why is there no "EdgeTrait"?
//!
//! The [Edge] struct is very simple and doesn't need a trait. It's just a struct with an ID, two node IDs, and some data.
//! If you want to add more functionality or data to the edge you can probably just add it to the data field, or add an edge as a field to your custom type.

use slotmap::{new_key_type, KeyData};

use super::*;

new_key_type! {
    /// An index to an edge in the slotmap
    pub struct EdgeID;
}
impl EdgeID {
    pub fn to_u64(&self) -> u64 {
        self.0.as_ffi()
    }
    pub fn from_u64(id: u64) -> Self {
        EdgeID::from(KeyData::from_ffi(id))
    }
}

/// # A struct representing an edge in the graph.
///
/// Contains an [EdgeID] which is a key to the edge in the slotmap, and two [NodeID]s which are the nodes the edge connects (from & to).
/// An edge can also have “data”, which could be anything or nothing; for example the weight of the connection or a struct or enum representing something else.
///
/// ## Why is there no "EdgeTrait"?
///
/// The [Edge] struct is very simple and doesn't need a trait. It's just a struct with an ID, two node IDs, and some data.
/// If you want to add more functionality or data to the edge you can probably just add it to the data field, or add an edge as a field to your custom type.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Edge<T: Clone> {
    pub id: EdgeID,
    pub from: NodeID,
    pub to: NodeID,
    pub data: T,
}

/// Implements Hash for Edge<T> so only the ID is used for hashing.
impl<T: std::hash::Hash> std::hash::Hash for Edge<T>
where
    T: Clone,
{
    fn hash<H: std::hash::Hasher>(&self, ra_expand_state: &mut H) {
        self.id.hash(ra_expand_state);
    }
}


/// Implements PartialEq for Edge<T> so only the ID is used for comparison.
impl<T: Clone> PartialEq for Edge<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T: Clone + fmt::Debug> fmt::Debug for Edge<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Edge {{ id: {:#?}, from: {:#?}, to: {:#?}, data: {:#?} }}",
            self.id, self.from, self.to, self.data
        )
    }
}

impl<T: Clone> Edge<T> {
    pub fn new(id: EdgeID, from: NodeID, to: NodeID, data: T) -> Edge<T> {
        Edge { id, from, to, data }
    }
}

