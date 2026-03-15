use wkt_crs_rs::{
	WktCrsTypes,
	base_types::{Ellipsoid, LengthUnit},
	parse_wkt_crs,
};

const EXAMPLE1: &str =
	r#"ELLIPSOID["GRS 1980",6378132,298.257222101,LENGTHUNIT["metre",1]]"#;
const EXAMPLE2: &str = r#"SPHEROID["GRS 1980",6378132,298.257222101]"#;
const EXAMPLE3: &str = r#"ELLIPSOID["Clark 1866",20925832.164,294.97869821,
LENGTHUNIT["US survey foot",0.304800609601219]]
"#;
const EXAMPLE4: &str =
	r#"ELLIPSOID["Sphere",6371000,0,LENGTHUNIT["metre",1.0]]"#;

#[test]
fn test_ellipsoid() {
	test_example_1();
	test_example_2();
	test_exmaple_3();
	test_example_4();
}

fn test_example_1() {
	// EXAMPLE 1

	let correct = vec![WktCrsTypes::Ellipsoid(Ellipsoid {
		ellipsoid_name: "GRS 1980".to_string(),
		semi_major_axis: 6378132.0,
		inverse_flattening: 298.257222101,
		length_unit: Some(LengthUnit {
			conversion_factor: 1.0,
			unit_name: "metre".to_string(),
			identifier: None,
		}),
	})];

	let ast = parse_wkt_crs(EXAMPLE1).unwrap();

	assert_eq!(ast, correct);
}

fn test_example_2() {
	// EXAMPLE 2
	let correct = vec![WktCrsTypes::Ellipsoid(Ellipsoid {
		ellipsoid_name: "GRS 1980".to_string(),
		semi_major_axis: 6378132.0,
		inverse_flattening: 298.257222101,
		length_unit: None,
	})];

	let ast = parse_wkt_crs(EXAMPLE2).unwrap();

	assert_eq!(ast, correct);
}

fn test_exmaple_3() {
	// Example 3
	let correct = vec![WktCrsTypes::Ellipsoid(Ellipsoid {
		ellipsoid_name: "Clark 1866".to_string(),
		semi_major_axis: 20925832.164,
		inverse_flattening: 294.97869821,
		length_unit: Some(LengthUnit {
			conversion_factor: 0.304800609601219,
			unit_name: "US survey foot".to_string(),
			identifier: None,
		}),
	})];

	let ast = parse_wkt_crs(EXAMPLE3).unwrap();

	assert_eq!(ast, correct);
}

fn test_example_4() {
	// Example 4
	let correct = vec![WktCrsTypes::Ellipsoid(Ellipsoid {
		ellipsoid_name: "Sphere".to_string(),
		semi_major_axis: 6371000.0,
		inverse_flattening: 0.0,
		length_unit: Some(LengthUnit {
			conversion_factor: 1.0,
			unit_name: "metre".to_string(),
			identifier: None,
		}),
	})];

	let ast = parse_wkt_crs(EXAMPLE4).unwrap();

	assert_eq!(ast, correct);
}
