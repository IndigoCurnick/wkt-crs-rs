use crate::{ast::parse_wkt, types::WktBaseType};

use super::Bearing;

const EXAMPLE1: &str = "BEARING[0]";

#[test]
fn test_bearing() {
	let correct = Bearing(0.0);

	let ast = parse_wkt(EXAMPLE1).unwrap();

	assert_eq!(ast.len(), 1);

	let bearing = Bearing::from_nodes(&ast).unwrap();

	assert_eq!(correct, bearing.result);
}
