use wkt_crs_rs::{WktCrsTypes, base_types::DatumEnsembleMember, parse_wkt_crs};

const EXAMPLE1: &str = r#"MEMBER["WGS 84 ensemble"]"#;

#[test]
fn test_ensemble_member() {
	let correct = vec![WktCrsTypes::DatumEnsembleMember(DatumEnsembleMember {
		ensemble_member_name: "WGS 84 ensemble".into(),
		identifier: None,
	})];

	let ast = parse_wkt_crs(EXAMPLE1).unwrap();

	assert_eq!(ast, correct);
}
