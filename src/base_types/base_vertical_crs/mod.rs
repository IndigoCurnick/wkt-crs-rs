pub use crate::base_types::base_vertical_crs::base_dynamic_vertical_crs::BaseDynamicVerticalCrs;
pub use crate::base_types::base_vertical_crs::base_static_vertical_crs::BaseStaticVerticalCrs;
use crate::{
    ast::WktNode,
    error::WktParseError,
    types::{WktBaseType, WktBaseTypeResult},
};

mod base_dynamic_vertical_crs;
mod base_static_vertical_crs;

#[derive(Debug, PartialEq)]
pub enum BaseVerticalCrs {
    BaseDynamicVerticalCrs(BaseDynamicVerticalCrs),
    BaseStaticVerticalCrs(BaseStaticVerticalCrs),
}

impl WktBaseType for BaseVerticalCrs {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let iter: Vec<&'a WktNode> = wkt_nodes.into_iter().collect();

        if let Ok(res) = BaseDynamicVerticalCrs::from_nodes(iter.clone()) {
            return Ok(WktBaseTypeResult {
                consumed: res.consumed,
                result: Self::BaseDynamicVerticalCrs(res.result),
            });
        }

        if let Ok(res) = BaseStaticVerticalCrs::from_nodes(iter.clone()) {
            return Ok(WktBaseTypeResult {
                consumed: res.consumed,
                result: Self::BaseStaticVerticalCrs(res.result),
            });
        }

        return Err(WktParseError::CouldNotDetermineType);
    }
}
