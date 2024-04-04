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

#[cfg(feature = "specta")]
const _: () = {
    const SID: specta::SpectaID = specta::internal::construct::sid(
        "EdgeID",
        concat!("::", module_path!(), ":", line!(), ":", column!()),
    );
    const IMPL_LOCATION: specta::ImplLocation =
        specta::internal::construct::impl_location(concat!(file!(), ":", line!(), ":", column!()));
    const DEFINITION_GENERICS: &[specta::DataType] = &[];

    #[automatically_derived]
    impl specta::Type for EdgeID {
        fn inline(
            type_map: &mut specta::TypeMap,
            generics: &[specta::DataType],
        ) -> specta::DataType {
            specta::DataType::Struct(specta::internal::construct::r#struct(
                "EdgeID".into(),
                Some(SID),
                vec![],
                specta::internal::construct::struct_named(
                    vec![
                        (
                            "idx".into(),
                            specta::internal::construct::field(
                                false,
                                false,
                                None,
                                "".into(),
                                Some({
                                    let ty = <u32 as specta::Type>::reference(type_map, &[]).inner;
                                    ty
                                }),
                            ),
                        ),
                        (
                            "version".into(),
                            specta::internal::construct::field(
                                false,
                                false,
                                None,
                                "".into(),
                                Some({
                                    let ty = <u32 as specta::Type>::reference(type_map, &[]).inner;
                                    ty
                                }),
                            ),
                        ),
                    ],
                    None,
                ),
            ))
        }
        fn definition(type_map: &mut specta::TypeMap) -> specta::DataType {
            Self::inline(type_map, &DEFINITION_GENERICS)
        }
        fn reference(
            type_map: &mut specta::TypeMap,
            generics: &[specta::DataType],
        ) -> specta::reference::Reference {
            {
                let generics: &[specta::DataType] = &[];
                specta::reference::reference::<Self>(
                    type_map,
                    specta::internal::construct::data_type_reference("EdgeID".into(), SID, vec![]),
                )
            }
        }
    }
    #[automatically_derived]
    impl specta::NamedType for EdgeID {
        const SID: specta::SpectaID = SID;
        const IMPL_LOCATION: specta::ImplLocation = IMPL_LOCATION;
        fn named_data_type(
            type_map: &mut specta::TypeMap,
            generics: &[specta::DataType],
        ) -> specta::NamedDataType {
            specta::internal::construct::named_data_type(
                "EdgeID".into(),
                "".into(),
                None,
                SID,
                IMPL_LOCATION,
                <Self as specta::Type>::inline(type_map, generics),
            )
        }
        fn definition_named_data_type(type_map: &mut specta::TypeMap) -> specta::NamedDataType {
            specta::internal::construct::named_data_type(
                "EdgeID".into(),
                "".into(),
                None,
                SID,
                IMPL_LOCATION,
                <Self as specta::Type>::definition(type_map),
            )
        }
    }
    #[automatically_derived]
    impl specta::Flatten for EdgeID {}
};
