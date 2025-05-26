use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::MEMBER,
    scope_extent_identifier_remark::Id,
};

#[derive(Debug, PartialEq)]
pub struct EnsembleMember {
    pub ensemble_member_name: String,
    pub identifier: Option<Id>, // TODO: Technically the spec allows multiple
}

impl TryFrom<&WktNode> for EnsembleMember {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != MEMBER {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![MEMBER.into()].into(),
                found: value.keyword.clone(),
            });
        }

        if !(value.args.len() == 1 || value.args.len() == 2) {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["1".into(), "2".into()].into(),
                found: value.args.len(),
            });
        }

        let ensemble_member_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let identifier = match value.args.get(1) {
            Some(a) => match a {
                WktArg::Node(n) => Some(Id::try_from(n)?),
                _ => return Err(WktParseError::ExpectedNode),
            },
            None => None,
        };

        return Ok(EnsembleMember {
            ensemble_member_name,
            identifier,
        });
    }
}
