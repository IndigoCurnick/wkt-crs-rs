use crate::ast::WktNode;
pub use crate::base_types::derived_geodetic_crs::derived_geographic_crs::derived_dynamic_geog_crs::DerivedDynamicGeogCrs;
pub use crate::base_types::derived_geodetic_crs::derived_geographic_crs::derived_static_geog_crs::DerivedStaticGeogCrs;
use crate::error::WktParseError;
use crate::types::{WktBaseType, WktBaseTypeResult};

#[derive(Debug, PartialEq)]
pub enum DerivedGeographicCrs {
    DerivedStaticGeogCrs(DerivedStaticGeogCrs),
    DerivedDynamicGeogCrs(DerivedDynamicGeogCrs),
}

impl WktBaseType for DerivedGeographicCrs {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let iter: Vec<&'a WktNode> = wkt_nodes.into_iter().collect();

        if let Ok(stati) = DerivedStaticGeogCrs::from_nodes(iter.clone()) {
            return Ok(WktBaseTypeResult {
                result: Self::DerivedStaticGeogCrs(stati.result),
                consumed: stati.consumed,
            });
        }

        if let Ok(stati) = DerivedDynamicGeogCrs::from_nodes(iter.clone()) {
            return Ok(WktBaseTypeResult {
                result: Self::DerivedDynamicGeogCrs(stati.result),
                consumed: stati.consumed,
            });
        }

        return Err(WktParseError::CouldNotDetermineType);
    }
}
