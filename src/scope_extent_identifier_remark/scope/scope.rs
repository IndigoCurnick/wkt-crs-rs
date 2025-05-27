use crate::{ast::WktArg, error::WktParseError, keywords::SCOPE};

#[derive(Debug, PartialEq)]
pub struct Scope(pub String);

impl TryFrom<&WktArg> for Scope {
    type Error = WktParseError;

    fn try_from(value: &WktArg) -> Result<Self, Self::Error> {
        let value = match value {
            WktArg::Node(node) => node,
            _ => return Err(WktParseError::ExpectedNode),
        };

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
