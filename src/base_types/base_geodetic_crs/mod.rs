mod base_dynamic_geodetic_crs;
mod base_dynamic_geographic_crs;
mod base_static_geodetic_crs;
mod base_static_geographic_crs;

use crate::{
    ast::WktNode,
    error::WktParseError,
    types::{WktBaseType, WktBaseTypeResult},
};

pub use base_dynamic_geodetic_crs::BaseDynamicGeodeticCrs;
pub use base_dynamic_geographic_crs::BaseDynamicGeographicCrs;
pub use base_static_geodetic_crs::BaseStaticGeodeticCrs;
pub use base_static_geographic_crs::BaseStaticGeographicCrs;

#[derive(Debug, PartialEq)]
pub enum BaseStaticCrs {
    BaseStaticGeodeticCrs(BaseStaticGeodeticCrs),
    BaseStaticGeographicCrs(BaseStaticGeographicCrs),
}

impl WktBaseType for BaseStaticCrs {
    fn from_nodes<'a, I>(
        wkt_nodes: I,
    ) -> Result<crate::types::WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let iter: Vec<&'a WktNode> = wkt_nodes.into_iter().collect();

        if let Ok(base_static) = BaseStaticGeodeticCrs::from_nodes(iter.clone()) {
            return Ok(WktBaseTypeResult {
                result: Self::BaseStaticGeodeticCrs(base_static.result),
                consumed: base_static.consumed,
            });
        }

        if let Ok(base_static) = BaseStaticGeographicCrs::from_nodes(iter.clone()) {
            return Ok(WktBaseTypeResult {
                result: Self::BaseStaticGeographicCrs(base_static.result),
                consumed: base_static.consumed,
            });
        }

        return Err(WktParseError::CouldNotDetermineType);
    }
}

// * yes yes it is called `geodetic` but contains geographic....
#[derive(Debug, PartialEq)]
pub enum BaseGeodeticCrs {
    BaseStaticGeodeticCrs(BaseStaticGeodeticCrs),
    BaseDynamicGeodeticCrs(BaseDynamicGeodeticCrs),
    BaseStaticGeographicCrs(BaseStaticGeographicCrs),
    BaseDynamicGeographicCrs(BaseDynamicGeographicCrs),
}

impl WktBaseType for BaseGeodeticCrs {
    fn from_nodes<'a, I>(
        wkt_nodes: I,
    ) -> Result<crate::types::WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let iter: Vec<&'a WktNode> = wkt_nodes.into_iter().collect();

        if let Ok(base_static) = BaseStaticGeodeticCrs::from_nodes(iter.clone()) {
            return Ok(WktBaseTypeResult {
                result: Self::BaseStaticGeodeticCrs(base_static.result),
                consumed: base_static.consumed,
            });
        }

        if let Ok(base_dynamic) = BaseDynamicGeodeticCrs::from_nodes(iter.clone()) {
            return Ok(WktBaseTypeResult {
                result: Self::BaseDynamicGeodeticCrs(base_dynamic.result),
                consumed: base_dynamic.consumed,
            });
        }

        if let Ok(base_static) = BaseStaticGeographicCrs::from_nodes(iter.clone()) {
            return Ok(WktBaseTypeResult {
                result: Self::BaseStaticGeographicCrs(base_static.result),
                consumed: base_static.consumed,
            });
        }

        if let Ok(base_dynamic) = BaseDynamicGeographicCrs::from_nodes(iter) {
            return Ok(WktBaseTypeResult {
                result: Self::BaseDynamicGeographicCrs(base_dynamic.result),
                consumed: base_dynamic.consumed,
            });
        }

        return Err(WktParseError::CouldNotDetermineType);
    }
}

#[derive(Debug, PartialEq)]
pub enum BaseDynamicCrs {
    BaseDynamicGeodeticCrs(BaseDynamicGeodeticCrs),
    BaseDynamicGeographicCrs(BaseDynamicGeographicCrs),
}

impl WktBaseType for BaseDynamicCrs {
    fn from_nodes<'a, I>(
        wkt_nodes: I,
    ) -> Result<crate::types::WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let iter: Vec<&'a WktNode> = wkt_nodes.into_iter().collect();

        if let Ok(base_dynamic) = BaseDynamicGeodeticCrs::from_nodes(iter.clone()) {
            return Ok(WktBaseTypeResult {
                result: Self::BaseDynamicGeodeticCrs(base_dynamic.result),
                consumed: base_dynamic.consumed,
            });
        }

        if let Ok(base_dynamic) = BaseDynamicGeographicCrs::from_nodes(iter) {
            return Ok(WktBaseTypeResult {
                result: Self::BaseDynamicGeographicCrs(base_dynamic.result),
                consumed: base_dynamic.consumed,
            });
        }

        return Err(WktParseError::CouldNotDetermineType);
    }
}
