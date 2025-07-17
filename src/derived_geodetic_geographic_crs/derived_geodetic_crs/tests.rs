use std::vec;

use crate::{
    ast::parse_wkt,
    coordinate_system::{
        AxisDirection, AxisOrder, CoordinateSystem, Dimension, SpatialAxis,
        SpatialCoordinateSystem, SpatialCsType,
    },
    datum::GeodeticReferenceFrameDatum,
    derived_crs::{
        DerivedCrsConversionParameter, DerivingConversion, OperationMethod,
        OperationParameterWrapper,
    },
    derived_geodetic_geographic_crs::{
        derived_dynamic_geog_crs::DerivedDynamicGeogCrs,
        derived_geodetic_crs::derived_geodetic_crs::DerivedGeodeticCrs,
        derived_geographic_crs::DerivedGeographicCrs,
    },
    dynamic_crs::{DynamicCrs, FrameReferenceEpoch},
    ellipsoid::Ellipsoid,
    proj_crs::{BaseDynamicCrs, BaseDynamicGeographicCrs},
    scope_extent_identifier_remark::{Id, ScopeExtentIdentifierRemark},
    units::{AngleUnit, LengthUnit, SpatialUnit, Unit},
    utils::NumText,
};

const EXAMPLE1: &str = r#"GEOGCRS["WMI Atlantic Pole",
    BASEGEOGCRS["WGS 84 (G1762)",
        DYNAMIC[FRAMEEPOCH[2005.0]],
        TRF["World Geodetic System 1984 (G1762)",
            ELLIPSOID["WGS 84",6378137,298.25,LENGTHUNIT["metre",1.0]]]
    ],
    DERIVINGCONVERSION["Atlantic pole",
        METHOD["Pole rotation",ID["Authority",1234]],
        PARAMETER["Latitude of rotated pole",52.0,
            ANGLEUNIT["degree",0.017]]
    ],
    CS[ellipsoidal,2]
        AXIS["latitude",north,ORDER[1]],
        AXIS["longitude",east,ORDER[2]],
        ANGLEUNIT["degree",0.017]]
"#;

#[test]
fn test_derived_geographic_crs() {
    let correct = DerivedGeodeticCrs::DerivedGeographicCrs(
        DerivedGeographicCrs::DerivedDynamicGeogCrs(DerivedDynamicGeogCrs {
            derived_crs_name: "WMI Atlantic Pole".to_string(),
            base_dynamic_crs: BaseDynamicCrs::BaseDynamicGeographicCrs(BaseDynamicGeographicCrs {
                base_crs_name: "WGS 84 (G1762)".to_string(),
                dynamic_crs: DynamicCrs {
                    frame_reference_epoch: FrameReferenceEpoch(2005.0),
                    deformation_model_id: None,
                },
                geodetic_data: GeodeticReferenceFrameDatum {
                    datum_name: "World Geodetic System 1984 (G1762)".to_string(),
                    ellipsoid: Ellipsoid {
                        ellipsoid_name: "WGS 84".to_string(),
                        semi_major_axis: 6378137.0,
                        inverse_flattening: 298.25,
                        length_unit: Some(LengthUnit {
                            unit_name: "metre".to_string(),
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
                deriving_conversion_name: "Atlantic pole".to_string(),
                operation_method: OperationMethod {
                    operation_method_name: "Pole rotation".to_string(),
                    identifier: Some(Id {
                        authority_name: "Authority".to_string(),
                        authority_unique_identifier: NumText::Num(1234.0),
                        version: None,
                        authority_citation: None,
                        id_uri: None,
                    }),
                },
                operation_parameter: Some(vec![OperationParameterWrapper::OperationParameter(
                    DerivedCrsConversionParameter {
                        parameter_name: "Latitude of rotated pole".to_string(),
                        parameter_value: 52.0,
                        parameter_unit: Unit::SpatialUnit(SpatialUnit::AngleUnit(AngleUnit {
                            unit_name: "degree".to_string(),
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
                        axis_name_abbreviation: "latitude".to_string(),
                        axis_direction: AxisDirection::North(None),
                        axis_order: Some(AxisOrder { order: 1.0 }),
                        spatial_unit: None,
                        identifier: None,
                    },
                    SpatialAxis {
                        axis_name_abbreviation: "longitude".to_string(),
                        axis_direction: AxisDirection::East,
                        axis_order: Some(AxisOrder { order: 2.0 }),
                        spatial_unit: None,
                        identifier: None,
                    },
                ],
                cs_unit: Some(Unit::SpatialUnit(SpatialUnit::AngleUnit(AngleUnit {
                    unit_name: "degree".to_string(),
                    conversion_factor: 0.017,
                    identifier: None,
                }))),
                needed_args: 4,
            }),
            scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
                identifier: None,
                remark: None,
                usage: None,
            },
        }),
    );

    let ast = parse_wkt(EXAMPLE1);
    assert_eq!(ast.len(), 1);
    let crs = DerivedGeodeticCrs::try_from(&ast[0]).unwrap();

    assert_eq!(crs, correct);
}
