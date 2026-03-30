use crate::{ast::parse_wkt, base_types::LengthUnit, types::WktBaseType};

use super::ellipsoid::Ellipsoid;

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
	// EXAMPLE 1

	let correct = Ellipsoid {
		ellipsoid_name: "GRS 1980".to_string(),
		semi_major_axis: 6378132.0,
		inverse_flattening: 298.257222101,
		length_unit: Some(LengthUnit {
			conversion_factor: 1.0,
			unit_name: "metre".to_string(),
			identifier: None,
		}),
		identifier: None,
	};

	let ast = parse_wkt(EXAMPLE1).unwrap();
	assert_eq!(ast.len(), 1);
	let el = Ellipsoid::from_nodes(&ast).unwrap();

	assert_eq!(el.result, correct);

	// EXAMPLE 2
	let correct = Ellipsoid {
		ellipsoid_name: "GRS 1980".to_string(),
		semi_major_axis: 6378132.0,
		inverse_flattening: 298.257222101,
		length_unit: None,
		identifier: None,
	};

	let ast = parse_wkt(EXAMPLE2).unwrap();
	assert_eq!(ast.len(), 1);

	let el = Ellipsoid::from_nodes(&ast).unwrap();

	assert_eq!(el.result, correct);

	// Example 3
	let correct = Ellipsoid {
		ellipsoid_name: "Clark 1866".to_string(),
		semi_major_axis: 20925832.164,
		inverse_flattening: 294.97869821,
		length_unit: Some(LengthUnit {
			conversion_factor: 0.304800609601219,
			unit_name: "US survey foot".to_string(),
			identifier: None,
		}),
		identifier: None,
	};

	let ast = parse_wkt(EXAMPLE3).unwrap();
	assert_eq!(ast.len(), 1);

	let el = Ellipsoid::from_nodes(&ast).unwrap();

	assert_eq!(el.result, correct);

	// Example 4
	let correct = Ellipsoid {
		ellipsoid_name: "Sphere".to_string(),
		semi_major_axis: 6371000.0,
		inverse_flattening: 0.0,
		length_unit: Some(LengthUnit {
			conversion_factor: 1.0,
			unit_name: "metre".to_string(),
			identifier: None,
		}),
		identifier: None,
	};

	let ast = parse_wkt(EXAMPLE4).unwrap();
	assert_eq!(ast.len(), 1);

	let el = Ellipsoid::from_nodes(&ast).unwrap();

	assert_eq!(el.result, correct);
}
