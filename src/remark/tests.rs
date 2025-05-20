use crate::ast::parse_wkt;

use super::remark::Remark;

const EXAMPLE1: &str = r#"REMARK["A remark in ASCII"]"#;

#[test]
fn test_remark() {
    let correct = Remark {
        remark: "A remark in ASCII".to_string(),
    };

    let ast = parse_wkt(EXAMPLE1);

    let remark = Remark::try_from(&ast).unwrap();

    assert_eq!(correct, remark);
}
