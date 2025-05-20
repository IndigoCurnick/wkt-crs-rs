use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::REMARK,
};

#[derive(Debug, PartialEq)]
pub struct Remark {
    pub remark: String,
}

impl TryFrom<&WktNode> for Remark {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != REMARK {
            let expected = vec![REMARK.to_string()];
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

        let remark = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        return Ok(Remark { remark });
    }
}
