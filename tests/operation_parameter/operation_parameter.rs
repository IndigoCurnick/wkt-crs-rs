use wkt_crs_rs::{
	WktCrsTypes,
	base_types::{AngleUnit, OperationParameter},
	compound_types::{SpatialUnit, Unit},
	parse_wkt_crs,
};

const EXAMPLE: &str = r#"PARAMETER["Latitude of rotated pole",52.0,
            ANGLEUNIT["degree",0.017]]"#;

#[test]
fn test_operation_parameter() {
	let correct = OperationParameter {
		parameter_name: "Latitude of rotated pole".into(),
		parameter_value: 52.0,
		parameter_unit: Some(Unit::SpatialUnit(SpatialUnit::AngleUnit(
			AngleUnit {
				unit_name: "degree".into(),
				conversion_factor: 0.017,
				identifier: None,
			},
		))),
		identifier: None,
	};

	let correct = vec![WktCrsTypes::OperationParameter(correct)];

	let ast = parse_wkt_crs(EXAMPLE).unwrap();

	assert_eq!(correct, ast);
}
