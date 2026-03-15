use wkt_crs_rs::{WktCrsTypes, base_types::Scope, parse_wkt_crs};

const EXAMPLE1: &str =
	r#"SCOPE["Large scale topographic mapping and cadastre."]"#;

#[test]
fn test_scope() {
	let correct = vec![WktCrsTypes::Scope(Scope(
		"Large scale topographic mapping and cadastre.".to_string(),
	))];

	let ast = parse_wkt_crs(EXAMPLE1).unwrap();

	assert_eq!(ast, correct);
}
