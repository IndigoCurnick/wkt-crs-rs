use crate::{WktBaseType, ast::parse_wkt, base_types::OperationAccuracy};

const EXAMPLE: &str = "OPERATIONACCURACY[5]";

#[test]
fn test_order() {
	let correct = OperationAccuracy(5.0);

	let ast = parse_wkt(EXAMPLE);

	let op = OperationAccuracy::from_nodes(&ast).unwrap();

	assert_eq!(correct, op.result);
	assert_eq!(op.consumed, 1);
}
