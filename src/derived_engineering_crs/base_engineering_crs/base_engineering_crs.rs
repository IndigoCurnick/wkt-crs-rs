use crate::{
    ast::{WktArg, WktNode},
    engineering_crs::EngineeringDatum,
    error::WktParseError,
    keywords::BASEENGCRS,
    scope_extent_identifier_remark::Id,
};

pub struct BaseEngineeringCrs {
    pub base_crs_name: String,
    pub engineering_datum: EngineeringDatum,
    pub identifier: Option<Id>,
}

impl TryFrom<&WktNode> for BaseEngineeringCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != BASEENGCRS {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![BASEENGCRS.into()].into(),
                found: value.keyword.clone(),
            });
        }

        // TODO: Arity?

        let base_crs_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let engineering_datum = match &value.args[1] {
            WktArg::Node(node) => EngineeringDatum::try_from(node)?,
            _ => return Err(WktParseError::ExpectedNode),
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
            engineering_datum,
            identifier,
        });
    }
}
