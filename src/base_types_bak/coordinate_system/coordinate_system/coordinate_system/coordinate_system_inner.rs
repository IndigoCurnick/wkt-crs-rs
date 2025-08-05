use crate::{
    ast::{WktArg, WktNode},
    coordinate_system::{cs_type::CsType, dimension::Dimension},
    error::WktParseError,
    keywords::CS,
    scope_extent_identifier_remark::Id,
};

pub struct CsInner {
    pub cs_type: CsType,
    pub dimension: Dimension,
    pub identifier: Option<Id>, // TODO: Technically this can be many
}

impl TryFrom<&WktArg> for CsInner {
    type Error = WktParseError;

    fn try_from(value: &WktArg) -> Result<Self, Self::Error> {
        let value = match value {
            WktArg::Node(node) => node,
            _ => return Err(WktParseError::ExpectedNode),
        };

        if value.keyword != CS {
            let expected = vec![CS.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if !(value.args.len() == 3 || value.args.len() == 2) {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["2".to_string(), "3".to_string()].into(),
                found: value.args.len(),
            });
        }

        let cs_type = CsType::try_from(&value.args[0])?;
        let dimension = Dimension::try_from(&value.args[1])?;

        let identifier = match value.args.get(2) {
            Some(x) => match x {
                WktArg::Node(y) => Some(Id::try_from(y)?),
                _ => return Err(WktParseError::ExpectedNode),
            },
            None => None,
        };

        Ok(CsInner {
            cs_type,
            dimension,
            identifier,
        })
    }
}
