use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::ANCHOR,
};

#[derive(Debug, PartialEq)]
pub struct DatumAnchor(pub String);

impl TryFrom<&WktNode> for DatumAnchor {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != ANCHOR {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![ANCHOR.into()].into(),
                found: value.keyword.clone(),
            });
        }

        if value.args.len() != 1 {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["1".into()].into(),
                found: value.args.len(),
            });
        }

        let anchor = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        return Ok(DatumAnchor(anchor));
    }
}
