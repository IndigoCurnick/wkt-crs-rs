use crate::ast::parse_wkt;

use super::ParametricUnit;

const EXAMPLE1: &str = r#"PARAMETRICUNIT["hectopascal",100]"#;

#[test]
fn test_parametric_unit() {
    let correct = ParametricUnit {
        unit_name: "hectopascal".to_string(),
        conversion_factor: 100.0,
        identifier: None,
    };

    let ast = parse_wkt(EXAMPLE1);

    let parametric = ParametricUnit::try_from(&ast).unwrap();

    assert_eq!(correct, parametric);
}
