use crate::{
    arity::match_arity,
    ast::{Parse, WktNode},
    base_types::{Id, TemporalDatum},
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct BaseTemporalCrs {
    pub base_crs_name: String,
    pub temporal_datum: TemporalDatum,
    pub identifier: Option<Id>, // TODO: Technically allowed multiple
}

impl WktBaseType for BaseTemporalCrs {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::BaseTimeCrs])?;
        match_arity(node.args.len(), 2, 3);

        let base_crs_name = node.args[0].parse()?;
        let temporal_datum = node.args[1].parse()?;

        let identifier = match node.args.get(2) {
            Some(x) => Some(x.parse()?),
            None => None,
        };

        let res = BaseTemporalCrs {
            base_crs_name,
            temporal_datum,
            identifier,
        };

        Ok(WktBaseTypeResult {
            result: res,
            consumed: 1,
        })
    }
}
