use crate::{
    ast::{WktArg, WktNode},
    coordinate_system::CoordinateSystem,
    error::WktParseError,
    keywords::{PROJCRS, PROJECTEDCRS},
    map_projection::MapProjection,
    proj_crs::base_geodetic_crs::BaseGeodeticGeographicCrs,
    scope_extent_identifier_remark::ScopeExtentIdentifierRemark,
};

#[derive(Debug, PartialEq)]
pub struct ProjectedCrs {
    pub crs_name: String,
    pub base_geodetic_crs: BaseGeodeticGeographicCrs,
    pub map_projection: MapProjection,
    pub coordinate_system: CoordinateSystem,
    pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl TryFrom<&WktNode> for ProjectedCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == PROJCRS || value.keyword == PROJECTEDCRS) {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![PROJCRS.into(), PROJECTEDCRS.into()].into(),
                found: value.keyword.clone(),
            });
        }

        // TODO: arity?

        let crs_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let base_geodetic_crs = match &value.args[1] {
            WktArg::Node(node) => BaseGeodeticGeographicCrs::try_from(node)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let map_projection = match &value.args[2] {
            WktArg::Node(node) => MapProjection::try_from(node)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let coordinate_system = CoordinateSystem::try_from(&value.args[3..value.args.len()])?;

        let args_needed = coordinate_system.needed_args();

        // TODO: Can this fail?
        let scope_extent_identifier_remark = ScopeExtentIdentifierRemark::try_from(
            &value.args[(3 + args_needed)..value.args.len()],
        )?;

        return Ok(ProjectedCrs {
            crs_name,
            base_geodetic_crs,
            map_projection,
            coordinate_system,
            scope_extent_identifier_remark,
        });
    }
}
