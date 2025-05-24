use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::{TEMPORALQUANTITY, TIMEUNIT},
    scope_extent_identifier_remark::Id,
};

#[derive(Debug, PartialEq)]
pub struct TimeUnit {
    pub unit_name: String,
    pub conversion_factor: Option<f64>,
    pub identifier: Option<Id>, // TODO: Technically the Specification allows for multiple
}

impl TryFrom<&WktNode> for TimeUnit {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == TIMEUNIT || value.keyword == TEMPORALQUANTITY) {
            let expected = vec![TIMEUNIT.to_string(), TEMPORALQUANTITY.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if !(value.args.len() == 1 || value.args.len() == 2 || value.args.len() == 3) {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["1".to_string(), "2".to_string(), "3".to_string()].into(),
                found: value.args.len(),
            });
        }

        let unit_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let conversion_factor = match value.args.get(1) {
            Some(x) => match x {
                WktArg::Number(n) => Some(n.clone()),
                _ => return Err(WktParseError::ExpectedNumber),
            },
            None => None,
        };

        let identifier = match value.args.get(2) {
            Some(x) => match x {
                WktArg::Node(node) => match Id::try_from(node) {
                    Ok(lu) => Some(lu),
                    Err(y) => return Err(y),
                },
                _ => return Err(WktParseError::ExpectedNode),
            },
            None => None,
        };

        Ok(TimeUnit {
            unit_name,
            conversion_factor,
            identifier,
        })
    }
}
