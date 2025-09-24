use crate::{
    arity::match_arity,
    ast::{Parse, WktNode},
    base_types::{Extent, Scope},
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct Usage {
    pub scope: Scope,
    pub extent: Extent,
}

impl WktBaseType for Usage {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::Usage])?;
        match_arity(node.args.len(), 2, 5)?; // ! Keep in mind, Extent is NOT a "real" type, and so its members just how you say extend this

        let scope = node.args[1].parse()?;

        let extent = Extent::from_args(&node.args[1..node.args.len()])?.result;

        let usage = Usage { scope, extent };

        let res = WktBaseTypeResult {
            result: usage,
            consumed: 1,
        };

        Ok(res)
    }
}
