use std::{num::IntErrorKind, process::id};

use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::{CALENDAR, ID, TDATUM, TIMEDATUM, TIMEORIGIN},
    scope_extent_identifier_remark::Id,
    temporal_crs::{calendar::Calendar, temporal_origin::TemporalOrigin},
};

#[derive(Debug, PartialEq)]
pub struct TemporalDatum {
    pub datum_name: String,
    pub calendar: Option<Calendar>,
    pub temporal_origin: Option<TemporalOrigin>,
    pub identifier: Option<Id>,
}

impl TryFrom<&WktNode> for TemporalDatum {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == TDATUM || value.keyword == TIMEDATUM) {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![TDATUM.into(), TIMEDATUM.into()].into(),
                found: value.keyword.clone(),
            });
        }

        let datum_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        // TODO: arity?
        let mut calendar = None;
        let mut temporal_origin = None;
        let mut identifier = None;

        for i in 1..value.args.len() {
            match &value.args[i] {
                WktArg::Node(node) => match node.keyword.as_str() {
                    CALENDAR => {
                        if temporal_origin.is_some() || identifier.is_some() {
                            return Err(WktParseError::IncorrectKeywordOrder);
                        }

                        if calendar.is_some() {
                            return Err(WktParseError::TooManyKeyword(CALENDAR.into()));
                        }

                        calendar = Some(Calendar::try_from(node)?);
                    }
                    TIMEORIGIN => {
                        if identifier.is_some() {
                            return Err(WktParseError::IncorrectKeywordOrder);
                        }

                        if temporal_origin.is_some() {
                            return Err(WktParseError::TooManyKeyword(CALENDAR.into()));
                        }

                        temporal_origin = Some(TemporalOrigin::try_from(node)?);
                    }
                    ID => {
                        if identifier.is_some() {
                            return Err(WktParseError::TooManyKeyword(CALENDAR.into()));
                        }

                        identifier = Some(Id::try_from(node)?);
                    }
                    _ => {
                        return Err(WktParseError::IncorrectKeyword {
                            expected: vec![CALENDAR.into(), TIMEORIGIN.into(), ID.into()].into(),
                            found: node.keyword.clone(),
                        });
                    }
                },
                _ => return Err(WktParseError::ExpectedNode),
            }
        }

        return Ok(TemporalDatum {
            datum_name,
            calendar,
            temporal_origin,
            identifier,
        });
    }
}
