use crate::{ast::parse_wkt, types::WktBaseType};

use super::remark::Remark;

const EXAMPLE1: &str = r#"REMARK["A remark in ASCII"]"#;
const EXAMPLE2: &str = r#"REMARK["Замечание на русском языке"]"#;

#[test]
fn test_remark() {
	test_example_1();
	test_example_2();
}

fn test_example_1() {
	let correct = Remark("A remark in ASCII".to_string());

	let ast = parse_wkt(EXAMPLE1);

	assert_eq!(ast.len(), 1);

	let remark = Remark::from_nodes(&ast).unwrap();

	assert_eq!(correct, remark.result);
}

fn test_example_2() {
	let correct = Remark("Замечание на русском языке".to_string());

	let ast = parse_wkt(EXAMPLE2);

	assert_eq!(ast.len(), 1);

	let remark = Remark::from_nodes(&ast).unwrap();

	assert_eq!(correct, remark.result);
}
