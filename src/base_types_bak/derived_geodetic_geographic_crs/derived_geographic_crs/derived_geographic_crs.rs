use crate::{
    ast::WktNode,
    derived_geodetic_geographic_crs::{
        derived_dynamic_geog_crs::DerivedDynamicGeogCrs,
        derived_static_geog_crs::DerivedStaticGeogCrs,
    },
    error::WktParseError,
    keywords::{GEOGCRS, GEOGRAPHICCRS},
};

#[derive(Debug, PartialEq)]
pub enum DerivedGeographicCrs {
    DerivedStaticGeogCrs(DerivedStaticGeogCrs),
    DerivedDynamicGeogCrs(DerivedDynamicGeogCrs),
}

impl TryFrom<&WktNode> for DerivedGeographicCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == GEOGCRS || value.keyword == GEOGRAPHICCRS) {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![GEOGCRS.into(), GEOGRAPHICCRS.into()].into(),
                found: value.keyword.clone(),
            });
        }

        if let Ok(derived_static_geog_crs) = DerivedStaticGeogCrs::try_from(value) {
            return Ok(Self::DerivedStaticGeogCrs(derived_static_geog_crs));
        }

        if let Ok(derived_dynamic_geog_crs) = DerivedDynamicGeogCrs::try_from(value) {
            return Ok(Self::DerivedDynamicGeogCrs(derived_dynamic_geog_crs));
        }

        return Err(WktParseError::CouldNotDetermineType);
    }
}
