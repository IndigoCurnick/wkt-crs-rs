use crate::{
    arity::match_arity,
    ast::{Parse, WktArg, WktNode},
    base_types::{Id, TemporalCountMeasureAxis},
    enumerations::{Dimension, TemporalCountMeasureCsType},
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult, WktInlineResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct TemporalCountMeasureCoordinateSystem {
    pub temporal_count_measure_cs_type: TemporalCountMeasureCsType,
    pub dimension: Dimension,
    pub identifier: Option<Id>, // TODO: Technically this can be multiple
    pub temporal_count_measure_axis: TemporalCountMeasureAxis,
}

impl WktInlineType for TemporalCountMeasureCoordinateSystem {
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

        let res = TemporalCountMeasureCoordinateSystem::from_nodes(nodes)?;

        return Ok(WktInlineResult {
            consumed: res.consumed,
            result: res.result,
        });
    }
}

impl WktBaseType for TemporalCountMeasureCoordinateSystem {
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

        let temporal_count_measure_cs_type = TemporalCountMeasureCsType::try_from(&node.args[0])?;
        let dimension = Dimension::try_from(&node.args[1])?;

        let identifier = match node.args.get(2) {
            Some(x) => Some(x.parse()?),
            None => None,
        };

        let axis_node = match iter.next() {
            Some(x) => x,
            None => return Err(WktParseError::ExpectedNode),
        };

        let temporal_count_measure_axis = axis_node.parse()?;

        let cs = TemporalCountMeasureCoordinateSystem {
            temporal_count_measure_cs_type,
            dimension,
            identifier,
            temporal_count_measure_axis,
        };

        let res = WktBaseTypeResult {
            consumed: 2,
            result: cs,
        };

        Ok(res)
    }
}
