use crate::{
    ast::parse_wkt,
    ensemble::{ensemble_accuracy::EnsembleAccuracy, ensemble_member::EnsembleMember},
};

use super::VerticalEnsemble;

const EXAMPLE1: &str = r#"ENSEMBLE["EVRS ensemble",
MEMBER["EVRF2000"],
MEMBER["EVRF2007"],
ENSEMBLEACCURACY[0.01]]
"#;

#[test]
fn test_vertical_ensemble() {
    let correct = VerticalEnsemble {
        datum_ensemble_name: "EVRS ensemble".into(),
        datum_ensemble_member: vec![
            EnsembleMember {
                ensemble_member_name: "EVRF2000".into(),
                identifier: None,
            },
            EnsembleMember {
                ensemble_member_name: "EVRF2007".into(),
                identifier: None,
            },
        ],
        datum_ensemble_accuracy: EnsembleAccuracy(0.01),
        identifier: None,
    };

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let vert = VerticalEnsemble::try_from(&ast[0]).unwrap();

    assert_eq!(vert, correct);
}
