use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::{MODEL, VELOCITYGRID},
};

#[derive(Debug, PartialEq)]
pub struct DeformationModel(pub String);

impl TryFrom<&WktNode> for DeformationModel {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == MODEL || value.keyword == VELOCITYGRID) {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![MODEL.into(), VELOCITYGRID.into()].into(),
                found: value.keyword.clone(),
            });
        }

        if value.args.len() != 1 {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["1".into()].into(),
                found: value.args.len(),
            });
        }

        let fr = match &value.args[0] {
            WktArg::String(f) => f.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        return Ok(DeformationModel(fr));
    }
}
