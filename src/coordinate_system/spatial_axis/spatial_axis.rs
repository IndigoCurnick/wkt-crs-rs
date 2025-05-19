use crate::{
    ast::{WktArg, WktNode},
    coordinate_system::{axis_direction::AxisDirection, axis_order::AxisOrder},
    error::WktParseError,
    id::Id,
    keywords::{ANGLEUNIT, AXIS, ID, LENGTHUNIT, ORDER, PARAMETRICUNIT, SCALEUNIT},
    units::SpatialUnit,
};

pub struct SpatialAxis {
    pub axis_name_abbreviation: String,
    pub axis_direction: AxisDirection,
    pub axis_order: Option<AxisOrder>, // ? See 7.5.4 for constraints
    pub spatial_unit: Option<SpatialUnit>,
    pub identifier: Option<Id>, // TODO: technically the spec allows for multiple
}

impl TryFrom<&WktNode> for SpatialAxis {
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
            return Ok(SpatialAxis {
                axis_name_abbreviation,
                axis_direction,
                axis_order: None,
                spatial_unit: None,
                identifier: None,
            });
        }

        let mut axis_order = None;
        let mut spatial_unit = None;
        let mut identifier = None;

        for i in 2..value.args.len() {
            let this_value = &value.args[i];

            match this_value {
                WktArg::Node(node) => {
                    match node.keyword.as_str() {
                        ORDER => {
                            // Axis order comes first

                            if spatial_unit.is_some() || identifier.is_some() {
                                return Err(WktParseError::IncorrectKeywordOrder);
                            }

                            axis_order = Some(AxisOrder::try_from(node)?);
                        }
                        LENGTHUNIT | ANGLEUNIT | SCALEUNIT | PARAMETRICUNIT => {
                            // Spatial unit comes before identifier

                            if identifier.is_some() {
                                return Err(WktParseError::IncorrectKeywordOrder);
                            }

                            spatial_unit = Some(SpatialUnit::try_from(node)?);
                        }
                        ID => {
                            identifier = Some(Id::try_from(node)?);
                        }
                        _ => {
                            return Err(WktParseError::IncorrectKeyword {
                                expected: vec![
                                    ORDER.to_string(),
                                    LENGTHUNIT.to_string(),
                                    ANGLEUNIT.to_string(),
                                    SCALEUNIT.to_string(),
                                    PARAMETRICUNIT.to_string(),
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

        Ok(SpatialAxis {
            axis_name_abbreviation,
            axis_direction,
            axis_order,
            spatial_unit,
            identifier,
        })
    }
}
