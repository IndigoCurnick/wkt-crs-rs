use crate::ast::parse_wkt;

use super::uri::Uri;

const EXAMPLE1: &str = r#"URI["urn:ogc:def:crs:EPSG:4326"]"#;

#[test]
fn test_uri() {
    let correct = Uri("urn:ogc:def:crs:EPSG:4326".to_string());

    let ast = parse_wkt(EXAMPLE1);

    let uri = Uri::try_from(&ast).unwrap();

    assert_eq!(uri, correct);
}
