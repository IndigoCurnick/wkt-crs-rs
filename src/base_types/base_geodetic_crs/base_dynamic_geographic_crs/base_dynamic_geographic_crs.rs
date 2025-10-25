use crate::{
    arity::match_arity,
    ast::{Parse, WktArg, WktNode},
    base_types::{AngleUnit, DynamicCrs, GeodeticReferenceFrame, Id},
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult, WktInlineResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct BaseDynamicGeographicCrs {
    pub base_crs_name: String,
    pub dynamic_crs: DynamicCrs,
    pub geodetic_data: GeodeticReferenceFrame,
    pub ellipsoidal_cs_unit: Option<AngleUnit>,
    pub identifier: Option<Id>,
}

impl WktInlineType for BaseDynamicGeographicCrs {
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

        let res = BaseDynamicGeographicCrs::from_nodes(nodes)?;

        return Ok(WktInlineResult {
            consumed: res.consumed,
            result: res.result,
        });
    }
}

impl WktBaseType for BaseDynamicGeographicCrs {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::BaseGeogCrs])?;
        match_arity(node.args.len(), 3, 5)?;

        let base_crs_name = node.args[0].parse()?;
        let dynamic_crs = node.args[1].parse()?;
        let geodetic_data = node.args[2].parse()?;

        let mut i = 3;

        let ellipsoidal_cs_unit = match node.args.get(i) {
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

        let res = BaseDynamicGeographicCrs {
            base_crs_name,
            geodetic_data,
            dynamic_crs,
            ellipsoidal_cs_unit,
            identifier,
        };

        Ok(WktBaseTypeResult {
            result: res,
            consumed: 1,
        })
    }
}
