use crate::{
    arity::match_arity,
    ast::{Parse, WktArg, WktNode},
    base_types::{AngleUnit, Id},
    compound_types::GeodeticData,
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult, WktInlineResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct BaseStaticGeographicCrs {
    pub base_crs_name: String,
    pub geodetic_data: GeodeticData,
    pub ellipsoidal_cs_unit: Option<AngleUnit>,
    pub identifier: Option<Id>,
}

impl WktInlineType for BaseStaticGeographicCrs {
    fn from_args<'a, I>(wkt_args: I) -> Result<WktInlineResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a crate::ast::WktArg>,
    {
        // Note that only nodes are necessary, so we can basically iterate the
        // args, consuming all nodes
        // when we hit not a node we can stop and just throw it into the base type
        let mut it = wkt_args.into_iter();

        let mut nodes = vec![];

        while let Some(arg) = it.next() {
            let node = match arg {
                WktArg::Node(n) => n,
                _ => break,
            };

            nodes.push(node);
        }

        let res = BaseStaticGeographicCrs::from_nodes(nodes)?;

        return Ok(WktInlineResult {
            consumed: res.consumed,
            result: res.result,
        });
    }
}

impl WktBaseType for BaseStaticGeographicCrs {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::BaseGeogCrs])?;
        match_arity(node.args.len(), 2, 4)?;

        let base_crs_name = node.args[0].parse()?;
        let geodetic_data = GeodeticData::from_args(&node.args[1..node.args.len()])?;

        let mut i = 1 + geodetic_data.consumed;

        let ellipsoidal_cs_unit = match node.args.get(i) {
            Some(x) => {
                if let Ok(ans) = x.parse() {
                    i += 1;
                    Some(ans)
                } else {
                    None
                }
            }
            None => None,
        };

        let identifier = match node.args.get(i) {
            Some(x) => Some(x.parse()?),
            None => None,
        };

        let res = BaseStaticGeographicCrs {
            base_crs_name,
            geodetic_data: geodetic_data.result,
            ellipsoidal_cs_unit,
            identifier,
        };

        Ok(WktBaseTypeResult {
            result: res,
            consumed: 1,
        })
    }
}
