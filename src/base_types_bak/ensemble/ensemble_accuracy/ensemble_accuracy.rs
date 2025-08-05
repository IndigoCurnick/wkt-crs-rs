use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::ENSEMBLEACCURACY,
};

#[derive(Debug, PartialEq)]
pub struct EnsembleAccuracy(pub f64); // ? could this be an int?

impl TryFrom<&WktNode> for EnsembleAccuracy {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != ENSEMBLEACCURACY {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![ENSEMBLEACCURACY.into()].into(),
                found: value.keyword.clone(),
            });
        }

        if value.args.len() != 1 {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["1".into()].into(),
                found: value.args.len(),
            });
        }

        let acc = match value.args[0] {
            WktArg::Number(f) => f.clone(),
            _ => return Err(WktParseError::ExpectedNumber),
        };

        return Ok(EnsembleAccuracy(acc));
    }
}
