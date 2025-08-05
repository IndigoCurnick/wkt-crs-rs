use crate::{
    ast::{WktArg, WktNode},
    datum::DatumAnchor,
    error::WktParseError,
    keywords::{ANCHOR, ID, PARAMETRICDATUM, PDATUM},
    scope_extent_identifier_remark::Id,
};

#[derive(Debug, PartialEq)]
pub struct ParametricDatum {
    pub datum_name: String,
    pub datum_anchor: Option<DatumAnchor>,
    pub identifier: Option<Id>,
}

impl TryFrom<&WktNode> for ParametricDatum {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == PDATUM || value.keyword == PARAMETRICDATUM) {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![PDATUM.into(), PARAMETRICDATUM.into()].into(),
                found: value.keyword.clone(),
            });
        }

        let len = value.args.len();
        if !(len == 1 || len == 2 || len == 3) {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["1".into(), "2".into(), "3".into()].into(),
                found: len,
            });
        }

        let datum_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let mut datum_anchor = None;
        let mut identifier = None;

        for i in 1..len {
            let this_arg = &value.args[i];

            match this_arg {
                WktArg::Node(node) => match node.keyword.as_str() {
                    ANCHOR => {
                        if datum_anchor.is_some() {
                            return Err(WktParseError::TooManyKeyword(ANCHOR.to_string()));
                        }

                        if identifier.is_some() {
                            return Err(WktParseError::IncorrectKeywordOrder);
                        }

                        datum_anchor = Some(DatumAnchor::try_from(node)?);
                    }
                    ID => {
                        if identifier.is_some() {
                            return Err(WktParseError::TooManyKeyword(ANCHOR.to_string()));
                        }

                        identifier = Some(Id::try_from(node)?);
                    }
                    _ => {
                        return Err(WktParseError::IncorrectKeyword {
                            expected: vec![ANCHOR.into(), ID.into()].into(),
                            found: node.keyword.clone(),
                        });
                    }
                },
                _ => return Err(WktParseError::ExpectedNode),
            }
        }

        return Ok(ParametricDatum {
            datum_name,
            datum_anchor,
            identifier,
        });
    }
}
