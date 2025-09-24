use crate::{
    arity::match_arity,
    ast::{Parse, WktNode},
    base_types::{DatumAnchor, Id},
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct VerticalReferenceFrame {
    pub datum_name: String,
    pub datum_anchor: Option<DatumAnchor>,
    pub identifier: Option<Id>,
}

impl WktBaseType for VerticalReferenceFrame {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        // Take 1

        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(
            &node.keyword,
            vec![Keywords::VDatum, Keywords::VRF, Keywords::VerticalDatum],
        )?;
        match_arity(node.args.len(), 1, 3);

        let datum_name = node.args[0].parse()?;

        let mut i = 1;

        // ? wait isn't this going to fail if we have an id but not an anchor?
        let datum_anchor = match node.args.get(i) {
            Some(x) => {
                i += 1;
                Some(x.parse()?)
            }
            None => None,
        };

        let identifier = match node.args.get(i) {
            Some(x) => Some(x.parse()?),
            None => None,
        };

        let res = VerticalReferenceFrame {
            datum_name,
            datum_anchor,
            identifier,
        };

        return Ok(WktBaseTypeResult {
            result: res,
            consumed: 1,
        });
    }
}
