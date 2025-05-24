use log::error;

use crate::{
    ast::WktNode,
    error::WktParseError,
    keywords::{ANGLEUNIT, LENGTHUNIT, PARAMETRICUNIT, SCALEUNIT, UNIT},
    units::{AngleUnit, LengthUnit, ParametricUnit, ScaleUnit},
};

pub enum SpatialUnit {
    AngleUnit(AngleUnit),
    LengthUnit(LengthUnit),
    ParametricUnit(ParametricUnit),
    ScaleUnit(ScaleUnit),
}

impl TryFrom<&WktNode> for SpatialUnit {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        match value.keyword.as_str() {
            LENGTHUNIT => {
                return match LengthUnit::try_from(value) {
                    Ok(x) => Ok(Self::LengthUnit(x)),
                    Err(y) => Err(y),
                };
            }
            ANGLEUNIT => {
                return match AngleUnit::try_from(value) {
                    Ok(x) => Ok(Self::AngleUnit(x)),
                    Err(y) => Err(y),
                };
            }
            SCALEUNIT => {
                return match ScaleUnit::try_from(value) {
                    Ok(x) => Ok(Self::ScaleUnit(x)),
                    Err(y) => Err(y),
                };
            }
            PARAMETRICUNIT => {
                return match ParametricUnit::try_from(value) {
                    Ok(x) => Ok(Self::ParametricUnit(x)),
                    Err(y) => Err(y),
                };
            }
            UNIT => {
                error!(
                    "Ambiguous Keyword `UNIT`. While older versions of the specification allowed for this keyword, in this context it is ambiguous and should be depreciated in favour of `LENGTHUNIT`, `ANGLEUNIT` or `SCALEUNIT`"
                );
                return Err(WktParseError::IncorrectKeyword {
                    expected: vec![
                        LENGTHUNIT.to_string(),
                        ANGLEUNIT.to_string(),
                        SCALEUNIT.to_string(),
                    ]
                    .into(),
                    found: UNIT.to_string(),
                });
            }
            _ => {
                return Err(WktParseError::IncorrectKeyword {
                    expected: vec![
                        LENGTHUNIT.to_string(),
                        ANGLEUNIT.to_string(),
                        SCALEUNIT.to_string(),
                    ]
                    .into(),
                    found: UNIT.to_string(),
                });
            }
        }
    }
}
