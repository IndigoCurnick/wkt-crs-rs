use wkt_crs_rs::{
	WktCrsTypes,
	base_types::{Id, Method},
	data_types::NumText,
	parse_wkt_crs,
};

const EXAMPLE: &str = r#"METHOD["NADCON",ID["EPSG",9613]]"#;

#[test]
fn test_operation_method() {
	let correct = Method {
		method_name: "NADCON".into(),
		identifier: Some(Id {
			authority_name: "EPSG".into(),
			authority_unique_identifier: NumText::Int(9613),
			version: None,
			authority_citation: None,
			id_uri: None,
		}),
	};

	let correct = vec![WktCrsTypes::Method(correct)];

	let op = parse_wkt_crs(EXAMPLE).unwrap();

	assert_eq!(correct, op);
}
