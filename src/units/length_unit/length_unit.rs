// 7.4.1

use log::warn;

use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::{LENGTHUNIT, UNIT},
};

// TODO: Can take an ID too optionally
#[derive(Debug, PartialEq)]
pub struct LengthUnit {
    pub unit_name: String,
    pub conversion_factor: f64,
}

impl TryFrom<&WktNode> for LengthUnit {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == LENGTHUNIT || value.keyword == UNIT) {
            let expected = vec![LENGTHUNIT.to_string(), UNIT.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if value.keyword == UNIT {
            warn!("Keyword UNIT depreciated. Consider using LENGTHUNIT instead");
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
            _ => return Err(WktParseError::ExpectedNumber),
        };

        Ok(LengthUnit {
            unit_name,
            conversion_factor,
        })
    }
}
