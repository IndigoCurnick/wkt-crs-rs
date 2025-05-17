use crate::{
    ast::{WktArg, WktNode},
    citation::Citation,
    error::WktParseError,
    keywords::{CITATION, ID, URI},
    uri::Uri,
    utils::NumText,
};

#[derive(Debug, PartialEq)]
pub struct Id {
    pub authority_name: String,
    pub authority_unique_identifier: NumText,
    pub version: Option<NumText>,
    pub authority_citation: Option<Citation>,
    pub id_uri: Option<Uri>,
}

impl TryFrom<&WktNode> for Id {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != ID {
            let expected = vec![ID.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if !(value.args.len() >= 2 || value.args.len() <= 5) {
            return Err(WktParseError::IncorrectArity {
                expected: vec![
                    "2".to_string(),
                    "3".to_string(),
                    "4".to_string(),
                    "5".to_string(),
                ]
                .into(),
                found: value.args.len(),
            });
        }

        let auth_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let auth_uniq = match &value.args[1] {
            WktArg::String(s) => NumText::Text(s.clone()),
            WktArg::Number(n) => NumText::Num(n.clone()),
            _ => return Err(WktParseError::ExpectedStringOrNumber),
        };

        // Beyond here is optional
        let mut version = None;
        let mut auth_cite = None;
        let mut id_uri = None;

        for i in 2..value.args.len() {
            match &value.args[i] {
                WktArg::String(s) => {
                    // Version string as text

                    // Already have version
                    if version.is_some() {
                        return Err(WktParseError::ExpectedNode);
                    }

                    // Gone past version i.e. auth_cite or id_uri is already present
                    // (Version must be before auth cite and id_uri)
                    if auth_cite.is_some() || id_uri.is_some() {
                        return Err(WktParseError::ExpectedNode);
                    }

                    version = Some(NumText::Text(s.clone()));
                }
                WktArg::Number(n) => {
                    // Version string as number

                    // Already have version
                    if version.is_some() {
                        return Err(WktParseError::ExpectedNode);
                    }

                    // Gone past version i.e. auth_cite or id_uri is already present
                    // (Version must be before auth cite and id_uri)
                    if auth_cite.is_some() || id_uri.is_some() {
                        return Err(WktParseError::ExpectedNode);
                    }

                    version = Some(NumText::Num(n.clone()));
                }
                WktArg::Node(wkt_node) => {
                    // Either Authority Citation or URI

                    match wkt_node.keyword.as_str() {
                        CITATION => {
                            // Already have citation
                            if auth_cite.is_some() {
                                return Err(WktParseError::TooManyKeyword(CITATION.to_string()));
                            }

                            // Authority citation must appear before URI
                            if id_uri.is_some() {
                                return Err(WktParseError::IncorrectKeywordOrder);
                            }

                            // Parse citation
                            auth_cite = match Citation::try_from(wkt_node) {
                                Ok(x) => Some(x),
                                Err(y) => return Err(y),
                            };
                        }
                        URI => {
                            // Already have URI
                            if id_uri.is_some() {
                                return Err(WktParseError::TooManyKeyword(URI.to_string()));
                            }

                            id_uri = match Uri::try_from(wkt_node) {
                                Ok(x) => Some(x),
                                Err(y) => return Err(y),
                            }
                        }
                        _ => {
                            return Err(WktParseError::IncorrectKeyword {
                                expected: vec![CITATION.to_string(), URI.to_string()].into(),
                                found: wkt_node.keyword.clone(),
                            });
                        }
                    }
                }
            }
        }

        return Ok(Id {
            authority_name: auth_name,
            authority_unique_identifier: auth_uniq,
            version: version,
            authority_citation: auth_cite,
            id_uri: id_uri,
        });
    }
}
