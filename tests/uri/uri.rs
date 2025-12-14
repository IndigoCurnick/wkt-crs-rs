use wkt_crs_rs::{WktCrsTypes, base_types::Uri, parse_wkt_crs};

const EXAMPLE1: &str = r#"URI["urn:ogc:def:crs:EPSG:4326"]"#;

#[test]
fn test_uri() {
    let correct = Uri("urn:ogc:def:crs:EPSG:4326".to_string());

    let correct = vec![WktCrsTypes::Uri(correct)];

    let ast = parse_wkt_crs(EXAMPLE1).unwrap();

    assert_eq!(ast, correct);
}
