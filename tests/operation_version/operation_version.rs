use wkt_crs_rs::{WktCrsTypes, base_types::OperationVersion, parse_wkt_crs};

const EXAMPLE: &str = r#"VERSION["GSI"]"#;

#[test]
fn test_operation_version() {
	let correct = OperationVersion("GSI".into());

	let correct = vec![WktCrsTypes::OperationVersion(correct)];

	let ast = parse_wkt_crs(EXAMPLE).unwrap();

	assert_eq!(correct, ast);
}
