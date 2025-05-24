use horologium::Temporal;

use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::{TIMEEXTENT, VERTICALEXTENT},
    units::LengthUnit,
};

#[derive(Debug, PartialEq)]
pub enum DateOrString {
    Date(Temporal),
    String(String),
}

#[derive(Debug, PartialEq)]
pub struct VerticalExtent {
    pub minimum_height: f64,
    pub maximum_height: f64,
    pub length_unit: Option<LengthUnit>,
}

impl TryFrom<&WktNode> for VerticalExtent {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != VERTICALEXTENT {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![VERTICALEXTENT.into()].into(),
                found: value.keyword.clone(),
            });
        }

        if value.args.len() == 1 || value.args.len() > 3 {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["2".into(), "3".into()].into(),
                found: value.args.len(),
            });
        }

        let minimum_height = match &value.args[0] {
            WktArg::Number(n) => n.clone(),
            _ => return Err(WktParseError::ExpectedNumber),
        };

        let maximum_height = match &value.args[1] {
            WktArg::Number(n) => n.clone(),
            _ => return Err(WktParseError::ExpectedNumber),
        };

        let length_unit = match value.args.get(2) {
            Some(x) => match x {
                WktArg::Node(n) => Some(LengthUnit::try_from(n)?),
                _ => return Err(WktParseError::ExpectedNode),
            },
            None => None,
        };

        return Ok(VerticalExtent {
            minimum_height,
            maximum_height,
            length_unit,
        });
    }
}
