use wkt_crs_rs::{WktCrsTypes, base_types::OperationParameterFile, parse_wkt_crs};

const EXAMPLE: &str =
    r#"PARAMETERFILE["Latitude and longitude difference file","nzdg2kgrid0005.gsb"]"#;

#[test]
fn test_operation_parameter_file() {
    let correct = OperationParameterFile {
        parameter_name: "Latitude and longitude difference file".into(),
        parameter_file_name: "nzdg2kgrid0005.gsb".into(),
        identifier: None,
    };

    let correct = vec![WktCrsTypes::OperationParameterFile(correct)];

    let ast = parse_wkt_crs(EXAMPLE).unwrap();

    assert_eq!(correct, ast);
}
