use crate::{
    WktBaseType,
    ast::parse_wkt,
    base_types::{TemporalCountMeasureAxis, TimeUnit},
    enumerations::AxisDirection,
};

const EXAMPLE: &str = r#"AXIS["(T)",future,TIMEUNIT["millisecond (ms)",0.001]]"#;

#[test]
fn test_temporal_count_measure_axis() {
    let correct = TemporalCountMeasureAxis {
        axis_name_abbreviation: "(T)".into(),
        axis_direction: AxisDirection::Future,
        axis_order: None,
        time_unit: Some(TimeUnit {
            unit_name: "millisecond (ms)".to_string(),
            conversion_factor: Some(0.001),
            identifier: None,
        }),
        identifier: None,
    };

    let ast = parse_wkt(EXAMPLE);

    assert_eq!(ast.len(), 1);

    let order = TemporalCountMeasureAxis::from_nodes(&ast).unwrap();

    assert_eq!(correct, order.result);
}
