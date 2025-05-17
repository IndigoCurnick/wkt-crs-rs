// Section 8.2.1

use log::warn;

use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    units::LengthUnit,
};

use super::keywords::{ELLIPSOID, SPHEROID};

#[derive(Debug, PartialEq)]
pub struct Ellipsoid {
    pub ellipsoid_name: String,
    pub semi_major_axis: f64,
    pub inverse_flattening: f64,
    pub length_unit: Option<LengthUnit>,
}

impl TryFrom<&WktNode> for Ellipsoid {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == ELLIPSOID || value.keyword == SPHEROID) {
            let expected = vec![ELLIPSOID.to_string(), SPHEROID.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if value.keyword == SPHEROID {
            warn!("Keyword SPHEROID depreciated. Consider using ELLIPSOID instead");
        }

        if !(value.args.len() == 3 || value.args.len() == 4) {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["3".to_string(), "4".to_string()].into(),
                found: value.args.len(),
            });
        }

        let ellipsoid_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let semi_major_axis = match value.args[1] {
            WktArg::Number(n) => n,
            _ => return Err(WktParseError::ExpectedString),
        };

        let inverse_flattening = match value.args[2] {
            WktArg::Number(n) => n,
            _ => return Err(WktParseError::ExpectedNumber),
        };

        let length_unit = match value.args.get(3) {
            Some(x) => match x {
                WktArg::Node(node) => match LengthUnit::try_from(node) {
                    Ok(lu) => Some(lu),
                    Err(y) => return Err(y),
                },
                _ => return Err(WktParseError::ExpectedNode),
            },
            None => None,
        };

        Ok(Ellipsoid {
            ellipsoid_name,
            semi_major_axis,
            inverse_flattening,
            length_unit,
        })
    }
}
