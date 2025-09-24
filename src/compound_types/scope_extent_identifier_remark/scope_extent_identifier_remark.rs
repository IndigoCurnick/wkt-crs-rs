use crate::{
    ast::{Parse, WktArg},
    base_types::{Id, Remark, Usage},
    error::WktParseError,
    keywords::Keywords,
    types::{WktInlineResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct ScopeExtentIdentifierRemark {
    pub usage: Option<Vec<Usage>>,
    pub identifier: Option<Vec<Id>>,
    pub remark: Option<Remark>,
}

impl WktInlineType for ScopeExtentIdentifierRemark {
    fn from_args<'a, I>(wkt_args: I) -> Result<WktInlineResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktArg>,
    {
        let mut usage_inner = vec![];
        let mut identifier_inner = vec![];
        let mut remark = None;

        let mut i = 0;

        let mut it = wkt_args.into_iter();

        while let Some(arg) = it.next() {
            match arg {
                WktArg::Node(node) => match node.keyword {
                    Keywords::Usage => {
                        usage_inner.push(node.parse()?);
                        i += 1;
                    }
                    Keywords::Id => {
                        identifier_inner.push(node.parse()?);
                        i += 1;
                    }
                    Keywords::Remark => {
                        remark = Some(node.parse()?);
                        i += 1;
                    }
                    _ => {
                        return Err(WktParseError::IncorrectKeyword {
                            expected: vec![Keywords::Usage, Keywords::Id, Keywords::Remark].into(),
                            found: node.keyword.clone(),
                        });
                    }
                },
                _ => return Err(WktParseError::ExpectedNode),
            }
        }

        let usage = if usage_inner.is_empty() {
            None
        } else {
            Some(usage_inner)
        };

        let identifier = if identifier_inner.is_empty() {
            None
        } else {
            Some(identifier_inner)
        };

        let seir = ScopeExtentIdentifierRemark {
            usage,
            identifier,
            remark,
        };

        let res = WktInlineResult {
            consumed: i,
            result: seir,
        };

        Ok(res)
    }
}
