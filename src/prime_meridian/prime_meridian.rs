use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    id::Id,
    keywords::{ANGLEUNIT, ID, PRIMEM, PRIMEMERIDIAN},
    units::AngleUnit,
};

pub struct PrimeMeridian {
    pub prime_meridian_name: String,
    pub irm_longitude: f64,
    pub angle_unit: Option<AngleUnit>, // * Note that in the spec, `irm_longitude` and `angle_unit` are wrapped into one flat group
    pub identifier: Option<Id>,
}

impl TryFrom<&WktNode> for PrimeMeridian {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == PRIMEMERIDIAN || value.keyword == PRIMEM) {
            let expected = vec![PRIMEM.to_string(), PRIMEMERIDIAN.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if !(value.args.len() == 2 || value.args.len() == 3 || value.args.len() == 4) {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["2".to_string(), "3".to_string(), "4".to_string()].into(),
                found: value.args.len(),
            });
        }

        let prime_meridian_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let irm_longitude = match &value.args[1] {
            WktArg::Number(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedNumber),
        };

        let mut angle_unit = None;
        let mut identifier = None;

        for i in 2..value.args.len() {
            match &value.args[i] {
                WktArg::Node(node) => {
                    match node.keyword.as_str() {
                        ANGLEUNIT => {
                            // Angleunit must be before id
                            if identifier.is_some() {
                                return Err(WktParseError::IncorrectKeywordOrder);
                            }

                            // Can only be one angle unit
                            if angle_unit.is_some() {
                                return Err(WktParseError::TooManyKeyword(ANGLEUNIT.to_string()));
                            }

                            angle_unit = Some(AngleUnit::try_from(node)?);
                        }
                        ID => {}
                        _ => {
                            return Err(WktParseError::IncorrectKeyword {
                                expected: vec![ANGLEUNIT.to_string(), ID.to_string()].into(),
                                found: node.keyword.clone(),
                            });
                        }
                    }
                }
                _ => return Err(WktParseError::ExpectedNode),
            }
        }

        todo!();
    }
}
