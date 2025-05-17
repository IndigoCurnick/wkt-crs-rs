use log::warn;

use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    id::Id,
    keywords::{SCALEUNIT, UNIT},
};

#[derive(Debug, PartialEq)]
pub struct ScaleUnit {
    pub unit_name: String,
    pub conversion_factor: f64,
    pub identifier: Option<Id>, // TODO: Technically the spec allows for many IDs here
}

impl TryFrom<&WktNode> for ScaleUnit {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == SCALEUNIT || value.keyword == UNIT) {
            let expected = vec![SCALEUNIT.to_string(), UNIT.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if value.keyword == UNIT {
            warn!("Keyword SPHEROID depreciated. Consider using ELLIPSOID instead");
        }

        if !(value.args.len() == 2 || value.args.len() == 3) {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["2".to_string(), "3".to_string()].into(),
                found: value.args.len(),
            });
        }

        let unit_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let conversion_factor = match value.args[1] {
            WktArg::Number(n) => n,
            _ => return Err(WktParseError::ExpectedString),
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

        Ok(ScaleUnit {
            unit_name,
            conversion_factor,
            identifier,
        })
    }
}
