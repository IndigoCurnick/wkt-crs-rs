use crate::{
    ast::WktNode,
    derived_vertical_crs::base_vertical_crs::{
        base_dynamic_vertical_crs::BaseDynamicVerticalCrs,
        base_static_vertical_crs::BaseStaticVerticalCrs,
    },
    error::WktParseError,
};

mod base_dynamic_vertical_crs;
mod base_static_vertical_crs;

pub enum BaseVerticalCrs {
    BaseDynamicVerticalCrs(BaseDynamicVerticalCrs),
    BaseStaticVerticalCrs(BaseStaticVerticalCrs),
}

impl TryFrom<&WktNode> for BaseVerticalCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if let Ok(base_dynamic_vertical_crs) = BaseDynamicVerticalCrs::try_from(value) {
            return Ok(Self::BaseDynamicVerticalCrs(base_dynamic_vertical_crs));
        };

        if let Ok(base_static_vertical_crs) = BaseStaticVerticalCrs::try_from(value) {
            return Ok(Self::BaseStaticVerticalCrs(base_static_vertical_crs));
        };

        return Err(WktParseError::CouldNotDetermineType);
    }
}
