use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::SCOPE,
};

#[derive(Debug, PartialEq)]
pub struct Scope(pub String);

impl TryFrom<&WktNode> for Scope {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != SCOPE {
            let expected = vec![SCOPE.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if value.args.len() != 1 {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["1".to_string()].into(),
                found: value.args.len(),
            });
        }

        let scope = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        Ok(Scope(scope))
    }
}
