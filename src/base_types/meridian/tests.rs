use crate::{
    ast::parse_wkt,
    base_types::{AngleUnit, meridian::Meridian},
    types::WktBaseType,
};

const EXAMPLE1: &str = r#"MERIDIAN[180,ANGLEUNIT["degree",0.0174]]"#;

#[test]
fn test_meridian() {
    let correct = Meridian {
        number: 180.0,
        angle_unit: AngleUnit {
            unit_name: "degree".to_string(),
            conversion_factor: 0.0174,
            identifier: None,
        },
    };

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let meridian = Meridian::from_nodes(&ast).unwrap();

    assert_eq!(correct, meridian.result);
}
