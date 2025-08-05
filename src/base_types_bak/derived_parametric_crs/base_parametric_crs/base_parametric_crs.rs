use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::BASEPARAMCRS,
    parametric_crs::ParametricDatum,
    scope_extent_identifier_remark::Id,
};

pub struct BaseParametricCrs {
    pub base_crs_name: String,
    pub parametric_datum: ParametricDatum,
    pub identifier: Option<Id>,
}

impl TryFrom<&WktNode> for BaseParametricCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != BASEPARAMCRS {
            let expected = vec![BASEPARAMCRS.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if !(value.args.len() == 2 || value.args.len() == 3) {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["2".to_string(), "3".to_string()].into(),
                found: value.args.len(),
            });
        }

        let base_crs_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let parametric_datum = match &value.args[1] {
            WktArg::Node(n) => ParametricDatum::try_from(n)?,
            _ => return Err(WktParseError::ExpectedString),
        };

        let identifier = match value.args.get(2) {
            Some(x) => match x {
                WktArg::Node(y) => Some(Id::try_from(y)?),
                _ => return Err(WktParseError::ExpectedNode),
            },
            None => None,
        };

        return Ok(Self {
            base_crs_name,
            parametric_datum,
            identifier,
        });
    }
}
