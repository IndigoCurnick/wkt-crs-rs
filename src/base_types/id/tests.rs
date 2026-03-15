use crate::{
	ast::parse_wkt, base_types::Uri, data_types::NumText, types::WktBaseType,
};

use super::Id;

const EXAMPLE1: &str = r#"ID["Authority Name","Abcd_Ef",7.1]"#;
const EXAMPLE2: &str = r#"ID["EPSG",4326]"#;
const EXAMPLE3: &str = r#"ID["EPSG",4326,URI["urn:ogc:def:crs:EPSG:4326"]]"#;
const EXAMPLE4: &str =
	r#"ID["EuroGeographics","ES_ED50 (BAL90) to ETRS89","2001-04-20"]"#;

#[test]
fn test_id() {
	// Example 1
	let correct = Id {
		authority_name: "Authority Name".to_string(),
		authority_unique_identifier: NumText::Text("Abcd_Ef".to_string()),
		version: Some(NumText::Float(7.1)),
		authority_citation: None,
		id_uri: None,
	};

	let ast = parse_wkt(EXAMPLE1);
	assert_eq!(ast.len(), 1);
	let id = Id::from_nodes(&ast).unwrap();

	assert_eq!(correct, id.result);

	// Example 2
	let correct = Id {
		authority_name: "EPSG".to_string(),
		authority_unique_identifier: NumText::Int(4326),
		version: None,
		authority_citation: None,
		id_uri: None,
	};

	let ast = parse_wkt(EXAMPLE2);
	assert_eq!(ast.len(), 1);
	let id = Id::from_nodes(&ast).unwrap();

	assert_eq!(correct, id.result);

	// Example 3
	let correct = Id {
		authority_name: "EPSG".to_string(),
		authority_unique_identifier: NumText::Int(4326),
		version: None,
		authority_citation: None,
		id_uri: Some(Uri("urn:ogc:def:crs:EPSG:4326".to_string())),
	};

	let ast = parse_wkt(EXAMPLE3);
	assert_eq!(ast.len(), 1);
	let id = Id::from_nodes(&ast).unwrap();

	assert_eq!(correct, id.result);

	// Example 4
	let correct = Id {
		authority_name: "EuroGeographics".to_string(),
		authority_unique_identifier: NumText::Text(
			"ES_ED50 (BAL90) to ETRS89".to_string(),
		),
		version: Some(NumText::Text("2001-04-20".to_string())),
		authority_citation: None,
		id_uri: None,
	};

	let ast = parse_wkt(EXAMPLE4);
	assert_eq!(ast.len(), 1);
	let id = Id::from_nodes(&ast).unwrap();

	assert_eq!(correct, id.result);
}
