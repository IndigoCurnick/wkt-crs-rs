use crate::{
    arity::match_arity,
    ast::{Parse, WktNode},
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktResult},
};

#[derive(Debug, PartialEq)]
pub struct Citation(pub String);

impl WktBaseType for Citation {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::Citation])?;
        match_arity(node.args.len(), 1, 1)?;

        let citation = node.args[0].parse()?;

        let cite = Citation(citation);

        return Ok(WktResult {
            result: cite,
            consumed: 1,
        });
    }
}
