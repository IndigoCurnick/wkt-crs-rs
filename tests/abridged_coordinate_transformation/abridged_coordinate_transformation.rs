use wkt_crs_rs::{
	WktCrsTypes,
	base_types::{AbridgedCoordinateTransformation, Id, Method, Parameter},
	compound_types::ScopeExtentIdentifierRemark,
	data_types::NumText,
	enumerations::OperationParameterWrapper,
	parse_wkt_crs,
};

const EXAMPLE1: &str = r#"ABRIDGEDTRANSFORMATION["Tokyo to JGD2000 (GSI)",
    METHOD["Geocentric translations",ID["EPSG",1031]],
    PARAMETER["X-axis translation",-146.414],
    PARAMETER["Y-axis translation",507.337],
    PARAMETER["Z-axis translation",680.507]
]
"#;

#[test]
fn test_abridged_transformation() {
	let correct = AbridgedCoordinateTransformation {
		operation_name: "Tokyo to JGD2000 (GSI)".to_string(),
		operation_version: None,
		operation_method: Method {
			method_name: "Geocentric translations".to_string(),
			identifier: Some(Id {
				authority_name: "EPSG".to_string(),
				authority_unique_identifier: NumText::Int(1031),
				version: None,
				authority_citation: None,
				id_uri: None,
			}),
		},
		operation_parameter_wrapper: Some(vec![
			OperationParameterWrapper::OperationParameter(Parameter {
				parameter_name: "X-axis translation".to_string(),
				parameter_value: -146.414,
				parameter_unit: None,
				identifier: None,
			}),
			OperationParameterWrapper::OperationParameter(Parameter {
				parameter_name: "Y-axis translation".to_string(),
				parameter_value: 507.337,
				parameter_unit: None,
				identifier: None,
			}),
			OperationParameterWrapper::OperationParameter(Parameter {
				parameter_name: "Z-axis translation".to_string(),
				parameter_value: 680.507,
				parameter_unit: None,
				identifier: None,
			}),
		]),
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: None,
			identifier: None,
			remark: None,
		},
	};

	let correct = vec![WktCrsTypes::AbridgedCoordinateTransformation(correct)];

	let wkt = parse_wkt_crs(EXAMPLE1).unwrap();

	assert_eq!(correct, wkt);
}
