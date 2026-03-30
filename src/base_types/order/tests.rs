use crate::{ast::parse_wkt, base_types::order::Order, types::WktBaseType};

const EXAMPLE1: &str = "ORDER[0]";
const EXAMPLE2: &str = "ORDER[1]";

#[test]
fn test_order() {
	test_example_1();
	test_example_2();
}

fn test_example_1() {
	let correct = Order(0);

	let ast = parse_wkt(EXAMPLE1).unwrap();

	assert_eq!(ast.len(), 1);

	let order = Order::from_nodes(&ast).unwrap();

	assert_eq!(correct, order.result);
}

fn test_example_2() {
	let correct = Order(1);

	let ast = parse_wkt(EXAMPLE2).unwrap();

	assert_eq!(ast.len(), 1);

	let order = Order::from_nodes(&ast).unwrap();

	assert_eq!(correct, order.result);
}
