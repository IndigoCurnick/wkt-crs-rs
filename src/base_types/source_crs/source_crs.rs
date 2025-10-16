use crate::{
    arity::lower_bound_arity,
    ast::WktNode,
    base_types::CoordinateSystem,
    compound_types::CoordinateReferenceSystem,
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct SourceCrs {
    pub coordinate_system: CoordinateReferenceSystem,
}

impl WktBaseType for SourceCrs {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };
        match_keywords(&node.keyword, vec![Keywords::SourceCrs])?;
        lower_bound_arity(node.args.len(), 1)?;

        let coordinate_system = CoordinateReferenceSystem::from_args(&node.args)?;

        let res = SourceCrs {
            coordinate_system: coordinate_system.result,
        };

        Ok(WktBaseTypeResult {
            result: res,
            consumed: 1,
        })
    }
}
