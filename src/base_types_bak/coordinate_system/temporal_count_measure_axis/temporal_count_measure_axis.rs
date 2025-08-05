// 7.5 Coordinate System
// 7.5.1 Syntax

use crate::{
    ast::{WktArg, WktNode},
    coordinate_system::{axis_direction::AxisDirection, axis_order::AxisOrder},
    error::WktParseError,
    keywords::{AXIS, ID, ORDER, TEMPORALQUANTITY, TIMEUNIT},
    scope_extent_identifier_remark::Id,
    units::TimeUnit,
};

#[derive(Debug, PartialEq)]
pub struct TemporalCountMeasureAxis {
    pub axis_name_abbreviation: String,
    pub axis_direction: AxisDirection,
    pub axis_order: Option<AxisOrder>,
    pub time_unit: Option<TimeUnit>,
    pub identifier: Option<Id>, // TODO: Technically the spec allows for multiple of these
}

impl TryFrom<&WktNode> for TemporalCountMeasureAxis {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != AXIS {
            let expected = vec![AXIS.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if !(value.args.len() >= 2 || value.args.len() <= 6) {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["2-6".to_string()].into(),
                found: value.args.len(),
            });
        }

        let axis_name_abbreviation = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let axis_direction = AxisDirection::try_from((&value.args[1], value.args.get(2)))?;

        if value.args.len() == 2 {
            return Ok(TemporalCountMeasureAxis {
                axis_name_abbreviation,
                axis_direction,
                axis_order: None,
                time_unit: None,
                identifier: None,
            });
        }

        let mut axis_order = None;
        let mut time_unit = None;
        let mut identifier = None;

        for i in 2..value.args.len() {
            let this_value = &value.args[i];

            match this_value {
                WktArg::Node(node) => {
                    match node.keyword.as_str() {
                        ORDER => {
                            // Axis order comes first

                            if time_unit.is_some() || identifier.is_some() {
                                return Err(WktParseError::IncorrectKeywordOrder);
                            }

                            axis_order = Some(AxisOrder::try_from(node)?);
                        }
                        TIMEUNIT | TEMPORALQUANTITY => {
                            // Spatial unit comes before identifier

                            if identifier.is_some() {
                                return Err(WktParseError::IncorrectKeywordOrder);
                            }

                            time_unit = Some(TimeUnit::try_from(node)?);
                        }
                        ID => {
                            identifier = Some(Id::try_from(node)?);
                        }
                        _ => {
                            return Err(WktParseError::IncorrectKeyword {
                                expected: vec![
                                    ORDER.to_string(),
                                    TIMEUNIT.to_string(),
                                    TEMPORALQUANTITY.to_string(),
                                    ID.to_string(),
                                ]
                                .into(),
                                found: node.keyword.to_string(),
                            });
                        }
                    }
                }
                _ => return Err(WktParseError::ExpectedNode),
            }
        }

        Ok(TemporalCountMeasureAxis {
            axis_name_abbreviation,
            axis_direction,
            axis_order,
            time_unit,
            identifier,
        })
    }
}
