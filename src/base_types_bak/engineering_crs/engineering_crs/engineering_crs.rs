use crate::{
    ast::{WktArg, WktNode},
    coordinate_system::CoordinateSystem,
    engineering_crs::engineering_datum::EngineeringDatum,
    error::WktParseError,
    keywords::{ENGCRS, ENGINEERINGCRS},
    scope_extent_identifier_remark::ScopeExtentIdentifierRemark,
};

#[derive(Debug, PartialEq)]
pub struct EngineeringCrs {
    pub crs_name: String,
    pub engineering_datum: EngineeringDatum,
    pub coordinate_system: CoordinateSystem,
    pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl TryFrom<&WktNode> for EngineeringCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == ENGCRS || value.keyword == ENGINEERINGCRS) {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![ENGCRS.into(), ENGINEERINGCRS.into()].into(),
                found: value.keyword.clone(),
            });
        }

        // TODO: Arity?

        let crs_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let engineering_datum = match &value.args[1] {
            WktArg::Node(node) => EngineeringDatum::try_from(node)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let coordinate_system = CoordinateSystem::try_from(&value.args[2..value.args.len()])?;

        let scope_extent_identifier_remark = ScopeExtentIdentifierRemark::try_from(
            &value.args[2 + coordinate_system.needed_args()..value.args.len()],
        )?;

        return Ok(EngineeringCrs {
            crs_name,
            engineering_datum,
            coordinate_system,
            scope_extent_identifier_remark,
        });
    }
}
