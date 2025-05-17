use crate::map_projection::MapProjection;

pub struct ProjectedCrs {
    crs_name: String,
    base_geodetic_crs: BaseGeodeticCrs,
    map_projection: MapProjection,
    coordinate_system: CoordinateSystem,
    scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

pub enum BaseGeodeticCrs {
    BaseStaticGeodeticCrs(BaseStaticGeodeticCrs),
    BaseDynamicGeodeticCrs(BaseDynamicGeodeticCrs),
    BaseStaticGeographicCrs(BaseStaticGeographicCrs),
    BaseDynamicGeographicCrs(BaseDynamicGeographicCrs),
}

pub struct BaseStaticGeodeticCrs {
    base_crs_name: String,
}
