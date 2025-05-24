use crate::ast::parse_wkt;

use super::remark::Remark;

const EXAMPLE1: &str = r#"REMARK["A remark in ASCII"]"#;
const EXAMPLE2: &str = r#"REMARK["Замечание на русском языке"]"#;

#[test]
fn test_remark() {
    // Example 1
    let correct = Remark {
        remark: "A remark in ASCII".to_string(),
    };

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let remark = Remark::try_from(&ast[0]).unwrap();

    assert_eq!(correct, remark);

    // Example 2

    let correct = Remark {
        remark: "Замечание на русском языке".to_string(),
    };

    let ast = parse_wkt(EXAMPLE2);

    assert_eq!(ast.len(), 1);

    let remark = Remark::try_from(&ast[0]).unwrap();

    assert_eq!(correct, remark);
}
