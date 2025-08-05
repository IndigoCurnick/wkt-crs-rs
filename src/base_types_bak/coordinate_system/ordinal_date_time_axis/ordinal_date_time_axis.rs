use crate::{
    ast::{WktArg, WktNode},
    coordinate_system::{axis_direction::AxisDirection, axis_order::AxisOrder},
    error::WktParseError,
    keywords::{AXIS, ID, ORDER},
    scope_extent_identifier_remark::Id,
};

#[derive(Debug, PartialEq)]
pub struct OrdinalDateTimeAxis {
    pub axis_name_abbreviation: String,
    pub axis_direction: AxisDirection,
    pub axis_order: Option<AxisOrder>,
    pub identifier: Option<Id>,
}

impl TryFrom<&WktNode> for OrdinalDateTimeAxis {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != AXIS {
            let expected = vec![AXIS.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if !(value.args.len() >= 2 || value.args.len() <= 5) {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["2-5".to_string()].into(),
                found: value.args.len(),
            });
        }

        let axis_name_abbreviation = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let axis_direction = AxisDirection::try_from((&value.args[1], value.args.get(2)))?;

        if value.args.len() == 2 {
            return Ok(OrdinalDateTimeAxis {
                axis_name_abbreviation,
                axis_direction,
                axis_order: None,
                identifier: None,
            });
        }

        // If Axis direction used 2 args then we start from 3
        let i: usize = match value.args[2] {
            WktArg::String(_) => 3,
            _ => 2,
        };

        let mut axis_order = None;
        let mut identifier = None;

        for j in i..value.args.len() {
            match &value.args[j] {
                WktArg::Node(n) => {
                    match n.keyword.as_str() {
                        ORDER => {
                            // ? Should handle more than one axis order?
                            // Order comes before identifier
                            if identifier.is_some() {
                                return Err(WktParseError::IncorrectKeywordOrder);
                            }

                            axis_order = Some(AxisOrder::try_from(n)?);
                        }
                        ID => {
                            // ? should handle more than one identifier?
                            identifier = Some(Id::try_from(n)?)
                        }
                        _ => {
                            return Err(WktParseError::IncorrectKeyword {
                                expected: vec![ORDER.to_string(), ID.to_string()].into(),
                                found: n.keyword.clone(),
                            });
                        }
                    }
                }
                _ => return Err(WktParseError::ExpectedNode),
            }
        }

        Ok(OrdinalDateTimeAxis {
            axis_name_abbreviation,
            axis_direction,
            axis_order,
            identifier,
        })
    }
}
