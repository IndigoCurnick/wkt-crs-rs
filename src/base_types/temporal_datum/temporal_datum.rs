use crate::{
    arity::match_arity,
    ast::{Parse, WktArg, WktNode},
    base_types::{Calendar, Id, TimeOrigin},
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct TemporalDatum {
    pub datum_name: String,
    pub calendar: Option<Calendar>,
    pub temporal_origin: Option<TimeOrigin>,
    pub identifier: Option<Id>,
}

impl WktBaseType for TemporalDatum {
    fn from_nodes<'a, I>(
        wkt_nodes: I,
    ) -> Result<crate::types::WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        // Take 1

        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::TDatum, Keywords::TimeDatum])?;
        match_arity(node.args.len(), 1, 4)?;

        let datum_name = node.args[0].parse()?;

        let mut calendar = None;
        let mut temporal_origin = None;
        let mut identifier = None;

        for i in 1..node.args.len() {
            match &node.args[i] {
                WktArg::Node(this) => match this.keyword {
                    Keywords::Calendar => {
                        if temporal_origin.is_some() || identifier.is_some() {
                            return Err(WktParseError::IncorrectKeywordOrder);
                        }

                        if calendar.is_some() {
                            return Err(WktParseError::TooManyKeyword(Keywords::Calendar));
                        }

                        calendar = Some(this.parse()?);
                    }
                    Keywords::TimeOrigin => {
                        if identifier.is_some() {
                            return Err(WktParseError::IncorrectKeywordOrder);
                        }

                        if temporal_origin.is_some() {
                            return Err(WktParseError::TooManyKeyword(Keywords::TimeOrigin));
                        }

                        temporal_origin = Some(this.parse()?);
                    }
                    Keywords::Id => {
                        if identifier.is_some() {
                            return Err(WktParseError::TooManyKeyword(Keywords::Id));
                        }

                        identifier = Some(this.parse()?);
                    }
                    _ => {
                        return Err(WktParseError::IncorrectKeyword {
                            expected: vec![Keywords::Calendar, Keywords::TimeOrigin, Keywords::Id]
                                .into(),
                            found: this.keyword.clone(),
                        });
                    }
                },
                WktArg::Data(_) => return Err(WktParseError::ExpectedNode),
            }
        }

        let res = TemporalDatum {
            datum_name,
            calendar,
            temporal_origin,
            identifier,
        };

        return Ok(WktBaseTypeResult {
            result: res,
            consumed: 1,
        });
    }
}
