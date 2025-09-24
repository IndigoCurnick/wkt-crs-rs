use crate::{
    ast::WktNode,
    compound_types::{ScopeExtentIdentifierRemark, SingleCrs},
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult, WktInlineType},
};

pub struct CompoundCrs {
    pub crs_one: SingleCrs,
    pub crs_two: SingleCrs,
    pub additional_crs: Option<Vec<SingleCrs>>,
    pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl WktBaseType for CompoundCrs {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::CompoundCrs])?;

        let mut i = 0;
        let len = node.args.len();

        let crs_one = SingleCrs::from_args(&node.args[i..len])?;
        i += crs_one.consumed;

        let crs_two = SingleCrs::from_args(&node.args[i..len])?;
        i += crs_two.consumed;

        let mut crss = vec![];

        loop {
            let res = SingleCrs::from_args(&node.args[i..len]);

            let tmp = match res {
                Ok(x) => x,
                Err(_) => break,
            };

            i += tmp.consumed;
            crss.push(tmp.result);
        }

        let additional_crs = if crss.is_empty() { None } else { Some(crss) };

        let scope_extent_identifier_remark =
            ScopeExtentIdentifierRemark::from_args(&node.args[i..len])?;

        let consumed = i + scope_extent_identifier_remark.consumed;

        let compound = CompoundCrs {
            crs_one: crs_one.result,
            crs_two: crs_two.result,
            additional_crs,
            scope_extent_identifier_remark: scope_extent_identifier_remark.result,
        };

        return Ok(WktBaseTypeResult {
            result: compound,
            consumed,
        });
    }
}
