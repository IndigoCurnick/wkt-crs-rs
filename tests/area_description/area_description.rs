use wkt_crs_rs::{WktCrsTypes, base_types::AreaDescription, parse_wkt_crs};

const EXAMPLE1: &str = r#"AREA["Netherlands offshore."]"#;

#[test]
fn test_area() {
	let correct = vec![WktCrsTypes::AreaDescription(AreaDescription(
		"Netherlands offshore.".to_string(),
	))];

	let ast = parse_wkt_crs(EXAMPLE1).unwrap();

	assert_eq!(correct, ast);
}
