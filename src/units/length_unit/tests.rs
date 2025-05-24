use crate::{ast::parse_wkt, units::LengthUnit};

const EXAMPLE1: &str = r#"LENGTHUNIT["metre",1]"#;
const EXAMPLE2: &str = r#"LENGTHUNIT["German legal metre",1.0000135965]"#;

#[test]
fn test_parse_length_unit() {
    // Example 1
    let correct = LengthUnit {
        unit_name: "metre".to_string(),
        conversion_factor: 1.0,
    };

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let lu = LengthUnit::try_from(&ast[0]).unwrap();

    assert_eq!(correct, lu);

    // Example 2

    let correct = LengthUnit {
        unit_name: "German legal metre".to_string(),
        conversion_factor: 1.0000135965,
    };

    let ast = parse_wkt(EXAMPLE2);

    assert_eq!(ast.len(), 1);

    let lu = LengthUnit::try_from(&ast[0]).unwrap();

    assert_eq!(correct, lu);
}
