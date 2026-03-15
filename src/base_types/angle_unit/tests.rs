use crate::{
	ast::parse_wkt, base_types::angle_unit::AngleUnit, types::WktBaseType,
};

const AU_EXAMPLE1: &str = r#"ANGLEUNIT["degree",0.0174]"#;

#[test]
fn test_angle_unit() {
	let correct = AngleUnit {
		conversion_factor: 0.0174,
		identifier: None,
		unit_name: "degree".to_string(),
	};

	let ast = parse_wkt(AU_EXAMPLE1);

	assert_eq!(ast.len(), 1);

	let au = AngleUnit::from_nodes(&ast).unwrap();

	assert_eq!(correct, au.result);
}
