use wkt_crs_rs::{
	WktCrsTypes,
	base_types::{Id, Uri},
	data_types::NumText,
	parse_wkt_crs,
};

const EXAMPLE1: &str = r#"ID["Authority Name","Abcd_Ef",7.1]"#;
const EXAMPLE2: &str = r#"ID["EPSG",4326]"#;
const EXAMPLE3: &str = r#"ID["EPSG",4326,URI["urn:ogc:def:crs:EPSG:4326"]]"#;
const EXAMPLE4: &str =
	r#"ID["EuroGeographics","ES_ED50 (BAL90) to ETRS89","2001-04-20"]"#;

#[test]
fn test_id() {
	test_example_1();
	test_example_2();
	test_exmaple_3();
	test_example_4();
}

fn test_example_1() {
	// Example 1
	let correct = vec![WktCrsTypes::Identifier(Id {
		authority_name: "Authority Name".to_string(),
		authority_unique_identifier: NumText::Text("Abcd_Ef".to_string()),
		version: Some(NumText::Float(7.1)),
		authority_citation: None,
		id_uri: None,
	})];

	let ast = parse_wkt_crs(EXAMPLE1).unwrap();

	assert_eq!(correct, ast);
}

fn test_example_2() {
	// Example 2
	let correct = vec![WktCrsTypes::Identifier(Id {
		authority_name: "EPSG".to_string(),
		authority_unique_identifier: NumText::Int(4326),
		version: None,
		authority_citation: None,
		id_uri: None,
	})];

	let ast = parse_wkt_crs(EXAMPLE2).unwrap();

	assert_eq!(correct, ast);
}

fn test_exmaple_3() {
	// Example 3
	let correct = vec![WktCrsTypes::Identifier(Id {
		authority_name: "EPSG".to_string(),
		authority_unique_identifier: NumText::Int(4326),
		version: None,
		authority_citation: None,
		id_uri: Some(Uri("urn:ogc:def:crs:EPSG:4326".to_string())),
	})];

	let ast = parse_wkt_crs(EXAMPLE3).unwrap();

	assert_eq!(correct, ast);
}

fn test_example_4() {
	// Example 4
	let correct = vec![WktCrsTypes::Identifier(Id {
		authority_name: "EuroGeographics".to_string(),
		authority_unique_identifier: NumText::Text(
			"ES_ED50 (BAL90) to ETRS89".to_string(),
		),
		version: Some(NumText::Text("2001-04-20".to_string())),
		authority_citation: None,
		id_uri: None,
	})];

	let ast = parse_wkt_crs(EXAMPLE4).unwrap();
	assert_eq!(correct, ast);
}
