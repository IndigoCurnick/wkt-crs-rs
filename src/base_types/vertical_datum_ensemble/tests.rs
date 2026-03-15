use crate::{
	ast::parse_wkt,
	base_types::{DatumEnsembleAccuracy, DatumEnsembleMember},
	types::WktBaseType,
};

use super::VerticalDatumEnsemble;

const EXAMPLE1: &str = r#"ENSEMBLE["EVRS ensemble",
MEMBER["EVRF2000"],
MEMBER["EVRF2007"],
ENSEMBLEACCURACY[0.01]]
"#;

#[test]
fn test_vertical_ensemble() {
	let correct = VerticalDatumEnsemble {
		datum_ensemble_name: "EVRS ensemble".into(),
		datum_ensemble_member: vec![
			DatumEnsembleMember {
				ensemble_member_name: "EVRF2000".into(),
				identifier: None,
			},
			DatumEnsembleMember {
				ensemble_member_name: "EVRF2007".into(),
				identifier: None,
			},
		],
		datum_ensemble_accuracy: DatumEnsembleAccuracy(0.01),
		identifier: None,
	};

	let ast = parse_wkt(EXAMPLE1);

	assert_eq!(ast.len(), 1);

	let vert = VerticalDatumEnsemble::from_nodes(&ast).unwrap();

	assert_eq!(vert.result, correct);
}
