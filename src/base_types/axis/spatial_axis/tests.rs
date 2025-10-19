use crate::{
    ast::parse_wkt,
    base_types::{Order, SpatialAxis},
    enumerations::AxisDirection,
    types::WktBaseType,
};

const EXAMPLE1: &str = r#"AXIS["(X)",geocentricX,ORDER[1]]"#;

#[test]
fn test_spatial_axis() {
    let correct = SpatialAxis {
        axis_name_abbreviation: "(X)".into(),
        axis_direction: AxisDirection::GeocentricX,
        axis_order: Some(Order(1)),
        spatial_unit: None,
        identifier: None,
    };

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let order = SpatialAxis::from_nodes(&ast).unwrap();

    assert_eq!(correct, order.result);
}
