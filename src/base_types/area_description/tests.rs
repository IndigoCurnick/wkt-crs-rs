use crate::{ast::parse_wkt, types::WktBaseType};

use super::area_description::AreaDescription;

const EXAMPLE1: &str = r#"AREA["Netherlands offshore."]"#;

#[test]
fn test_area() {
	let correct = AreaDescription("Netherlands offshore.".to_string());

	let ast = parse_wkt(EXAMPLE1);

	assert_eq!(ast.len(), 1);
	let area = AreaDescription::from_nodes(&ast).unwrap();

	assert_eq!(correct, area.result);
	assert_eq!(area.consumed, 1);
}
