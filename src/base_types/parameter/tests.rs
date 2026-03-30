use crate::{
	WktBaseType,
	ast::parse_wkt,
	base_types::{AngleUnit, Parameter},
	compound_types::{SpatialUnit, Unit},
};

const EXAMPLE: &str = r#"PARAMETER["Latitude of rotated pole",52.0,
            ANGLEUNIT["degree",0.017]]"#;

#[test]
fn test_operation_parameter() {
	let correct = Parameter {
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

	let ast = parse_wkt(EXAMPLE).unwrap();

	let res = Parameter::from_nodes(&ast).unwrap();

	assert_eq!(correct, res.result);
}
