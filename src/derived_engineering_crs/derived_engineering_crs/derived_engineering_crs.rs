use crate::{
    ast::{WktArg, WktNode},
    coordinate_system::CoordinateSystem,
    derived_crs::DerivingConversion,
    derived_engineering_crs::base_engineering_crs::BaseEngineeringCrs,
    error::WktParseError,
    keywords::{ENGCRS, ENGINEERINGCRS},
    scope_extent_identifier_remark::ScopeExtentIdentifierRemark,
};

pub struct DerivedEngineeringCrs {
    pub derived_crs_name: String,
    pub base_engineering_crs: BaseEngineeringCrs,
    pub deriving_conversion: DerivingConversion,
    pub coordinate_system: CoordinateSystem,
    pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl TryFrom<&WktNode> for DerivedEngineeringCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == ENGCRS || value.keyword == ENGINEERINGCRS) {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![ENGCRS.into(), ENGINEERINGCRS.into()].into(),
                found: value.keyword.clone(),
            });
        }

        // TODO: What is the arity?

        let derived_crs_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let base_engineering_crs = match &value.args[1] {
            WktArg::Node(node) => BaseEngineeringCrs::try_from(node)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let deriving_conversion = match &value.args[2] {
            WktArg::Node(node) => DerivingConversion::try_from(node)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let coordinate_system = CoordinateSystem::try_from(&value.args[3..value.args.len()])?;

        let scope_extent_identifier_remark = ScopeExtentIdentifierRemark::try_from(
            &value.args[3 + coordinate_system.needed_args()..value.args.len()],
        )?;

        return Ok(Self {
            derived_crs_name,
            base_engineering_crs,
            deriving_conversion,
            coordinate_system,
            scope_extent_identifier_remark,
        });
    }
}
