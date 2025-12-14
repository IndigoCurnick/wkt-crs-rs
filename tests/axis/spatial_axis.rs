use wkt_crs_rs::{
    WktCrsTypes,
    base_types::{Axis, Order, SpatialAxis},
    enumerations::AxisDirection,
    parse_wkt_crs,
};

const EXAMPLE1: &str = r#"AXIS["(X)",geocentricX,ORDER[1]]"#;

#[test]
fn test_spatial_axis() {
    let spatial = SpatialAxis {
        axis_name_abbreviation: "(X)".into(),
        axis_direction: AxisDirection::GeocentricX,
        axis_order: Some(Order(1)),
        spatial_unit: None,
        identifier: None,
    };

    let correct = vec![WktCrsTypes::Axis(Axis::SpatialAxis(spatial))];

    let ast = parse_wkt_crs(EXAMPLE1).unwrap();

    assert_eq!(correct, ast);
}
