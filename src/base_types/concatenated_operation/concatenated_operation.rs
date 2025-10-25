use crate::{
    arity::lower_bound_arity,
    ast::{Parse, WktArg, WktNode},
    base_types::{OperationAccuracy, OperationVersion, SourceCrs, TargetCrs},
    compound_types::{ScopeExtentIdentifierRemark, Step},
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct ConcatenatedOperation {
    pub operation_name: String,
    pub operation_version: Option<OperationVersion>,
    pub source_crs: SourceCrs,
    pub target_crs: TargetCrs,
    pub step: Vec<Step>,
    pub operation_accuracy: Option<OperationAccuracy>,
    pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl WktBaseType for ConcatenatedOperation {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::ConcatenatedOperation])?;
        lower_bound_arity(node.args.len(), 4)?;

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

        let source_crs = match &node.args[i] {
            WktArg::Node(n) => match SourceCrs::from_nodes(vec![n]) {
                Ok(x) => {
                    i += 1;
                    x.result
                }
                Err(y) => return Err(y),
            },
            _ => return Err(WktParseError::ExpectedNode),
        };

        let target_crs = match &node.args[i] {
            WktArg::Node(n) => match TargetCrs::from_nodes(vec![n]) {
                Ok(x) => {
                    i += 1;
                    x.result
                }
                Err(y) => return Err(y),
            },
            _ => return Err(WktParseError::ExpectedNode),
        };

        let mut step = vec![];

        for _j in i..len {
            let op = match node.args.get(i) {
                Some(a) => match a {
                    WktArg::Node(n) => match Step::from_nodes(vec![n]) {
                        Ok(x) => Some(x.result),
                        Err(_) => None,
                    },
                    _ => None,
                },
                None => break,
            };

            if let Some(o) = op {
                step.push(o);
                i += 1;
            } else {
                break;
            }
        }

        let operation_accuracy = match node.args.get(i) {
            Some(a) => match a {
                WktArg::Node(n) => match OperationAccuracy::from_nodes(vec![n]) {
                    Ok(x) => {
                        i += 1;
                        Some(x.result)
                    }
                    Err(_) => None,
                },
                _ => None,
            },
            None => None,
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

        let compound = ConcatenatedOperation {
            operation_name,
            operation_version,
            source_crs,
            target_crs,
            step,
            operation_accuracy,
            scope_extent_identifier_remark,
        };

        return Ok(WktBaseTypeResult {
            result: compound,
            consumed: 1,
        });
    }
}
