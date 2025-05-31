use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::GEOIDMODEL,
    scope_extent_identifier_remark::Id,
};

#[derive(Debug, PartialEq)]
pub struct GeoidModelId {
    pub geoid_model_name: String,
    pub identifier: Option<Id>,
}

impl TryFrom<&WktNode> for GeoidModelId {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != GEOIDMODEL {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![GEOIDMODEL.into()].into(),
                found: value.keyword.clone(),
            });
        }

        if !(value.args.len() == 1 || value.args.len() == 2) {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["1".into(), "2".into()].into(),
                found: value.args.len(),
            });
        }

        let geoid_model_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let identifier = match value.args.get(1) {
            Some(node) => match node {
                WktArg::Node(n) => Some(Id::try_from(n)?),
                _ => return Err(WktParseError::ExpectedNode),
            },
            None => None,
        };

        return Ok(GeoidModelId {
            geoid_model_name,
            identifier,
        });
    }
}
