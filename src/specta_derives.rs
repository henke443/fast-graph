use crate::*;

/* -------------------------------------------------------------------------- */
/*                                   NodeID                                   */
/* -------------------------------------------------------------------------- */

#[cfg(feature = "specta")]
const _: () = {
    const SID: specta::SpectaID = specta::internal::construct::sid(
        "NodeID",
        concat!("::", module_path!(), ":", line!(), ":", column!()),
    );
    const IMPL_LOCATION: specta::ImplLocation =
        specta::internal::construct::impl_location(concat!(file!(), ":", line!(), ":", column!()));
    const DEFINITION_GENERICS: &[specta::DataType] = &[];

    #[automatically_derived]
    impl specta::Type for NodeID {
        fn inline(
            type_map: &mut specta::TypeMap,
            generics: &[specta::DataType],
        ) -> specta::DataType {
            specta::DataType::Struct(specta::internal::construct::r#struct(
                "NodeID".into(),
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
                    specta::internal::construct::data_type_reference("NodeID".into(), SID, vec![]),
                )
            }
        }
    }
    #[automatically_derived]
    impl specta::NamedType for NodeID {
        const SID: specta::SpectaID = SID;
        const IMPL_LOCATION: specta::ImplLocation = IMPL_LOCATION;
        fn named_data_type(
            type_map: &mut specta::TypeMap,
            generics: &[specta::DataType],
        ) -> specta::NamedDataType {
            specta::internal::construct::named_data_type(
                "NodeID".into(),
                "".into(),
                None,
                SID,
                IMPL_LOCATION,
                <Self as specta::Type>::inline(type_map, generics),
            )
        }
        fn definition_named_data_type(type_map: &mut specta::TypeMap) -> specta::NamedDataType {
            specta::internal::construct::named_data_type(
                "NodeID".into(),
                "".into(),
                None,
                SID,
                IMPL_LOCATION,
                <Self as specta::Type>::definition(type_map),
            )
        }
    }
    #[automatically_derived]
    impl specta::Flatten for NodeID {}
};

/* -------------------------------------------------------------------------- */
/*                                   EdgeID                                   */
/* -------------------------------------------------------------------------- */

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
