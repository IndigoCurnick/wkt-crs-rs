use crate::{
    arity::match_arity,
    ast::{Parse, WktArg, WktNode},
    base_types::{Id, OrdinalDateTimeAxis},
    enumerations::{Dimension, OrdinalDateTimeCsType},
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult, WktInlineResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct OrdinalDateTimeCoordinateSystem {
    pub ordinal_date_time_cs_type: OrdinalDateTimeCsType,
    pub dimension: Dimension,
    pub identifier: Option<Id>, // TODO: technically the spec allows for many...
    pub ordinal_date_time_axis: Vec<OrdinalDateTimeAxis>,
}

impl WktInlineType for OrdinalDateTimeCoordinateSystem {
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

        let res = OrdinalDateTimeCoordinateSystem::from_nodes(nodes)?;

        return Ok(WktInlineResult {
            consumed: res.consumed,
            result: res.result,
        });
    }
}

impl WktBaseType for OrdinalDateTimeCoordinateSystem {
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
        println!("found a CS keyword");
        println!("I am about to try and parse: {:?}", node.args[0]);
        let ordinal_date_time_cs_type = OrdinalDateTimeCsType::try_from(&node.args[0])?;
        println!("got cs type");
        let dimension = Dimension::try_from(&node.args[1])?;

        let identifier = match node.args.get(2) {
            Some(x) => Some(x.parse()?),
            None => None,
        };

        let mut ordinal_date_time_axis = vec![];

        let mut i = 1;

        while let Some(next) = iter.next() {
            let axis = match next.parse() {
                Ok(x) => x,
                Err(_) => break,
            };

            i += 1;

            ordinal_date_time_axis.push(axis);
        }

        let cs = OrdinalDateTimeCoordinateSystem {
            dimension,
            identifier,
            ordinal_date_time_axis,
            ordinal_date_time_cs_type,
        };

        let res = WktBaseTypeResult {
            consumed: i,
            result: cs,
        };

        Ok(res)
    }
}
