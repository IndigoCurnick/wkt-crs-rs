use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::TIMEORIGIN,
    scope_extent_identifier_remark::DateOrString,
};

pub struct TemporalOrigin(pub DateOrString);

impl TryFrom<&WktNode> for TemporalOrigin {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != TIMEORIGIN {
            let expected = vec![TIMEORIGIN.to_string()];
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

        let cal = match &value.args[0] {
            WktArg::String(s) => DateOrString::String(s.clone()),
            WktArg::DateTime(t) => DateOrString::Date(t.clone()),
            _ => return Err(WktParseError::ExpectedString),
        };

        Ok(TemporalOrigin(cal))
    }
}
