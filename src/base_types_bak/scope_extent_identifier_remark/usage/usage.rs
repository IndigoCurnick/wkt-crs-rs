use crate::{
    ast::WktNode,
    error::WktParseError,
    keywords::USAGE,
    scope_extent_identifier_remark::{Scope, extent::Extent},
};

#[derive(Debug, PartialEq)]
pub struct Usage {
    pub scope: Scope,
    pub extent: Extent,
}

impl TryFrom<&WktNode> for Usage {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != USAGE {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec!["USAGE".into()].into(),
                found: value.keyword.clone(),
            });
        }

        if value.args.len() < 2 || value.args.len() > 5 {
            // ?: 1-5? How many extent are you allowed? None?
            return Err(WktParseError::IncorrectArity {
                expected: vec!["2-5".into()].into(),
                found: value.args.len(),
            });
        }

        let scope = Scope::try_from(&value.args[0])?;

        // TODO: The probability that I did the array indicies correct here is
        // approaching 0 at a rapid rate! Pls test!!
        let extent = Extent::try_from(&value.args[1..value.args.len()])?;

        return Ok(Usage { scope, extent });
    }
}
