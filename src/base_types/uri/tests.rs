use crate::{ast::parse_wkt, types::WktBaseType};

use super::uri::Uri;

const EXAMPLE1: &str = r#"URI["urn:ogc:def:crs:EPSG:4326"]"#;

#[test]
fn test_uri() {
	let correct = Uri("urn:ogc:def:crs:EPSG:4326".to_string());

	let ast = parse_wkt(EXAMPLE1);
	assert_eq!(ast.len(), 1);
	let uri = Uri::from_nodes(&ast).unwrap();

	assert_eq!(uri.result, correct);
}
