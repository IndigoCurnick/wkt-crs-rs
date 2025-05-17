use log::error;

use crate::{
    ast::WktNode,
    error::WktParseError,
    keywords::{
        ANGLEUNIT, LENGTHUNIT, PARAMETRICUNIT, SCALEUNIT, TEMPORALQUANTITY, TIMEUNIT, UNIT,
    },
};

use super::{spatial_unit::SpatialUnit, time_unit::TimeUnit};

pub enum Unit {
    SpatialUnit(SpatialUnit),
    TimeUnit(TimeUnit),
}

impl TryFrom<&WktNode> for Unit {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        match value.keyword.as_str() {
            LENGTHUNIT | ANGLEUNIT | SCALEUNIT | PARAMETRICUNIT => {
                return match SpatialUnit::try_from(value) {
                    Ok(x) => Ok(Self::SpatialUnit(x)),
                    Err(y) => Err(y),
                };
            }
            TIMEUNIT | TEMPORALQUANTITY => {
                return match TimeUnit::try_from(value) {
                    Ok(x) => Ok(Self::TimeUnit(x)),
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
                        PARAMETRICUNIT.to_string(),
                        TEMPORALQUANTITY.to_string(),
                        TIMEUNIT.to_string(),
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
