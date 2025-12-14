use wkt_crs_rs::{
    WktCrsTypes,
    base_types::{Axis, Order, OrdinalDateTimeAxis},
    enumerations::AxisDirection,
    parse_wkt_crs,
};

const EXAMPLE: &str = r#"AXIS["Inline (I)",northEast,ORDER[1]]"#;

#[test]
fn test_ordinal_date_time_axis() {
    let correct = OrdinalDateTimeAxis {
        axis_name_abbreviation: "Inline (I)".to_string(),
        axis_direction: AxisDirection::NorthEast,
        axis_order: Some(Order(1)),
        identifier: None,
    };

    let correct = vec![WktCrsTypes::Axis(Axis::OrdinalDateTimeAxis(correct))];

    let ast = parse_wkt_crs(EXAMPLE).unwrap();

    assert_eq!(correct, ast);
}
