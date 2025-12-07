use wkt_crs_rs::{
    WktCrsTypes,
    base_types::{CoordinateSystem, LengthUnit, SpatialAxis, SpatialCoordinateSystem},
    compound_types::{SpatialUnit, Unit},
    enumerations::{AxisDirection, Dimension, SpatialCsType},
    parse_wkt_crs,
};

const EXAMPLE: &str = r#"CS[Cartesian,3],
AXIS["(X)",geocentricX],
AXIS["(Y)",geocentricY],
AXIS["(Z)",geocentricZ],
LENGTHUNIT["metre",1.0]
"#;

#[test]
fn test_spatial_coordinate_system() {
    let scs = CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
        spatial_cs_type: SpatialCsType::Cartesian,
        dimension: Dimension::Three,
        identifier: None,
        spatial_axis: vec![
            SpatialAxis {
                axis_name_abbreviation: "(X)".to_string(),
                axis_direction: AxisDirection::GeocentricX,
                axis_order: None,
                identifier: None,
                spatial_unit: None,
            },
            SpatialAxis {
                axis_name_abbreviation: "(Y)".to_string(),
                axis_direction: AxisDirection::GeocentricY,
                axis_order: None,
                identifier: None,
                spatial_unit: None,
            },
            SpatialAxis {
                axis_name_abbreviation: "(Z)".to_string(),
                axis_direction: AxisDirection::GeocentricZ,
                axis_order: None,
                identifier: None,
                spatial_unit: None,
            },
        ],
        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
            unit_name: "metre".to_string(),
            conversion_factor: 1.0,
        }))),
    });

    let correct = vec![WktCrsTypes::CoordinateSystem(scs)];

    let ast = parse_wkt_crs(EXAMPLE).unwrap();

    assert_eq!(correct, ast);
}
