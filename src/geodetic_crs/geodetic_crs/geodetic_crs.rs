use crate::{
    ast::WktNode,
    error::WktParseError,
    geodetic_crs::{
        dynamic_geodetic_crs::DynamicGeodeticCrs,
        geographic_crs::{DynamicGeographicCrs, GeographicCrs, StaticGeographicCrs},
        static_geodetic_crs::StaticGeodeticCrs,
    },
    keywords::{GEODCRS, GEODETICCRS, GEOGCRS, GEOGRAPHICCRS},
};

#[derive(Debug, PartialEq)]
pub enum GeodeticCrs {
    StaticGeodeticCrs(StaticGeodeticCrs),
    DynamicGeodeticCrs(DynamicGeodeticCrs),
    GeographicCrs(GeographicCrs),
}

impl TryFrom<&WktNode> for GeodeticCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        match value.keyword.as_str() {
            GEODCRS | GEODETICCRS => {
                // First try dynamic
                match DynamicGeodeticCrs::try_from(value) {
                    Ok(x) => return Ok(GeodeticCrs::DynamicGeodeticCrs(x)),
                    Err(_) => {}
                }

                // Then try static

                match StaticGeodeticCrs::try_from(value) {
                    Ok(x) => return Ok(GeodeticCrs::StaticGeodeticCrs(x)),
                    Err(y) => return Err(y),
                }
            }
            GEOGCRS | GEOGRAPHICCRS => {
                // First try dynamic
                match DynamicGeographicCrs::try_from(value) {
                    Ok(x) => {
                        return Ok(GeodeticCrs::GeographicCrs(
                            GeographicCrs::DynamicGeographicCrs(x),
                        ));
                    }
                    Err(_) => {}
                }

                // Now try static
                match StaticGeographicCrs::try_from(value) {
                    Ok(x) => {
                        return Ok(GeodeticCrs::GeographicCrs(
                            GeographicCrs::StaticGeographicCrs(x),
                        ));
                    }
                    Err(y) => return Err(y),
                }
            }
            _ => {
                return Err(WktParseError::IncorrectKeyword {
                    expected: vec![
                        GEODCRS.into(),
                        GEODETICCRS.into(),
                        GEOGCRS.into(),
                        GEOGRAPHICCRS.into(),
                    ]
                    .into(),
                    found: value.keyword.clone(),
                });
            }
        }
    }
}
