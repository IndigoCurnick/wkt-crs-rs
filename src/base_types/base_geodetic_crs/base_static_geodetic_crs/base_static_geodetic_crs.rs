use crate::{
    arity::match_arity,
    ast::{Parse, WktNode},
    base_types::{AngleUnit, Id},
    compound_types::GeodeticData,
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct BaseStaticGeodeticCrs {
    pub base_crs_name: String,
    pub geodetic_data: GeodeticData,
    pub ellipsoidal_cs_unit: Option<AngleUnit>,
    pub identifier: Option<Id>,
}

impl WktBaseType for BaseStaticGeodeticCrs {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::BaseGeodCrs])?;
        match_arity(node.args.len(), 2, 4);

        let base_crs_name = node.args[0].parse()?;
        let geodetic_data = node.args[1].parse()?;

        let mut i = 2;

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

        let res = BaseStaticGeodeticCrs {
            base_crs_name,
            geodetic_data,
            ellipsoidal_cs_unit,
            identifier,
        };

        Ok(WktBaseTypeResult {
            result: res,
            consumed: 1,
        })
    }
}
