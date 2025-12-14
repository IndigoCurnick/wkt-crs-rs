use crate::{ast::parse_wkt, types::WktBaseType};

use super::TimeUnit;

const EXAMPLE1: &str = r#"TIMEUNIT["millisecond",0.001]"#;
const EXAMPLE2: &str = r#"TIMEUNIT["calendar month"]"#;
const EXAMPLE3: &str = r#"TIMEUNIT["calendar second"]"#;
const EXAMPLE4: &str = r#"TIMEUNIT["day",86400.0]"#;

#[test]
fn test_time_unit() {
    test_example_1();
    test_example_2();
    test_example_3();
    test_example_4();
}

fn test_example_1() {
    let correct = TimeUnit {
        unit_name: "millisecond".to_string(),
        conversion_factor: Some(0.001),
        identifier: None,
    };

    let ast = parse_wkt(EXAMPLE1);
    assert_eq!(ast.len(), 1);
    let time = TimeUnit::from_nodes(&ast).unwrap();

    assert_eq!(time.result, correct);
}

fn test_example_2() {
    let correct = TimeUnit {
        unit_name: "calendar month".to_string(),
        conversion_factor: None,
        identifier: None,
    };

    let ast = parse_wkt(EXAMPLE2);
    assert_eq!(ast.len(), 1);
    let time = TimeUnit::from_nodes(&ast).unwrap();

    assert_eq!(time.result, correct);
}

fn test_example_3() {
    let correct = TimeUnit {
        unit_name: "calendar second".to_string(),
        conversion_factor: None,
        identifier: None,
    };

    let ast = parse_wkt(EXAMPLE3);
    assert_eq!(ast.len(), 1);
    let time = TimeUnit::from_nodes(&ast).unwrap();

    assert_eq!(time.result, correct);
}

fn test_example_4() {
    let correct = TimeUnit {
        unit_name: "day".to_string(),
        conversion_factor: Some(86400.0),
        identifier: None,
    };

    let ast = parse_wkt(EXAMPLE4);
    assert_eq!(ast.len(), 1);
    let time = TimeUnit::from_nodes(&ast).unwrap();

    assert_eq!(time.result, correct);
}
