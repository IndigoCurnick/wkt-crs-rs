use crate::{
    ast::{WktArg, WktNode},
    dynamic_crs::DynamicCrs,
    error::WktParseError,
    keywords::BASEVERTCRS,
    scope_extent_identifier_remark::Id,
    vertical_crs::VerticalReferenceFrame,
};

#[derive(Debug, PartialEq)]
pub struct BaseDynamicVerticalCrs {
    pub base_crs_name: String,
    pub dynamic_crs: DynamicCrs,
    pub vertical_reference_frame: VerticalReferenceFrame,
    pub identifier: Option<Id>, // TODO: multiple
}

impl TryFrom<&WktNode> for BaseDynamicVerticalCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != BASEVERTCRS {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![BASEVERTCRS.into()].into(),
                found: value.keyword.clone(),
            });
        }

        // TODO: What is the arity?

        let base_crs_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let dynamic_crs = match &value.args[1] {
            WktArg::Node(node) => DynamicCrs::try_from(node)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let vertical_reference_frame = match &value.args[2] {
            WktArg::Node(node) => VerticalReferenceFrame::try_from(node)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let identifier = match value.args.get(3) {
            Some(x) => match x {
                WktArg::Node(n) => Some(Id::try_from(n)?),
                _ => return Err(WktParseError::ExpectedNode),
            },
            None => None,
        };

        return Ok(Self {
            base_crs_name,
            dynamic_crs,
            vertical_reference_frame,
            identifier,
        });
    }
}
