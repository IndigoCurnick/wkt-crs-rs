use crate::{
	WktBaseType,
	ast::parse_wkt,
	base_types::{Axis, Order, TimeUnit},
	compound_types::Unit,
	enumerations::AxisDirection,
};

const EXAMPLE1: &str = r#"AXIS["Inline (I)",northEast,ORDER[1]]"#;
const EXAMPLE2: &str = r#"AXIS["(X)",geocentricX,ORDER[1]]"#;
const EXAMPLE3: &str =
	r#"AXIS["(T)",future,TIMEUNIT["millisecond (ms)",0.001]]"#;

#[test]
fn test_ordinal_date_time_axis() {
	let correct = Axis {
		axis_name_abbreviation: "Inline (I)".to_string(),
		axis_direction: AxisDirection::NorthEast,
		axis_order: Some(Order(1)),
		unit: None,
		identifier: None,
	};

	let ast = parse_wkt(EXAMPLE1).unwrap();

	assert_eq!(ast.len(), 1);

	let order = Axis::from_nodes(&ast).unwrap();

	assert_eq!(correct, order.result);
}

#[test]
fn test_spatial_axis() {
	let correct = Axis {
		axis_name_abbreviation: "(X)".into(),
		axis_direction: AxisDirection::GeocentricX,
		axis_order: Some(Order(1)),
		unit: None,
		identifier: None,
	};

	let ast = parse_wkt(EXAMPLE2).unwrap();

	assert_eq!(ast.len(), 1);

	let order = Axis::from_nodes(&ast).unwrap();

	assert_eq!(correct, order.result);
}

#[test]
fn test_temporal_count_measure_axis() {
	let correct = Axis {
		axis_name_abbreviation: "(T)".into(),
		axis_direction: AxisDirection::Future,
		axis_order: None,
		unit: Some(Unit::TimeUnit(TimeUnit {
			unit_name: "millisecond (ms)".to_string(),
			conversion_factor: Some(0.001),
			identifier: None,
		})),
		identifier: None,
	};

	let ast = parse_wkt(EXAMPLE3).unwrap();

	assert_eq!(ast.len(), 1);

	let order = Axis::from_nodes(&ast).unwrap();

	assert_eq!(correct, order.result);
}
