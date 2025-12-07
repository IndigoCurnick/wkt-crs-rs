use wkt_crs_rs::{
    WktCrsTypes,
    base_types::{CoordinateSystem, Order, OrdinalDateTimeAxis, OrdinalDateTimeCoordinateSystem},
    enumerations::{AxisDirection, Dimension, OrdinalDateTimeCsType},
    parse_wkt_crs,
};

const EXAMPLE: &str = r#"CS[ordinal,2],
AXIS["Inline (I)",northEast,ORDER[1]],
AXIS["Crossline (J)",northWest,ORDER[2]]
"#;

#[test]
fn test_ordinal_date_time_coordinate_system() {
    let ocs = CoordinateSystem::OrdinalDateTimeCS(OrdinalDateTimeCoordinateSystem {
        ordinal_date_time_cs_type: OrdinalDateTimeCsType::Ordinal,
        dimension: Dimension::Two,
        identifier: None,
        ordinal_date_time_axis: vec![
            OrdinalDateTimeAxis {
                axis_name_abbreviation: "Inline (I)".to_string(),
                axis_direction: AxisDirection::NorthEast,
                axis_order: Some(Order(1)),
                identifier: None,
            },
            OrdinalDateTimeAxis {
                axis_name_abbreviation: "Crossline (J)".to_string(),
                axis_direction: AxisDirection::NorthWest,
                axis_order: Some(Order(2)),
                identifier: None,
            },
        ],
    });

    let correct = vec![WktCrsTypes::CoordinateSystem(ocs)];

    let ast = parse_wkt_crs(EXAMPLE).unwrap();
    assert_eq!(ast, correct)
}
