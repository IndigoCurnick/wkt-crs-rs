use crate::{
    ast::{WktArg, WktNode},
    dynamic_crs::DynamicCrs,
    ensemble::VerticalEnsemble,
    error::WktParseError,
    keywords::BASEVERTCRS,
    scope_extent_identifier_remark::Id,
    vertical_crs::{VerticalCrs, VerticalFrameDatum, VerticalReferenceFrame},
};

#[derive(Debug, PartialEq)]
pub struct BaseStaticVerticalCrs {
    pub base_crs_name: String,
    pub vertical_frame_datum: VerticalFrameDatum,
    pub vertical_reference_frame: VerticalReferenceFrame,
    pub identifier: Option<Id>, // TODO: multiple
}

impl TryFrom<&WktNode> for BaseStaticVerticalCrs {
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

        let vertical_frame_datum = match &value.args[1] {
            WktArg::Node(node) => VerticalFrameDatum::try_from(node)?,
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
            vertical_frame_datum,
            vertical_reference_frame,
            identifier,
        });
    }
}
