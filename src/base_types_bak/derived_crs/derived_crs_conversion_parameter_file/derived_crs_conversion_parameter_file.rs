use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::PARAMETERFILE,
    scope_extent_identifier_remark::Id,
};

#[derive(Debug, PartialEq)]
pub struct DerivedCrsConversionParameterFile {
    pub parameter_name: String,
    pub parameter_file_name: String,
    pub identifier: Option<Id>, // TODO: technically allowed multiple of these
}

impl TryFrom<&WktNode> for DerivedCrsConversionParameterFile {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == PARAMETERFILE) {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![PARAMETERFILE.into()].into(),
                found: value.keyword.clone(),
            });
        }

        if value.args.len() < 3 {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["3+".into()].into(),
                found: value.args.len(),
            });
        }

        let parameter_name = match &value.args[0] {
            WktArg::String(f) => f.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let parameter_file_name = match &value.args[1] {
            WktArg::String(f) => f.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let identifier = match value.args.get(1) {
            Some(x) => match x {
                WktArg::Node(n) => Some(Id::try_from(n)?),
                _ => return Err(WktParseError::ExpectedNode),
            },
            None => None,
        };

        return Ok(Self {
            parameter_name,
            parameter_file_name,
            identifier,
        });
    }
}
