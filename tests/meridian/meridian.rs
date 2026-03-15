use wkt_crs_rs::{
	WktCrsTypes,
	base_types::{AngleUnit, Meridian},
	parse_wkt_crs,
};

const EXAMPLE1: &str = r#"MERIDIAN[180,ANGLEUNIT["degree",0.0174]]"#;

#[test]
fn test_meridian() {
	let correct = vec![WktCrsTypes::Meridian(Meridian {
		number: 180.0,
		angle_unit: AngleUnit {
			unit_name: "degree".to_string(),
			conversion_factor: 0.0174,
			identifier: None,
		},
	})];

	let ast = parse_wkt_crs(EXAMPLE1).unwrap();

	assert_eq!(correct, ast);
}
