use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::BASETIMECRS,
    scope_extent_identifier_remark::Id,
    temporal_crs::TemporalDatum,
};

#[derive(Debug, PartialEq)]
pub struct BaseTemporalCrs {
    pub base_crs_name: String,
    pub temporal_datum: TemporalDatum,
    pub identifier: Option<Id>, // TODO: Technically allowed multiple
}

impl TryFrom<&WktNode> for BaseTemporalCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != BASETIMECRS {
            let expected = vec![BASETIMECRS.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if !(value.args.len() >= 2) {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["2+".to_string()].into(),
                found: value.args.len(),
            });
        }

        let base_crs_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let temporal_datum = match &value.args[1] {
            WktArg::Node(x) => TemporalDatum::try_from(x)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let identifier = match value.args.get(2) {
            Some(x) => match x {
                WktArg::Node(y) => Some(Id::try_from(y)?),
                _ => return Err(WktParseError::ExpectedNode),
            },
            None => None,
        };

        return Ok(Self {
            base_crs_name,
            temporal_datum,
            identifier,
        });
    }
}
