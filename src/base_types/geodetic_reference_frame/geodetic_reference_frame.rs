use crate::{
    arity::match_arity,
    ast::{Parse, WktNode},
    base_types::{DatumAnchor, Ellipsoid, Id, PrimeMeridian},
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct GeodeticReferenceFrame {
    pub datum_name: String,
    pub ellipsoid: Ellipsoid,
    pub anchor: Option<DatumAnchor>,
    pub identifier: Option<Id>, // TODO: technically multiple allowed
    pub prime_meridian: Option<PrimeMeridian>,
}

impl WktBaseType for GeodeticReferenceFrame {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let mut it = wkt_nodes.into_iter();

        let node = match it.next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(
            &node.keyword,
            vec![Keywords::Datum, Keywords::TRF, Keywords::GeodeticDatum],
        )?;
        match_arity(node.args.len(), 2, 4)?;

        let datum_name = node.args[0].parse()?;
        let ellipsoid = node.args[1].parse()?;

        let mut i = 2;

        let anchor = match node.args.get(i) {
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

        // Second node

        let prime_meridian = match it.next() {
            Some(second) => match second.parse() {
                Ok(x) => Some(x),
                Err(_) => None,
            },
            None => None,
        };

        let consumed = if prime_meridian.is_some() { 2 } else { 1 };

        let datum = GeodeticReferenceFrame {
            datum_name,
            ellipsoid,
            anchor,
            identifier,
            prime_meridian,
        };

        let res = WktBaseTypeResult {
            consumed: consumed,
            result: datum,
        };

        Ok(res)
    }
}
