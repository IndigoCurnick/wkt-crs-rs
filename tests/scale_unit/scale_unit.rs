use wkt_crs_rs::{WktCrsTypes, base_types::ScaleUnit, parse_wkt_crs};

const SU_EXAMPLE1: &str = r#"SCALEUNIT["parts per million",1E-06]"#;

#[test]
fn test_scale_unit() {
	let correct = vec![WktCrsTypes::ScaleUnit(ScaleUnit {
		conversion_factor: 1e-6,
		identifier: None,
		unit_name: "parts per million".to_string(),
	})];

	let ast = parse_wkt_crs(SU_EXAMPLE1).unwrap();

	assert_eq!(correct, ast);
}
