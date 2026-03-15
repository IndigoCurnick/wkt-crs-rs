use crate::{
	ast::parse_wkt,
	base_types::{
		AngleUnit, Id, OperationMethod, Parameter,
		deriving_conversion::DerivingConversion,
	},
	compound_types::{SpatialUnit, Unit},
	data_types::NumText,
	enumerations::OperationParameterWrapper,
	types::WktBaseType,
};

const EXAMPLE1: &str = r#"DERIVINGCONVERSION["conversion name",
    METHOD["method name",ID["authority",123]],
    PARAMETER["parameter 1 name",0,
        ANGLEUNIT["degree",0.017],
        ID["authority",456]
    ],
    PARAMETER["parameter 2 name",-123,
        ANGLEUNIT["degree",0.017],
        ID["authority",789]
    ]
]
"#;

#[test]
fn test_derived_conversion() {
	let correct = DerivingConversion {
		deriving_conversion_name: "conversion name".to_string(),
		operation_method: OperationMethod {
			operation_method_name: "method name".to_string(),
			identifier: Some(Id {
				authority_name: "authority".to_string(),
				authority_unique_identifier: NumText::Int(123),
				version: None,
				authority_citation: None,
				id_uri: None,
			}),
		},
		operation_parameter: Some(vec![
			OperationParameterWrapper::OperationParameter(Parameter {
				parameter_name: "parameter 1 name".to_string(),
				parameter_value: 0.0,
				parameter_unit: Some(Unit::SpatialUnit(
					SpatialUnit::AngleUnit(AngleUnit {
						unit_name: "degree".to_string(),
						conversion_factor: 0.017,
						identifier: None,
					}),
				)),
				identifier: Some(Id {
					authority_name: "authority".to_string(),
					authority_unique_identifier: NumText::Int(456),
					version: None,
					authority_citation: None,
					id_uri: None,
				}),
			}),
			OperationParameterWrapper::OperationParameter(Parameter {
				parameter_name: "parameter 2 name".to_string(),
				parameter_value: -123.0,
				parameter_unit: Some(Unit::SpatialUnit(
					SpatialUnit::AngleUnit(AngleUnit {
						unit_name: "degree".to_string(),
						conversion_factor: 0.017,
						identifier: None,
					}),
				)),
				identifier: Some(Id {
					authority_name: "authority".to_string(),
					authority_unique_identifier: NumText::Int(789),
					version: None,
					authority_citation: None,
					id_uri: None,
				}),
			}),
		]),
		identifier: None,
	};

	let ast = parse_wkt(EXAMPLE1);

	assert_eq!(ast.len(), 1);

	let derived = DerivingConversion::from_nodes(&ast).unwrap();

	assert_eq!(correct, derived.result);
}
