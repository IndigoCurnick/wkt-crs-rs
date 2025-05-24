use crate::ast::parse_wkt;

use super::area_description::AreaDescription;

const EXAMPLE1: &str = r#"AREA["Netherlands offshore."]"#;

fn test_area() {
    let correct = AreaDescription("Netherlands offshore.".to_string());

    let ast = parse_wkt(EXAMPLE1);

    let area = AreaDescription::try_from(&ast).unwrap();

    assert_eq!(correct, area);
}
