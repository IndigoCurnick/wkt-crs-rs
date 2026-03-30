use crate::{WktBaseType, ast::parse_wkt, base_types::OperationVersion};

const EXAMPLE: &str = r#"VERSION["GSI"]"#;

#[test]
fn test_operation_version() {
	let correct = OperationVersion("GSI".into());

	let ast = parse_wkt(EXAMPLE).unwrap();

	let res = OperationVersion::from_nodes(&ast).unwrap();

	assert_eq!(correct, res.result);
}
