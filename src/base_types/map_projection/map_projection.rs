use crate::{
    arity::lower_bound_arity,
    ast::{Parse, WktArg, WktNode},
    base_types::{Id, MapProjectionMethod, MapProjectionParameter},
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct MapProjection {
    pub map_projection_name: String,
    pub map_projection_method: MapProjectionMethod,
    pub map_projection_parameters: Option<Vec<MapProjectionParameter>>,
    pub identifier: Option<Id>, // TODO: Technically the spec allows for multiple
}

impl WktBaseType for MapProjection {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::Conversion])?;
        lower_bound_arity(node.args.len(), 2)?;

        let map_projection_name = node.args[0].parse()?;
        let map_projection_method = node.args[1].parse()?;

        let mut v = vec![];
        let mut identifier = None;

        for i in 2..node.args.len() {
            let this_value = &node.args[i];

            match this_value {
                WktArg::Node(node) => {
                    match node.keyword {
                        Keywords::Parameter => {
                            // Parameters must come before identifier

                            if identifier.is_some() {
                                return Err(WktParseError::IncorrectKeywordOrder);
                            }

                            let param = node.parse()?;

                            v.push(param);
                        }
                        Keywords::Id => {
                            identifier = Some(node.parse()?);
                        }
                        _ => {
                            return Err(WktParseError::IncorrectKeyword {
                                expected: vec![Keywords::Parameter, Keywords::Id].into(),
                                found: node.keyword.clone(),
                            });
                        }
                    }
                }
                _ => return Err(WktParseError::ExpectedNode),
            }
        }

        let map_projection_parameters = if v.is_empty() { None } else { Some(v) };

        let m = Self {
            identifier,
            map_projection_method,
            map_projection_name,
            map_projection_parameters,
        };

        Ok(WktBaseTypeResult {
            result: m,
            consumed: 1,
        })
    }
}
