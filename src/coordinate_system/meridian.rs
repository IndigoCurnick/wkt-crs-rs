use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::MERIDIAN,
    units::AngleUnit,
};

#[derive(Debug, PartialEq)]
pub struct Meridian {
    pub number: f64,
    pub angle_unit: AngleUnit,
}

impl TryFrom<&WktNode> for Meridian {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != MERIDIAN {
            let expected = vec![MERIDIAN.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if value.args.len() != 2 {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["2".to_string()].into(),
                found: value.args.len(),
            });
        }

        let number = match &value.args[0] {
            WktArg::Number(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedNumber),
        };

        let angle_unit = match &value.args[1] {
            WktArg::Node(n) => AngleUnit::try_from(n)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        Ok(Meridian { number, angle_unit })
    }
}
