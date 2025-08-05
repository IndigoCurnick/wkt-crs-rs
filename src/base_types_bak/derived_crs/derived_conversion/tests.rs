use crate::{
    ast::parse_wkt,
    derived_crs::{
        derived_conversion::derived_conversion::{DerivingConversion, OperationParameterWrapper},
        derived_crs_conversion_parameter::DerivedCrsConversionParameter,
        operation_method::OperationMethod,
    },
    scope_extent_identifier_remark::Id,
    units::{AngleUnit, SpatialUnit, Unit},
    utils::NumText,
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
                authority_unique_identifier: NumText::Num(123.0),
                version: None,
                authority_citation: None,
                id_uri: None,
            }),
        },
        operation_parameter: Some(vec![
            OperationParameterWrapper::OperationParameter(DerivedCrsConversionParameter {
                parameter_name: "parameter 1 name".to_string(),
                parameter_value: 0.0,
                parameter_unit: Unit::SpatialUnit(SpatialUnit::AngleUnit(AngleUnit {
                    unit_name: "degree".to_string(),
                    conversion_factor: 0.017,
                    identifier: None,
                })),
                identifier: Some(Id {
                    authority_name: "authority".to_string(),
                    authority_unique_identifier: NumText::Num(456.0),
                    version: None,
                    authority_citation: None,
                    id_uri: None,
                }),
            }),
            OperationParameterWrapper::OperationParameter(DerivedCrsConversionParameter {
                parameter_name: "parameter 2 name".to_string(),
                parameter_value: -123.0,
                parameter_unit: Unit::SpatialUnit(SpatialUnit::AngleUnit(AngleUnit {
                    unit_name: "degree".to_string(),
                    conversion_factor: 0.017,
                    identifier: None,
                })),
                identifier: Some(Id {
                    authority_name: "authority".to_string(),
                    authority_unique_identifier: NumText::Num(789.0),
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

    let derived = DerivingConversion::try_from(&ast[0]).unwrap();

    assert_eq!(correct, derived);
}
