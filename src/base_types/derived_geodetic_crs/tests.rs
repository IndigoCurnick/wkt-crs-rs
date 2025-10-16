use crate::{
    ast::parse_wkt,
    base_types::{
        AngleUnit, BaseDynamicCrs, BaseDynamicGeographicCrs, CoordinateSystem,
        OperationParameter, DerivedDynamicGeogCrs, DerivedGeodeticCrs,
        DerivedGeographicCrs, DerivingConversion, DynamicCrs, Ellipsoid, FrameEpoch,
        GeodeticReferenceFrame, Id, LengthUnit, OperationMethod, Order, SpatialAxis,
        SpatialCoordinateSystem,
    },
    compound_types::{ScopeExtentIdentifierRemark, SpatialUnit, Unit},
    data_types::NumText,
    enumerations::{AxisDirection, Dimension, OperationParameterWrapper, SpatialCsType},
    types::WktBaseType,
};

const EXAMPLE_1: &str = r#"GEOGCRS["WMO Atlantic Pole",
    BASEGEOGCRS["WGS 84 (G1762)",
        DYNAMIC[FRAMEEPOCH[2005.0]],
        TRF["World Geodetic System 1984 (G1762)",
            ELLIPSOID["WGS 84",6378137,298.257,LENGTHUNIT["metre",1.0]]]
    ],
    DERIVINGCONVERSION["Atlantic pole",
        METHOD["Pole rotation",ID["Authority",1234]],
        PARAMETER["Latitude of rotated pole",52.0,
            ANGLEUNIT["degree",0.017]]
    ],
    CS[ellipsoidal,2],
        AXIS["latitude",north,ORDER[1]],
        AXIS["longitude",east,ORDER[2]],
        ANGLEUNIT["degree",0.017]]
"#;

#[test]
fn test_derived_geodetic_crs() {
    let correct = DerivedGeodeticCrs::DerivedGeographicCrs(
        DerivedGeographicCrs::DerivedDynamicGeogCrs(DerivedDynamicGeogCrs {
            derived_crs_name: "WMO Atlantic Pole".into(),
            base_dynamic_crs: BaseDynamicCrs::BaseDynamicGeographicCrs(BaseDynamicGeographicCrs {
                base_crs_name: "WGS 84 (G1762)".into(),
                dynamic_crs: DynamicCrs {
                    frame_reference_epoch: FrameEpoch(2005.0),
                    deformation_model_id: None,
                },
                geodetic_data: GeodeticReferenceFrame {
                    datum_name: "World Geodetic System 1984 (G1762)".into(),
                    ellipsoid: Ellipsoid {
                        ellipsoid_name: "WGS 84".into(),
                        semi_major_axis: 6378137.0,
                        inverse_flattening: 298.257,
                        length_unit: Some(LengthUnit {
                            unit_name: "metre".into(),
                            conversion_factor: 1.0,
                        }),
                    },
                    anchor: None,
                    identifier: None,
                    prime_meridian: None,
                },
                ellipsoidal_cs_unit: None,
                identifier: None,
            }),
            deriving_conversion: DerivingConversion {
                deriving_conversion_name: "Atlantic pole".into(),
                operation_method: OperationMethod {
                    operation_method_name: "Pole rotation".into(),
                    identifier: Some(Id {
                        authority_name: "Authority".into(),
                        authority_unique_identifier: NumText::Int(1234),
                        authority_citation: None,
                        id_uri: None,

                        version: None,
                    }),
                },
                operation_parameter: Some(vec![OperationParameterWrapper::OperationParameter(
                    OperationParameter {
                        parameter_name: "Latitude of rotated pole".into(),
                        parameter_value: 52.0,
                        parameter_unit: Unit::SpatialUnit(SpatialUnit::AngleUnit(AngleUnit {
                            unit_name: "degree".into(),
                            conversion_factor: 0.017,
                            identifier: None,
                        })),
                        identifier: None,
                    },
                )]),
                identifier: None,
            },

            coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
                spatial_cs_type: SpatialCsType::Ellipsoidal,
                dimension: Dimension::Two,
                identifier: None,
                spatial_axis: vec![
                    SpatialAxis {
                        axis_name_abbreviation: "latitude".into(),
                        axis_direction: AxisDirection::North(None),
                        axis_order: Some(Order(1)),
                        spatial_unit: None,
                        identifier: None,
                    },
                    SpatialAxis {
                        axis_name_abbreviation: "longitude".into(),
                        axis_direction: AxisDirection::East,
                        axis_order: Some(Order(2)),
                        spatial_unit: None,
                        identifier: None,
                    },
                ],
                cs_unit: Some(Unit::SpatialUnit(SpatialUnit::AngleUnit(AngleUnit {
                    unit_name: "degree".into(),
                    conversion_factor: 0.017,
                    identifier: None,
                }))),
            }),
            scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
                identifier: None,
                remark: None,
                usage: None,
            },
        }),
    );

    let ast = parse_wkt(EXAMPLE_1);

    let res = DerivedGeodeticCrs::from_nodes(&ast).unwrap();

    assert_eq!(correct, res.result);
}
