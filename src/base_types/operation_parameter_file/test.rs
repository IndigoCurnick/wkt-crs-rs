use crate::{WktBaseType, ast::parse_wkt, base_types::OperationParameterFile};

const EXAMPLE: &str = r#"PARAMETERFILE["Latitude and longitude difference file","nzdg2kgrid0005.gsb"]"#;

#[test]
fn test_operation_parameter_file() {
	let correct = OperationParameterFile {
		parameter_name: "Latitude and longitude difference file".into(),
		parameter_file_name: "nzdg2kgrid0005.gsb".into(),
		identifier: None,
	};

	let ast = parse_wkt(EXAMPLE);

	let res = OperationParameterFile::from_nodes(&ast).unwrap();

	assert_eq!(correct, res.result);
}
