use crate::{
    arity::match_arity,
    ast::{Parse, WktNode},
    data_types::DateOrString,
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct TemporalExtent {
    pub from: DateOrString,
    pub to: DateOrString,
}

impl WktBaseType for TemporalExtent {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<crate::types::WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        // Take 1

        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::TimeExtent])?;
        match_arity(node.args.len(), 2, 2)?;

        let from = node.args[0].parse()?;
        let to = node.args[1].parse()?;

        let res = TemporalExtent { from, to };

        return Ok(WktBaseTypeResult {
            result: res,
            consumed: 1,
        });
    }
}
