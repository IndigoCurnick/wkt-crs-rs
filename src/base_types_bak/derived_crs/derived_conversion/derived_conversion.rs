use crate::{
    ast::{WktArg, WktNode},
    derived_crs::{
        derived_crs_conversion_parameter::DerivedCrsConversionParameter,
        derived_crs_conversion_parameter_file::DerivedCrsConversionParameterFile,
        operation_method::OperationMethod,
    },
    error::WktParseError,
    keywords::{DERIVINGCONVERSION, ID, PARAMETER, PARAMETERFILE},
    scope_extent_identifier_remark::Id,
};

#[derive(Debug, PartialEq)]
pub struct DerivingConversion {
    pub deriving_conversion_name: String,
    pub operation_method: OperationMethod,
    pub operation_parameter: Option<Vec<OperationParameterWrapper>>,
    pub identifier: Option<Id>, // TODO: allow multiple
}

impl TryFrom<&WktNode> for DerivingConversion {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == DERIVINGCONVERSION) {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![DERIVINGCONVERSION.into()].into(),
                found: value.keyword.clone(),
            });
        }

        if value.args.len() < 3 {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["3+".into()].into(),
                found: value.args.len(),
            });
        }

        let deriving_conversion_name = match &value.args[0] {
            WktArg::String(f) => f.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let operation_method = match &value.args[1] {
            WktArg::Node(n) => OperationMethod::try_from(n)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let mut operation_parameters = vec![];
        let mut identifier = None;

        for i in 2..value.args.len() {
            let this_node = match &value.args[i] {
                WktArg::Node(n) => n,
                _ => return Err(WktParseError::ExpectedNode),
            };

            match this_node.keyword.as_str() {
                PARAMETER | PARAMETERFILE => {
                    // Before identifier
                    if identifier.is_some() {
                        return Err(WktParseError::IncorrectKeywordOrder);
                    }

                    operation_parameters.push(OperationParameterWrapper::try_from(this_node)?);
                }
                ID => {
                    if identifier.is_some() {
                        return Err(WktParseError::TooManyKeyword(ID.to_string()));
                    }
                    identifier = Some(Id::try_from(this_node)?)
                }
                _ => {
                    return Err(WktParseError::IncorrectKeyword {
                        expected: vec![PARAMETER.into(), PARAMETERFILE.to_string(), ID.into()]
                            .into(),
                        found: this_node.keyword.clone(),
                    });
                }
            }
        }

        let operation_parameter = if operation_parameters.is_empty() {
            None
        } else {
            Some(operation_parameters)
        };

        return Ok(Self {
            deriving_conversion_name,
            operation_method,
            operation_parameter,
            identifier,
        });
    }
}

#[derive(Debug, PartialEq)]
pub enum OperationParameterWrapper {
    OperationParameter(DerivedCrsConversionParameter),
    OperationParameterFile(DerivedCrsConversionParameterFile),
}

impl TryFrom<&WktNode> for OperationParameterWrapper {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        match value.keyword.as_str() {
            PARAMETER => Ok(Self::OperationParameter(
                DerivedCrsConversionParameter::try_from(value)?,
            )),
            PARAMETERFILE => Ok(Self::OperationParameterFile(
                DerivedCrsConversionParameterFile::try_from(value)?,
            )),
            _ => Err(WktParseError::IncorrectKeyword {
                expected: vec![PARAMETER.into(), PARAMETERFILE.into()].into(),
                found: value.keyword.clone(),
            }),
        }
    }
}
