use crate::{
    arity::lower_bound_arity,
    ast::{Parse, WktNode},
    base_types::{BaseGeodeticCrs, CoordinateSystem, MapProjection},
    compound_types::ScopeExtentIdentifierRemark,
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct ProjectedCrs {
    pub crs_name: String,
    pub base_geodetic_crs: BaseGeodeticCrs,
    pub map_projection: MapProjection,
    pub coordinate_system: CoordinateSystem,
    pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl WktBaseType for ProjectedCrs {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(
            &node.keyword,
            vec![Keywords::ProjCrs, Keywords::ProjectedCrs],
        )?;
        lower_bound_arity(node.args.len(), 4);

        let crs_name = node.args[0].parse()?;
        let base_geodetic_crs = node.args[1].parse()?;
        let map_projection = node.args[2].parse()?;

        let coordinate_system = CoordinateSystem::from_args(&node.args[3..node.args.len()])?;

        let scope_extent_identifier_remark = ScopeExtentIdentifierRemark::from_args(
            &node.args[3 + coordinate_system.consumed..node.args.len()],
        )?;

        let res = ProjectedCrs {
            crs_name,
            base_geodetic_crs,
            map_projection,
            coordinate_system: coordinate_system.result,
            scope_extent_identifier_remark: scope_extent_identifier_remark.result,
        };

        Ok(WktBaseTypeResult {
            result: res,
            consumed: 1,
        })
    }
}
