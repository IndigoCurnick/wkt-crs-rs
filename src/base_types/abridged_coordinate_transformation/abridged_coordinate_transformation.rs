use crate::{
    arity::lower_bound_arity,
    ast::{Parse, WktArg, WktNode},
    base_types::{OperationMethod, OperationVersion},
    compound_types::ScopeExtentIdentifierRemark,
    enumerations::OperationParameterWrapper,
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct AbridgedCoordinateTransformation {
    pub operation_name: String,
    pub operation_version: Option<OperationVersion>,
    pub operation_method: OperationMethod,
    pub operation_parameter_wrapper: Option<Vec<OperationParameterWrapper>>,
    pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl WktBaseType for AbridgedCoordinateTransformation {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        println!("Node I get: {:?}", node);

        match_keywords(&node.keyword, vec![Keywords::AbridgedTransformation])?;
        lower_bound_arity(node.args.len(), 3)?;

        let operation_name = node.args[0].parse()?;

        let mut i = 1;
        let len = node.args.len();

        let operation_version = match &node.args[i] {
            WktArg::Node(n) => match OperationVersion::from_nodes(vec![n]) {
                Ok(x) => {
                    i += 1;
                    Some(x.result)
                }
                Err(_) => None,
            },
            _ => None,
        };

        let operation_method = match &node.args[i] {
            WktArg::Node(n) => match OperationMethod::from_nodes(vec![n]) {
                Ok(x) => {
                    i += 1;
                    x.result
                }
                Err(y) => return Err(y),
            },
            _ => return Err(WktParseError::ExpectedNode),
        };

        let mut operation_paramters = vec![];

        for j in i..len {
            let op = match node.args.get(i) {
                Some(a) => match a {
                    WktArg::Node(n) => match OperationParameterWrapper::from_nodes(vec![n]) {
                        Ok(x) => Some(x.result),
                        Err(_) => None,
                    },
                    _ => None,
                },
                None => break,
            };

            if let Some(o) = op {
                operation_paramters.push(o);
                i += 1;
            } else {
                break;
            }
        }

        let operation_parameter_wrapper = if operation_paramters.is_empty() {
            None
        } else {
            Some(operation_paramters)
        };

        let maybe_slice = node.args.get(i..len);

        let scope_extent_identifier_remark = match maybe_slice {
            Some(x) => ScopeExtentIdentifierRemark::from_args(x)?.result,
            None => ScopeExtentIdentifierRemark {
                usage: None,
                identifier: None,
                remark: None,
            },
        };

        let compound = AbridgedCoordinateTransformation {
            operation_name,
            operation_version,
            operation_method,
            operation_parameter_wrapper,
            scope_extent_identifier_remark,
        };

        return Ok(WktBaseTypeResult {
            result: compound,
            consumed: 1,
        });
    }
}
