use crate::{ast::WktArg, error::WktParseError};

#[derive(Debug, PartialEq)]
pub enum Dimension {
    One,
    Two,
    Three,
}

impl From<&Dimension> for usize {
    fn from(value: &Dimension) -> Self {
        match value {
            Dimension::One => 1,
            Dimension::Two => 2,
            Dimension::Three => 3,
        }
    }
}

impl TryFrom<i32> for Dimension {
    type Error = WktParseError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        return match value {
            1 => Ok(Dimension::One),
            2 => Ok(Dimension::Two),
            3 => Ok(Dimension::Three),
            _ => Err(WktParseError::IncorrectValue),
        };
    }
}

impl TryFrom<&str> for Dimension {
    type Error = WktParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let num = match value.parse::<i32>() {
            Ok(x) => x,
            Err(_) => return Err(WktParseError::ExpectedNumber),
        };

        return Self::try_from(num);
    }
}

impl TryFrom<&WktArg> for Dimension {
    type Error = WktParseError;

    fn try_from(value: &WktArg) -> Result<Self, Self::Error> {
        return match value {
            WktArg::Data(n) => Ok(Dimension::try_from(n.as_str())?),
            _ => return Err(WktParseError::ExpectedNumber),
        };
    }
}
