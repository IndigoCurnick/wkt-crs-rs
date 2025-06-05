use crate::{
    ast::{WktArg, WktNode},
    coordinate_system::CoordinateSystem,
    error::WktParseError,
    keywords::TIMECRS,
    scope_extent_identifier_remark::ScopeExtentIdentifierRemark,
    temporal_crs::temporal_datum::TemporalDatum,
};

#[derive(Debug, PartialEq)]
pub struct TemporalCrs {
    pub crs_name: String,
    pub temporal_datum: TemporalDatum,
    pub coordinate_system: CoordinateSystem,
    pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl TryFrom<&WktNode> for TemporalCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != TIMECRS {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![TIMECRS.into()].into(),
                found: value.keyword.clone(),
            });
        }

        // TODO: Arity

        let crs_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let temporal_datum = match &value.args[1] {
            WktArg::Node(node) => TemporalDatum::try_from(node)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let coordinate_system = CoordinateSystem::try_from(&value.args[2..value.args.len()])?;

        let args_needed = coordinate_system.needed_args();

        // TODO: Can this fail?
        let scope_extent_identifier_remark = ScopeExtentIdentifierRemark::try_from(
            &value.args[(2 + args_needed)..value.args.len()],
        )?;

        return Ok(TemporalCrs {
            crs_name,
            temporal_datum,
            coordinate_system,
            scope_extent_identifier_remark,
        });
    }
}
