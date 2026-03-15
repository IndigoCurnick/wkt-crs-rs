use crate::{
	ast::parse_wkt,
	base_types::{
		AbridgedCoordinateTransformation, Id, OperationMethod,
		OperationParameter,
	},
	compound_types::ScopeExtentIdentifierRemark,
	data_types::NumText,
	enumerations::OperationParameterWrapper,
	types::WktBaseType,
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
		operation_method: OperationMethod {
			operation_method_name: "Geocentric translations".to_string(),
			identifier: Some(Id {
				authority_name: "EPSG".to_string(),
				authority_unique_identifier: NumText::Int(1031),
				version: None,
				authority_citation: None,
				id_uri: None,
			}),
		},
		operation_parameter_wrapper: Some(vec![
			OperationParameterWrapper::OperationParameter(OperationParameter {
				parameter_name: "X-axis translation".to_string(),
				parameter_value: -146.414,
				parameter_unit: None,
				identifier: None,
			}),
			OperationParameterWrapper::OperationParameter(OperationParameter {
				parameter_name: "Y-axis translation".to_string(),
				parameter_value: 507.337,
				parameter_unit: None,
				identifier: None,
			}),
			OperationParameterWrapper::OperationParameter(OperationParameter {
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

	let ast = parse_wkt(EXAMPLE1);

	assert_eq!(ast.len(), 1);

	let order = AbridgedCoordinateTransformation::from_nodes(&ast).unwrap();

	assert_eq!(correct, order.result);
}
