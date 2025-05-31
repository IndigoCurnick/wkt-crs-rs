use dynamic_vertical_crs::DynamicVerticalCrs;
use static_vertical_crs::StaticVerticalCrs;

use crate::{ast::WktNode, error::WktParseError};

mod dynamic_vertical_crs;
mod geoid_model_id;
mod static_vertical_crs;
mod vertical_crs;
mod vertical_reference_frame;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub enum VerticalCrs {
    StaticVerticalCrs(StaticVerticalCrs),
    DynamicVerticalCrs(DynamicVerticalCrs),
}

impl TryFrom<&WktNode> for VerticalCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        match DynamicVerticalCrs::try_from(value) {
            Ok(x) => return Ok(VerticalCrs::DynamicVerticalCrs(x)),
            Err(_) => {}
        };

        match StaticVerticalCrs::try_from(value) {
            Ok(x) => return Ok(VerticalCrs::StaticVerticalCrs(x)),
            Err(y) => return Err(y),
        }
    }
}
