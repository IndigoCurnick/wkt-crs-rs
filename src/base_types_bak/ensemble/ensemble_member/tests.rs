use crate::ast::parse_wkt;

use super::ensemble_member::EnsembleMember;

const EXAMPLE1: &str = r#"MEMBER["WGS 84 ensemble"]"#;

#[test]
fn test_ensemble_member() {
    let correct = EnsembleMember {
        ensemble_member_name: "WGS 84 ensemble".into(),
        identifier: None,
    };

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let member = EnsembleMember::try_from(&ast[0]).unwrap();

    assert_eq!(member, correct);
}
