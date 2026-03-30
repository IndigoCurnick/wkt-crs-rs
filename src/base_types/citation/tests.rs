// ! I was unable to find an example of a citation in the documentation
use crate::{
	ast::parse_wkt, base_types::citation::Citation, types::WktBaseType,
};

const EXAMPLE1: &str = r#"CITATION["some-citation"]"#;

#[test]
fn test_citation() {
	let correct = Citation("some-citation".to_string());

	let ast = parse_wkt(EXAMPLE1).unwrap();

	assert_eq!(ast.len(), 1);

	let citation = Citation::from_nodes(&ast).unwrap();

	assert_eq!(citation.result, correct);
	assert_eq!(citation.consumed, 1);
}
