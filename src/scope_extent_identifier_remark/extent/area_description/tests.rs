use crate::ast::parse_wkt;

use super::area_description::AreaDescription;

const EXAMPLE1: &str = r#"AREA["Netherlands offshore."]"#;

#[test]
fn test_area() {
    let correct = AreaDescription("Netherlands offshore.".to_string());

    let ast = parse_wkt(EXAMPLE1);
    assert_eq!(ast.len(), 1);
    let area = AreaDescription::try_from(&ast[0]).unwrap();

    assert_eq!(correct, area);
}
