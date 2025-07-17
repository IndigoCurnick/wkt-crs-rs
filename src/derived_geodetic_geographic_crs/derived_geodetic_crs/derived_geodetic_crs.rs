use crate::{
    ast::WktNode,
    derived_geodetic_geographic_crs::{
        derived_dynamic_geod_crs::DerivedDynamicGeodCrs,
        derived_geographic_crs::DerivedGeographicCrs,
        derived_static_geod_crs::DerivedStaticGeodCrs,
    },
    error::WktParseError,
    keywords::{GEODCRS, GEODETICCRS, GEOGCRS, GEOGRAPHICCRS},
};

#[derive(Debug, PartialEq)]
pub enum DerivedGeodeticCrs {
    DerivedStaticGeodCrs(DerivedStaticGeodCrs),
    DerivedDynamicGeodCrs(DerivedDynamicGeodCrs),
    DerivedGeographicCrs(DerivedGeographicCrs),
}

impl TryFrom<&WktNode> for DerivedGeodeticCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == GEOGCRS || value.keyword == GEOGRAPHICCRS) {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![GEOGCRS.into(), GEOGRAPHICCRS.into()].into(),
                found: value.keyword.clone(),
            });
        }

        match value.keyword.as_str() {
            GEOGCRS | GEOGRAPHICCRS => {
                return match DerivedGeographicCrs::try_from(value) {
                    Ok(x) => Ok(Self::DerivedGeographicCrs(x)),
                    Err(y) => Err(y),
                };
            }
            GEODCRS | GEODETICCRS => {
                if let Ok(derived_static_geod_crs) = DerivedStaticGeodCrs::try_from(value) {
                    return Ok(Self::DerivedStaticGeodCrs(derived_static_geod_crs));
                }

                return match DerivedDynamicGeodCrs::try_from(value) {
                    Ok(x) => Ok(Self::DerivedDynamicGeodCrs(x)),
                    Err(y) => Err(y),
                };
            }
            _ => {
                return Err(WktParseError::IncorrectKeyword {
                    expected: vec![
                        GEOGCRS.into(),
                        GEOGRAPHICCRS.into(),
                        GEODCRS.into(),
                        GEODETICCRS.into(),
                    ]
                    .into(),
                    found: value.keyword.clone(),
                });
            }
        }
    }
}
