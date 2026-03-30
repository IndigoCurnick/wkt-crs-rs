use crate::{ast::parse_wkt, base_types::AngleUnit, types::WktBaseType};

use super::PrimeMeridian;

const EXAMPLE1: &str =
	r#"PRIMEM["Paris",2.5969213,ANGLEUNIT["grad",0.015707]]"#;
const EXAMPLE2: &str = r#"PRIMEM["Ferro",-17.6666667]"#;
const EXAMPLE3: &str = r#"PRIMEM["Greenwich",0.0,ANGLEUNIT["degree",0.017]]"#;

#[test]
fn test_primem() {
	// Example 1

	let correct = PrimeMeridian {
		prime_meridian_name: "Paris".into(),
		irm_longitude: 2.5969213,
		angle_unit: Some(AngleUnit {
			unit_name: "grad".into(),
			conversion_factor: 0.015707,
			identifier: None,
		}),
		identifier: None,
	};

	let ast = parse_wkt(EXAMPLE1).unwrap();

	assert_eq!(ast.len(), 1);

	let primem = PrimeMeridian::from_nodes(&ast).unwrap();

	assert_eq!(correct, primem.result);

	// Example 2

	let correct = PrimeMeridian {
		prime_meridian_name: "Ferro".into(),
		irm_longitude: -17.6666667,
		angle_unit: None,
		identifier: None,
	};

	let ast = parse_wkt(EXAMPLE2).unwrap();

	assert_eq!(ast.len(), 1);

	let primem = PrimeMeridian::from_nodes(&ast).unwrap();

	assert_eq!(correct, primem.result);

	// Example 3

	let correct = PrimeMeridian {
		prime_meridian_name: "Greenwich".into(),
		irm_longitude: 0.0,
		angle_unit: Some(AngleUnit {
			unit_name: "degree".into(),
			conversion_factor: 0.017,
			identifier: None,
		}),
		identifier: None,
	};

	let ast = parse_wkt(EXAMPLE3).unwrap();

	assert_eq!(ast.len(), 1);

	let primem = PrimeMeridian::from_nodes(&ast).unwrap();

	assert_eq!(correct, primem.result);
}
