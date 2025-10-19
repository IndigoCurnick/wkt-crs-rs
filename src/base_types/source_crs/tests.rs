use crate::{
    ast::parse_wkt,
    base_types::{
        CoordinateSystem, Ellipsoid, GeodeticCrs, GeodeticReferenceFrame, LengthUnit, Order,
        SourceCrs, SpatialAxis, SpatialCoordinateSystem, StaticGeodeticCrs,
    },
    compound_types::{
        CoordinateReferenceSystem, GeodeticData, ScopeExtentIdentifierRemark, SingleCrs,
        SpatialUnit, Unit,
    },
    enumerations::{AxisDirection, Dimension, SpatialCsType},
    types::WktBaseType,
};

const EXAMPLE1: &str = r#"SOURCECRS[
GEODCRS["Tokyo",
    DATUM["Tokyo 1918",
        ELLIPSOID["Bessel 1841",6377397.155,299.1528128,
            LENGTHUNIT["metre",1.0]]],
    CS[Cartesian,3],
        AXIS["(X)",geocentricX,ORDER[1]],
        AXIS["(Y)",geocentricY,ORDER[2]],
        AXIS["(Z)",geocentricZ,ORDER[3]],
        LENGTHUNIT["metre",1.0]
]
]
"#;

#[test]
fn test_source_crs() {
    let correct = SourceCrs {
        coordinate_system: CoordinateReferenceSystem::SingleCrs(SingleCrs::GeodeticCrs(
            GeodeticCrs::StaticGeodeticCrs(StaticGeodeticCrs {
                crs_name: "Tokyo".into(),
                frame: GeodeticData::GeodeticReferenceFrame(GeodeticReferenceFrame {
                    datum_name: "Tokyo 1918".into(),
                    ellipsoid: Ellipsoid {
                        ellipsoid_name: "Bessel 1841".into(),
                        semi_major_axis: 6377397.155,
                        inverse_flattening: 299.1528128,
                        length_unit: Some(LengthUnit {
                            unit_name: "metre".into(),
                            conversion_factor: 1.0,
                        }),
                    },
                    anchor: None,
                    identifier: None,
                    prime_meridian: None,
                }),
                coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
                    spatial_cs_type: SpatialCsType::Cartesian,
                    dimension: Dimension::Three,
                    identifier: None,
                    spatial_axis: vec![
                        SpatialAxis {
                            axis_name_abbreviation: "(X)".into(),
                            axis_direction: AxisDirection::GeocentricX,
                            axis_order: Some(Order(1)),
                            spatial_unit: None,
                            identifier: None,
                        },
                        SpatialAxis {
                            axis_name_abbreviation: "(Y)".into(),
                            axis_direction: AxisDirection::GeocentricY,
                            axis_order: Some(Order(2)),
                            spatial_unit: None,
                            identifier: None,
                        },
                        SpatialAxis {
                            axis_name_abbreviation: "(Z)".into(),
                            axis_direction: AxisDirection::GeocentricZ,
                            axis_order: Some(Order(3)),
                            spatial_unit: None,
                            identifier: None,
                        },
                    ],
                    cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                        unit_name: "metre".into(),
                        conversion_factor: 1.0,
                    }))),
                }),
                scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
                    identifier: None,
                    remark: None,
                    usage: None,
                },
            }),
        )),
    };

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let order = SourceCrs::from_nodes(&ast).unwrap();

    assert_eq!(correct, order.result);
}
