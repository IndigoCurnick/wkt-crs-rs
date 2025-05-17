use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    id::Id,
    keywords::{
        ANGLEUNIT, AXIS, CS, ID, LENGTHUNIT, PARAMETRICUNIT, SCALEUNIT, TEMPORALQUANTITY, TIMEUNIT,
    },
    units::Unit,
};

use super::{spatial_axis::SpatialAxis, spatial_cs_type::SpatialCsType};

pub struct SpatialCoordinateSystem {
    pub spatial_cs_type: SpatialCsType,
    pub dimension: Dimension,
    pub identifier: Option<Id>, // TODO: Technically the spec allows for any number of these
    pub spatial_axis: Vec<SpatialAxis>,
    pub cs_unit: Option<Unit>,
}

pub enum Dimension {
    One,
    Two,
    Three,
}

impl TryFrom<f64> for Dimension {
    type Error = WktParseError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        return match value {
            1.0 => Ok(Dimension::One),
            2.0 => Ok(Dimension::Two),
            3.0 => Ok(Dimension::Three),
            _ => Err(WktParseError::IncorrectValue),
        };
    }
}

impl TryFrom<&WktArg> for Dimension {
    type Error = WktParseError;

    fn try_from(value: &WktArg) -> Result<Self, Self::Error> {
        return match value {
            WktArg::Number(n) => Ok(Dimension::try_from(*n)?),
            _ => return Err(WktParseError::ExpectedNumber),
        };
    }
}

impl TryFrom<&WktNode> for SpatialCoordinateSystem {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != CS {
            let expected = vec![CS.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if !(value.args.len() >= 3) {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["3+".to_string()].into(),
                found: value.args.len(),
            });
        }

        let spatial_cs_type = SpatialCsType::try_from(&value.args[0])?;
        let dimension = Dimension::try_from(&value.args[1])?;

        let mut identifier = None;
        let mut spatial_axis = vec![];
        let mut cs_unit = None;

        for i in 2..value.args.len() {
            let this_arg = &value.args[i];

            match this_arg {
                WktArg::Node(node) => match node.keyword.as_str() {
                    ID => {
                        // ID is before spatial axis and cs unti

                        if !spatial_axis.is_empty() || cs_unit.is_some() {
                            return Err(WktParseError::IncorrectKeywordOrder);
                        }

                        if identifier.is_some() {
                            return Err(WktParseError::TooManyKeyword(ID.to_string()));
                        }

                        identifier = Some(Id::try_from(node)?);
                    }
                    AXIS => {
                        // Spatial axis is before cs_unit

                        if cs_unit.is_some() {
                            return Err(WktParseError::IncorrectKeywordOrder);
                        }

                        let sa = SpatialAxis::try_from(node)?;
                        spatial_axis.push(sa);
                    }
                    LENGTHUNIT | ANGLEUNIT | SCALEUNIT | PARAMETRICUNIT | TIMEUNIT
                    | TEMPORALQUANTITY => {
                        cs_unit = Some(Unit::try_from(node)?);
                    }
                    _ => {
                        return Err(WktParseError::IncorrectKeyword {
                            expected: vec![
                                ID.to_string(),
                                AXIS.to_string(),
                                LENGTHUNIT.to_string(),
                                ANGLEUNIT.to_string(),
                                SCALEUNIT.to_string(),
                                PARAMETRICUNIT.to_string(),
                                TIMEUNIT.to_string(),
                                TEMPORALQUANTITY.to_string(),
                            ]
                            .into(),
                            found: node.keyword.clone(),
                        });
                    }
                },
                _ => return Err(WktParseError::ExpectedNode),
            }
        }

        // Must be at least one spatial axis

        if spatial_axis.is_empty() {
            return Err(WktParseError::TooFewKeyword(AXIS.to_string()));
        }

        Ok(SpatialCoordinateSystem {
            spatial_cs_type,
            dimension,
            identifier,
            spatial_axis,
            cs_unit,
        })
    }
}
