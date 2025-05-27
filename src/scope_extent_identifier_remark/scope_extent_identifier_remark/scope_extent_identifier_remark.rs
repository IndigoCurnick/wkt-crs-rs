use crate::{
    ast::WktArg,
    error::WktParseError,
    keywords::{ID, REMARK, USAGE},
    scope_extent_identifier_remark::{Id, Remark, usage::Usage},
};

#[derive(Debug, PartialEq)]
pub struct ScopeExtentIdentifierRemark {
    pub usage: Option<Vec<Usage>>,
    pub identifier: Option<Vec<Id>>,
    pub remark: Option<Remark>,
}

// TODO: Tests needed badly!!
impl TryFrom<&[WktArg]> for ScopeExtentIdentifierRemark {
    type Error = WktParseError;

    // TODO: We will assume that this is always coming at the end so that everything
    // in the provided array is part of the `ScopeExtentIdentifierRemark` structure
    // But this might not necessarily always be true so maybe something more robust?
    fn try_from(value: &[WktArg]) -> Result<Self, Self::Error> {
        let mut usage_inner = vec![];
        let mut identifier_inner = vec![];
        let mut remark = None;

        for arg in value {
            match arg {
                WktArg::Node(node) => match node.keyword.as_str() {
                    USAGE => {
                        usage_inner.push(Usage::try_from(node)?);
                    }
                    ID => {
                        identifier_inner.push(Id::try_from(node)?);
                    }
                    REMARK => remark = Some(Remark::try_from(node)?),
                    _ => {
                        return Err(WktParseError::IncorrectKeyword {
                            expected: vec![USAGE.into(), ID.into(), REMARK.into()].into(),
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

        return Ok(ScopeExtentIdentifierRemark {
            usage,
            identifier,
            remark,
        });
    }
}
