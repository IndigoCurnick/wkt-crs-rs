use horologium::Temporal;

use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::TIMEEXTENT,
};

pub enum DateOrString {
    Date(Temporal),
    String(String),
}

pub struct TemporalExtent {
    pub from: DateOrString,
    pub to: DateOrString,
}

impl TryFrom<&WktNode> for TemporalExtent {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != TIMEEXTENT {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![TIMEEXTENT.into()].into(),
                found: value.keyword.clone(),
            });
        }

        if value.args.len() != 2 {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["2".into()].into(),
                found: value.args.len(),
            });
        }

        let from = match &value.args[0] {
            WktArg::String(s) => DateOrString::String(s.clone()),
            WktArg::DateTime(t) => DateOrString::Date(t.clone()),
            _ => return Err(WktParseError::ExpectedStringOrDate),
        };

        let to = match &value.args[1] {
            WktArg::String(s) => DateOrString::String(s.clone()),
            WktArg::DateTime(t) => DateOrString::Date(t.clone()),
            _ => return Err(WktParseError::ExpectedStringOrDate),
        };

        return Ok(TemporalExtent { from, to });
    }
}
