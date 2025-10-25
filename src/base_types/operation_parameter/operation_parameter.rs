use crate::{
    arity::match_arity,
    ast::{Parse, WktNode},
    base_types::Id,
    compound_types::Unit,
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct OperationParameter {
    pub parameter_name: String,
    pub parameter_value: f64,
    pub parameter_unit: Option<Unit>,
    pub identifier: Option<Id>, // TODO: technically allowed multiple of these
}

impl WktBaseType for OperationParameter {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::Parameter])?;
        match_arity(node.args.len(), 1, 4)?;

        let parameter_name = node.args[0].parse()?;
        let parameter_value = node.args[1].parse()?;

        let mut i = 2;
        let parameter_unit = match node.args.get(i) {
            Some(x) => match x.parse() {
                Ok(x) => {
                    i += 1;
                    Some(x)
                }
                Err(_) => None,
            },
            None => None,
        };

        let identifier = match node.args.get(i) {
            Some(x) => Some(x.parse()?),
            None => None,
        };

        let datum = OperationParameter {
            parameter_name,
            parameter_value,
            parameter_unit,
            identifier,
        };

        Ok(WktBaseTypeResult {
            result: datum,
            consumed: 1,
        })
    }
}
