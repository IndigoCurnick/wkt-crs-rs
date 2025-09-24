use crate::{
    arity::match_arity,
    ast::{Parse, WktNode},
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct DeformationModelId(pub String);

impl WktBaseType for DeformationModelId {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::Model, Keywords::VelocityGrid])?;
        match_arity(node.args.len(), 1, 1)?;

        let dm = node.args[0].parse()?;

        let accuracy = DeformationModelId(dm);

        Ok(WktBaseTypeResult {
            result: accuracy,
            consumed: 1,
        })
    }
}
