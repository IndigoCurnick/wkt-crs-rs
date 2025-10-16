use crate::{
    arity::match_arity,
    ast::{Parse, WktNode},
    base_types::Id,
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct OperationParameterFile {
    pub parameter_name: String,
    pub parameter_file_name: String,
    pub identifier: Option<Id>, // TODO: technically allowed multiple of these
}

impl WktBaseType for OperationParameterFile {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::ParameterFile])?;
        match_arity(node.args.len(), 1, 3)?;

        let parameter_name = node.args[0].parse()?;
        let parameter_file_name = node.args[1].parse()?;
        let identifier = match node.args.get(2) {
            Some(x) => Some(x.parse()?),
            None => None,
        };

        let datum = OperationParameterFile {
            parameter_name,
            parameter_file_name,
            identifier,
        };

        Ok(WktBaseTypeResult {
            result: datum,
            consumed: 1,
        })
    }
}
