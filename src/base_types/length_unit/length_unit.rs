// 7.4.1

use log::warn;

use crate::{
    arity::match_arity,
    ast::{Parse, WktNode},
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult},
};

// TODO: Can take an ID too optionally
#[derive(Debug, PartialEq)]
pub struct LengthUnit {
    pub unit_name: String,
    pub conversion_factor: f64,
}

impl WktBaseType for LengthUnit {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::LengthUnit, Keywords::Unit])?;
        match_arity(node.args.len(), 2, 3)?;

        if node.keyword == Keywords::Unit {
            warn!("Keyword UNIT depreciated. Consider using LENGTHUNIT instead");
        }

        let unit_name = node.args[0].parse()?;
        let conversion_factor = node.args[1].parse()?;

        let lu = LengthUnit {
            unit_name,
            conversion_factor,
        };

        Ok(WktBaseTypeResult {
            result: lu,
            consumed: 1,
        })
    }
}
