// Section 8.2.1

use log::warn;

use crate::{
    arity::match_arity,
    ast::{Parse, WktNode},
    base_types::LengthUnit,
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktResult},
};

#[derive(Debug, PartialEq)]
pub struct Ellipsoid {
    pub ellipsoid_name: String,
    pub semi_major_axis: f64,
    pub inverse_flattening: f64,
    pub length_unit: Option<LengthUnit>,
}

impl WktBaseType for Ellipsoid {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<crate::types::WktResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        // Take 1
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::Spheroid, Keywords::Ellipsoid])?;

        if node.keyword == Keywords::Spheroid {
            warn!("Keyword `SPHEROID` depreciated - use `ELLIPSOID` instead");
        }

        match_arity(node.args.len(), 3, 4)?;

        let ellipsoid_name = node.args[0].parse()?;

        let semi_major_axis = node.args[1].parse()?;

        let inverse_flattening = node.args[2].parse()?;

        let length_unit = match node.args.get(3) {
            Some(x) => Some(x.parse()?),
            None => None,
        };

        let ellipsoid = Ellipsoid {
            ellipsoid_name,
            inverse_flattening,
            length_unit,
            semi_major_axis,
        };

        let res = WktResult {
            consumed: 1,
            result: ellipsoid,
        };

        Ok(res)
    }
}
