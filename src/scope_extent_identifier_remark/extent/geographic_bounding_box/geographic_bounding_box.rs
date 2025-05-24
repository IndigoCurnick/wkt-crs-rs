use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::BBOX,
};

#[derive(Debug, PartialEq)]
pub struct GeographicBoundingBox {
    pub lower_left_latitude: f64,
    pub lower_left_longitude: f64,
    pub upper_right_latitude: f64,
    pub upper_right_longitude: f64,
}

// TODO: There should be some restrictions on the allowed lats/lons
impl TryFrom<&WktNode> for GeographicBoundingBox {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != BBOX {
            let expected = vec![BBOX.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if value.args.len() != 4 {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["4".to_string()].into(),
                found: value.args.len(),
            });
        }

        let lower_left_latitude = match &value.args[0] {
            WktArg::Number(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedNumber),
        };

        let lower_left_longitude = match &value.args[1] {
            WktArg::Number(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedNumber),
        };

        let upper_right_latitude = match &value.args[2] {
            WktArg::Number(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedNumber),
        };

        let upper_right_longitude = match &value.args[3] {
            WktArg::Number(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedNumber),
        };

        return Ok(GeographicBoundingBox {
            lower_left_latitude,
            lower_left_longitude,
            upper_right_latitude,
            upper_right_longitude,
        });
    }
}
