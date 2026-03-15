use crate::{ast::parse_wkt, base_types::LengthUnit, types::WktBaseType};

use super::vertical_extent::VerticalExtent;

const EXAMPLE1: &str = r#"VERTICALEXTENT[-1000,0,LENGTHUNIT["metre",1.0]]"#;
const EXAMPLE2: &str = "VERTICALEXTENT[-1000,0]";

#[test]
fn test_vertical_extent() {
	test_example_1();
	test_example_2();
}

fn test_example_1() {
	let correct = VerticalExtent {
		minimum_height: -1000.0,
		maximum_height: 0.0,
		length_unit: Some(LengthUnit {
			unit_name: "metre".to_string(),
			conversion_factor: 1.0,
			identifier: None,
		}),
	};

	let ast = parse_wkt(EXAMPLE1);

	assert_eq!(ast.len(), 1);

	let vert = VerticalExtent::from_nodes(&ast).unwrap();

	assert_eq!(vert.result, correct);
	assert_eq!(vert.consumed, 1);
}

fn test_example_2() {
	let correct = VerticalExtent {
		minimum_height: -1000.0,
		maximum_height: 0.0,
		length_unit: None,
	};

	let ast = parse_wkt(EXAMPLE2);

	assert_eq!(ast.len(), 1);

	let vert = VerticalExtent::from_nodes(&ast).unwrap();

	assert_eq!(vert.result, correct);
	assert_eq!(vert.consumed, 1);
}
