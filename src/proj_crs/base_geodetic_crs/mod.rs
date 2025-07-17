pub use base_dynamic_geodetic_crs::BaseDynamicGeodeticCrs;
pub use base_dynamic_geographic_crs::BaseDynamicGeographicCrs;
pub use base_static_geodetic_crs::BaseStaticGeodeticCrs;
pub use base_static_geographic_crs::BaseStaticGeographicCrs;

use crate::{
    ast::WktNode,
    error::WktParseError,
    keywords::{BASEGEODCRS, BASEGEOGCRS},
};

mod base_dynamic_geodetic_crs;
mod base_dynamic_geographic_crs;
mod base_static_geodetic_crs;
mod base_static_geographic_crs;

#[derive(Debug, PartialEq)]
pub enum BaseStaticCrs {
    BaseStaticGeodeticCrs(BaseStaticGeodeticCrs),
    BaseStaticGeographicCrs(BaseStaticGeographicCrs),
}

impl TryFrom<&WktNode> for BaseStaticCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        match value.keyword.as_str() {
            BASEGEODCRS => {
                return match BaseStaticGeodeticCrs::try_from(value) {
                    Ok(x) => Ok(BaseStaticCrs::BaseStaticGeodeticCrs(x)),
                    Err(y) => Err(y),
                };
            }
            BASEGEOGCRS => {
                return match BaseStaticGeographicCrs::try_from(value) {
                    Ok(x) => Ok(BaseStaticCrs::BaseStaticGeographicCrs(x)),
                    Err(y) => Err(y),
                };
            }
            _ => {
                return Err(WktParseError::IncorrectKeyword {
                    expected: vec![BASEGEODCRS.into(), BASEGEOGCRS.into()].into(),
                    found: value.keyword.clone(),
                });
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BaseGeodeticCrs {
    BaseStaticGeodeticCrs(BaseStaticGeodeticCrs),
    BaseDynamicGeodeticCrs(BaseDynamicGeodeticCrs),
    BaseStaticGeographicCrs(BaseStaticGeographicCrs),
    BaseDynamicGeographicCrs(BaseDynamicGeographicCrs),
}

impl TryFrom<&WktNode> for BaseGeodeticCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        match value.keyword.as_str() {
            BASEGEODCRS => {
                match BaseDynamicGeodeticCrs::try_from(value) {
                    Ok(x) => return Ok(BaseGeodeticCrs::BaseDynamicGeodeticCrs(x)),
                    _ => {}
                };

                return match BaseStaticGeodeticCrs::try_from(value) {
                    Ok(x) => Ok(BaseGeodeticCrs::BaseStaticGeodeticCrs(x)),
                    Err(y) => Err(y),
                };
            }
            BASEGEOGCRS => {
                match BaseDynamicGeographicCrs::try_from(value) {
                    Ok(x) => return Ok(BaseGeodeticCrs::BaseDynamicGeographicCrs(x)),
                    _ => {}
                };

                return match BaseStaticGeographicCrs::try_from(value) {
                    Ok(x) => Ok(BaseGeodeticCrs::BaseStaticGeographicCrs(x)),
                    Err(y) => Err(y),
                };
            }
            _ => {
                return Err(WktParseError::IncorrectKeyword {
                    expected: vec![BASEGEODCRS.into(), BASEGEOGCRS.into()].into(),
                    found: value.keyword.clone(),
                });
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BaseDynamicCrs {
    BaseDynamicGeodeticCrs(BaseDynamicGeodeticCrs),
    BaseDynamicGeographicCrs(BaseDynamicGeographicCrs),
}

impl TryFrom<&WktNode> for BaseDynamicCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        match value.keyword.as_str() {
            BASEGEODCRS => {
                return match BaseDynamicGeodeticCrs::try_from(value) {
                    Ok(x) => Ok(BaseDynamicCrs::BaseDynamicGeodeticCrs(x)),
                    Err(y) => Err(y),
                };
            }
            BASEGEOGCRS => {
                return match BaseDynamicGeographicCrs::try_from(value) {
                    Ok(x) => Ok(BaseDynamicCrs::BaseDynamicGeographicCrs(x)),
                    Err(y) => Err(y),
                };
            }
            _ => {
                return Err(WktParseError::IncorrectKeyword {
                    expected: vec![BASEGEODCRS.into(), BASEGEOGCRS.into()].into(),
                    found: value.keyword.clone(),
                });
            }
        }
    }
}
