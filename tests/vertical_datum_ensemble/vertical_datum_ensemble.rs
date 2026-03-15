use wkt_crs_rs::{
	WktCrsTypes,
	base_types::{
		DatumEnsembleAccuracy, DatumEnsembleMember, VerticalDatumEnsemble,
	},
	parse_wkt_crs,
};

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

	let correct = vec![WktCrsTypes::VerticalDatumEnsemble(correct)];

	let ast = parse_wkt_crs(EXAMPLE1).unwrap();

	assert_eq!(ast, correct);
}
