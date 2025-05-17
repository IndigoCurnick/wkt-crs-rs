use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::ORDER,
};

#[derive(Debug, PartialEq)]
pub struct AxisOrder {
    pub bearing: f64, // TODO: Technically this should only ever be a uint
}

impl TryFrom<&WktNode> for AxisOrder {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != ORDER {
            let expected = vec![ORDER.to_string()];
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

        let bearing = match &value.args[0] {
            WktArg::Number(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedNumber),
        };

        return Ok(AxisOrder { bearing });
    }
}
