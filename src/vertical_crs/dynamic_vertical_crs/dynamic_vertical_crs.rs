use crate::{
    ast::{WktArg, WktNode},
    coordinate_system::CoordinateSystem,
    dynamic_crs::DynamicCrs,
    error::WktParseError,
    keywords::{VERTCRS, VERTICALCRS},
    scope_extent_identifier_remark::ScopeExtentIdentifierRemark,
    vertical_crs::{
        geoid_model_id::GeoidModelId, vertical_reference_frame::VerticalReferenceFrame,
    },
};

#[derive(Debug, PartialEq)]
pub struct DynamicVerticalCrs {
    pub crs_name: String,
    pub dynamic_crs: DynamicCrs,
    pub vertical_reference_frame: VerticalReferenceFrame,
    pub coordinate_system: CoordinateSystem,
    pub geoid_model_id: Option<GeoidModelId>,
    pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl TryFrom<&WktNode> for DynamicVerticalCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == VERTCRS || value.keyword == VERTICALCRS) {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![VERTCRS.into(), VERTICALCRS.into()].into(),
                found: value.keyword.clone(),
            });
        }

        // TODO: Arity?

        let crs_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let dynamic_crs = match &value.args[1] {
            WktArg::Node(node) => DynamicCrs::try_from(node)?,
            _ => return Err(WktParseError::CouldNotDetermineType),
        };

        let vertical_reference_frame = match &value.args[2] {
            WktArg::Node(node) => VerticalReferenceFrame::try_from(node)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let coordinate_system = CoordinateSystem::try_from(&value.args[3..value.args.len()])?;

        let mut i = 3 + coordinate_system.needed_args();

        let geoid_model_id = match value.args.get(i) {
            Some(arg) => {
                match arg {
                    WktArg::Node(node) => match GeoidModelId::try_from(node) {
                        Ok(x) => {
                            i += 1;
                            Some(x)
                        }
                        Err(_) => None, // Probably just something else
                    },
                    _ => None, //Probably just something else
                }
            }
            None => None,
        };

        let scope_extent_identifier_remark =
            ScopeExtentIdentifierRemark::try_from(&value.args[i..value.args.len()])?;

        return Ok(DynamicVerticalCrs {
            crs_name,
            dynamic_crs,
            vertical_reference_frame,
            coordinate_system,
            geoid_model_id,
            scope_extent_identifier_remark,
        });
    }
}
