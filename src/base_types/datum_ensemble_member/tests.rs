use crate::{ast::parse_wkt, types::WktBaseType};

use super::datum_ensemble_member::DatumEnsembleMember;

const EXAMPLE1: &str = r#"MEMBER["WGS 84 ensemble"]"#;

#[test]
fn test_ensemble_member() {
    let correct = DatumEnsembleMember {
        ensemble_member_name: "WGS 84 ensemble".into(),
        identifier: None,
    };

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let member = DatumEnsembleMember::from_nodes(&ast).unwrap();

    assert_eq!(member.result, correct);
}
