use crate::{ast::WktArg, error::WktParseError};

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
