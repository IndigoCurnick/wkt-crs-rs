use wkt_crs_rs::{WktCrsTypes, base_types::LengthUnit, parse_wkt_crs};

const EXAMPLE1: &str = r#"LENGTHUNIT["metre",1]"#;
const EXAMPLE2: &str = r#"LENGTHUNIT["German legal metre",1.0000135965]"#;

#[test]
fn test_parse_length_unit() {
	test_example_1();
	test_example_2();
}

fn test_example_1() {
	// Example 1
	let correct = vec![WktCrsTypes::LengthUnit(LengthUnit {
		unit_name: "metre".to_string(),
		conversion_factor: 1.0,
		identifier: None,
	})];

	let ast = parse_wkt_crs(EXAMPLE1).unwrap();

	assert_eq!(correct, ast);
}

fn test_example_2() {
	// Example 2

	let correct = vec![WktCrsTypes::LengthUnit(LengthUnit {
		unit_name: "German legal metre".to_string(),
		conversion_factor: 1.0000135965,
		identifier: None,
	})];

	let ast = parse_wkt_crs(EXAMPLE2).unwrap();

	assert_eq!(correct, ast);
}
