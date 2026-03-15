use wkt_crs_rs::{WktCrsTypes, base_types::ParametricUnit, parse_wkt_crs};

const EXAMPLE1: &str = r#"PARAMETRICUNIT["hectopascal",100]"#;

#[test]
fn test_parametric_unit() {
	let correct = vec![WktCrsTypes::ParametricUnit(ParametricUnit {
		unit_name: "hectopascal".to_string(),
		conversion_factor: 100.0,
		identifier: None,
	})];

	let ast = parse_wkt_crs(EXAMPLE1).unwrap();

	assert_eq!(correct, ast);
}
