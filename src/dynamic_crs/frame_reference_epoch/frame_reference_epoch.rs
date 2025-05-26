use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::FRAMEEPOCH,
};

#[derive(Debug, PartialEq)]
pub struct FrameReferenceEpoch(pub f64);

impl TryFrom<&WktNode> for FrameReferenceEpoch {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != FRAMEEPOCH {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![FRAMEEPOCH.into()].into(),
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
            WktArg::Number(f) => f.clone(),
            _ => return Err(WktParseError::ExpectedNumber),
        };

        return Ok(FrameReferenceEpoch(fr));
    }
}
