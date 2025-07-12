use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::PARAMETER,
    scope_extent_identifier_remark::Id,
    units::Unit,
};

#[derive(Debug, PartialEq)]
pub struct DerivedCrsConversionParameter {
    pub parameter_name: String,
    pub parameter_value: f64,
    pub parameter_unit: Unit,
    pub identifier: Option<Id>, // TODO: technically allowed multiple of these
}

impl TryFrom<&WktNode> for DerivedCrsConversionParameter {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == PARAMETER) {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![PARAMETER.into()].into(),
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

        let parameter_value = match &value.args[1] {
            WktArg::Number(f) => f.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let parameter_unit = match &value.args[2] {
            WktArg::Node(n) => Unit::try_from(n)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let identifier = match value.args.get(3) {
            Some(x) => match x {
                WktArg::Node(n) => Some(Id::try_from(n)?),
                _ => return Err(WktParseError::ExpectedNode),
            },
            None => None,
        };

        return Ok(Self {
            parameter_name,
            parameter_value,
            parameter_unit,
            identifier,
        });
    }
}
