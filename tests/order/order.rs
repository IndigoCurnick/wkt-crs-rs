use wkt_crs_rs::{WktCrsTypes, base_types::Order, parse_wkt_crs};

const EXAMPLE1: &str = "ORDER[0]";
const EXAMPLE2: &str = "ORDER[1]";

#[test]
fn test_order() {
	test_example_1();
	test_example_2();
}

fn test_example_1() {
	let correct = vec![WktCrsTypes::Order(Order(0))];

	let ast = parse_wkt_crs(EXAMPLE1).unwrap();

	assert_eq!(correct, ast);
}

fn test_example_2() {
	let correct = vec![WktCrsTypes::Order(Order(1))];

	let ast = parse_wkt_crs(EXAMPLE2).unwrap();

	assert_eq!(correct, ast);
}
