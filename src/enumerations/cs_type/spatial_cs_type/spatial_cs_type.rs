use std::str::FromStr;

use strum::{AsRefStr, EnumString};

use crate::{ast::WktArg, error::WktParseError};

#[derive(Debug, PartialEq, EnumString, AsRefStr)]
pub enum SpatialCsType {
    #[strum(serialize = "affine")]
    Affine,
    #[strum(serialize = "Cartesian")]
    Cartesian,
    #[strum(serialize = "cylindrical")]
    Cylindrical,
    #[strum(serialize = "ellipsoidal")]
    Ellipsoidal,
    #[strum(serialize = "linear")]
    Linear,
    #[strum(serialize = "parametric")]
    Parametric,
    #[strum(serialize = "polar")]
    Polar,
    #[strum(serialize = "spherical")]
    Spherical,
    #[strum(serialize = "vertical")]
    Vertical,
}

impl TryFrom<&WktArg> for SpatialCsType {
    type Error = WktParseError;

    fn try_from(value: &WktArg) -> Result<Self, Self::Error> {
        match value {
            WktArg::Data(s) => match SpatialCsType::from_str(s) {
                Ok(x) => return Ok(x),
                Err(y) => return Err(WktParseError::ParseError(y)),
            },
            _ => return Err(WktParseError::ExpectedString),
        }
    }
}
