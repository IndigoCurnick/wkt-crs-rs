use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::METHOD,
    scope_extent_identifier_remark::Id,
};

#[derive(Debug, PartialEq)]
pub struct OperationMethod {
    pub operation_method_name: String,
    pub identifier: Option<Id>, // TODO: technically allowed multiple of these
}

impl TryFrom<&WktNode> for OperationMethod {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == METHOD) {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![METHOD.into()].into(),
                found: value.keyword.clone(),
            });
        }

        if value.args.len() < 1 {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["1+".into()].into(),
                found: value.args.len(),
            });
        }

        let operation_method_name = match &value.args[0] {
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
            operation_method_name,
            identifier,
        });
    }
}
