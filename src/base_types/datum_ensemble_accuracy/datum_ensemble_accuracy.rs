use crate::{
    arity::match_arity,
    ast::{Parse, WktNode},
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct DatumEnsembleAccuracy(pub f64); // ? could this be an int?

impl WktBaseType for DatumEnsembleAccuracy {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::EnsembleAccuracy])?;
        match_arity(node.args.len(), 1, 1)?;

        let acc = node.args[0].parse()?;

        let accuracy = DatumEnsembleAccuracy(acc);

        Ok(WktBaseTypeResult {
            result: accuracy,
            consumed: 1,
        })
    }
}
