use crate::{
    WktBaseType,
    ast::parse_wkt,
    base_types::{Order, OrdinalDateTimeAxis},
    enumerations::AxisDirection,
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

    let ast = parse_wkt(EXAMPLE);

    assert_eq!(ast.len(), 1);

    let order = OrdinalDateTimeAxis::from_nodes(&ast).unwrap();

    assert_eq!(correct, order.result);
}
