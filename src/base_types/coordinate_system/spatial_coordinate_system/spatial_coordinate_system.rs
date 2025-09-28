use crate::{
    arity::match_arity,
    ast::{Parse, WktArg, WktNode},
    base_types::{Id, SpatialAxis},
    compound_types::Unit,
    enumerations::{Dimension, SpatialCsType},
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult, WktInlineResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct SpatialCoordinateSystem {
    pub spatial_cs_type: SpatialCsType,
    pub dimension: Dimension,
    pub identifier: Option<Id>, // TODO: Technically the spec allows for any number of these
    pub spatial_axis: Vec<SpatialAxis>,
    pub cs_unit: Option<Unit>,
}

impl WktInlineType for SpatialCoordinateSystem {
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

        let res = SpatialCoordinateSystem::from_nodes(nodes)?;

        return Ok(WktInlineResult {
            consumed: res.consumed,
            result: res.result,
        });
    }
}

impl WktBaseType for SpatialCoordinateSystem {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let mut iter = wkt_nodes.into_iter();

        let node = match iter.next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::Cs])?;

        match_arity(node.args.len(), 1, 3)?;

        let spatial_cs_type = SpatialCsType::try_from(&node.args[0])?;

        let dimension = Dimension::try_from(&node.args[1])?;

        let identifier = match node.args.get(2) {
            Some(x) => Some(x.parse()?),
            None => None,
        };

        let mut spatial_axis = vec![];
        let mut cs_unit = None;

        let mut i = 1;

        while let Some(next) = iter.next() {
            match next.keyword {
                Keywords::Axis => {
                    if cs_unit.is_some() {
                        return Err(WktParseError::IncorrectKeywordOrder);
                    }

                    let axis = next.parse()?;

                    i += 1;

                    spatial_axis.push(axis);
                }
                Keywords::LengthUnit
                | Keywords::AngleUnit
                | Keywords::ScaleUnit
                | Keywords::ParametricUnit
                | Keywords::TimeUnit => {
                    let unit = next.parse()?;
                    i += 1;
                    cs_unit = Some(unit);
                    break;
                }
                _ => break,
            }
        }

        let cs = SpatialCoordinateSystem {
            spatial_cs_type,
            dimension,
            identifier,
            spatial_axis,
            cs_unit,
        };

        let res = WktBaseTypeResult {
            consumed: i,
            result: cs,
        };

        Ok(res)
    }
}
