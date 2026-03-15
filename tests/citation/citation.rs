use wkt_crs_rs::{WktCrsTypes, base_types::Citation, parse_wkt_crs};

const EXAMPLE1: &str = r#"CITATION["some-citation"]"#;

#[test]
fn test_citation() {
	let correct =
		vec![WktCrsTypes::Citation(Citation("some-citation".to_string()))];

	let ast = parse_wkt_crs(EXAMPLE1).unwrap();

	assert_eq!(ast, correct);
}
