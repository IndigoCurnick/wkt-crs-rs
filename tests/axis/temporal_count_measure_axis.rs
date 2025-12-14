use wkt_crs_rs::{
    WktCrsTypes,
    base_types::{Axis, TemporalCountMeasureAxis, TimeUnit},
    enumerations::AxisDirection,
    parse_wkt_crs,
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

    let correct = vec![WktCrsTypes::Axis(Axis::TemporalCountMeasureAxis(correct))];

    let ast = parse_wkt_crs(EXAMPLE).unwrap();

    assert_eq!(correct, ast);
}
