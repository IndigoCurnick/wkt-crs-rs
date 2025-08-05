use crate::{
    ast::{WktArg, WktNode},
    derived_crs::DerivingConversion,
    derived_vertical_crs::base_vertical_crs::BaseVerticalCrs,
    error::WktParseError,
    keywords::{VERTCRS, VERTICALCRS},
    scope_extent_identifier_remark::ScopeExtentIdentifierRemark,
};

pub struct DerivedVerticalCrs {
    pub derived_crs_name: String,
    pub base_vertical_crs: BaseVerticalCrs,
    pub deriving_conversion: DerivingConversion,
    pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl TryFrom<&WktNode> for DerivedVerticalCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == VERTCRS || value.keyword == VERTICALCRS) {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![VERTCRS.into(), VERTICALCRS.into()].into(),
                found: value.keyword.clone(),
            });
        }

        // TODO: What is the arity?

        let derived_crs_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let base_vertical_crs = match &value.args[1] {
            WktArg::Node(node) => BaseVerticalCrs::try_from(node)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let deriving_conversion = match &value.args[2] {
            WktArg::Node(node) => DerivingConversion::try_from(node)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let scope_extent_identifier_remark =
            ScopeExtentIdentifierRemark::try_from(&value.args[3..value.args.len()])?;

        return Ok(Self {
            derived_crs_name,
            base_vertical_crs,
            deriving_conversion,
            scope_extent_identifier_remark,
        });
    }
}
