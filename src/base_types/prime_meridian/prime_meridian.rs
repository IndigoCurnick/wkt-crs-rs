use crate::{
    arity::match_arity,
    ast::{Parse, WktArg, WktNode},
    base_types::{AngleUnit, Id},
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktResult},
};

#[derive(Debug, PartialEq)]
pub struct PrimeMeridian {
    pub prime_meridian_name: String,
    pub irm_longitude: f64,
    pub angle_unit: Option<AngleUnit>, // * Note that in the spec, `irm_longitude` and `angle_unit` are wrapped into one flat group
    pub identifier: Option<Id>,
}

impl WktBaseType for PrimeMeridian {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::Id])?;
        match_arity(node.args.len(), 2, 4)?;

        let prime_meridian_name = node.args[0].parse()?;
        let irm_longitude = node.args[1].parse()?;

        // Beyond here is optional
        let mut angle_unit = None;
        let mut identifier = None;

        for i in 2..node.args.len() {
            match &node.args[i] {
                WktArg::Data(_) => return Err(WktParseError::ExpectedNode),
                WktArg::Node(wkt_node) => match wkt_node.keyword {
                    Keywords::AngleUnit | Keywords::Unit => {
                        if angle_unit.is_some() {
                            return Err(WktParseError::TooManyKeyword(Keywords::AngleUnit));
                        }

                        if identifier.is_some() {
                            return Err(WktParseError::IncorrectKeywordOrder);
                        }

                        angle_unit = Some(AngleUnit::from_nodes(vec![wkt_node])?.result);
                    }
                    Keywords::Id => {
                        if identifier.is_some() {
                            return Err(WktParseError::TooManyKeyword(Keywords::Id));
                        }

                        identifier = Some(Id::from_nodes(vec![wkt_node])?.result);
                    }
                    _ => {
                        return Err(WktParseError::IncorrectKeyword {
                            expected: vec![Keywords::AngleUnit, Keywords::Unit, Keywords::Id]
                                .into(),
                            found: node.keyword.clone(),
                        });
                    }
                },
            }
        }

        let pm = PrimeMeridian {
            prime_meridian_name,
            irm_longitude,
            angle_unit,
            identifier,
        };

        return Ok(WktResult {
            result: pm,
            consumed: 1,
        });
    }
}
