use crate::{
    arity::match_arity,
    ast::{Parse, WktArg, WktNode},
    base_types::{Citation, Uri},
    data_types::NumText,
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktResult},
};

#[derive(Debug, PartialEq)]
pub struct Id {
    pub authority_name: String,
    pub authority_unique_identifier: NumText,
    pub version: Option<NumText>,
    pub authority_citation: Option<Citation>,
    pub id_uri: Option<Uri>,
}

impl WktBaseType for Id {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::Id])?;
        match_arity(node.args.len(), 2, 5)?;

        let authority_name = node.args[0].parse()?;
        let authority_unique_identifier = node.args[1].parse()?;

        // Beyond here is optional
        let mut version = None;
        let mut authority_citation = None;
        let mut id_uri = None;

        for i in 2..node.args.len() {
            match &node.args[i] {
                WktArg::Data(s) => {
                    // Already have version
                    if version.is_some() {
                        return Err(WktParseError::ExpectedNode);
                    }

                    // Gone past version i.e. auth_cite or id_uri is already present
                    // (Version must be before auth cite and id_uri)
                    if authority_citation.is_some() || id_uri.is_some() {
                        return Err(WktParseError::ExpectedNode);
                    }

                    version = Some(NumText::from(s.as_str()));
                }
                WktArg::Node(wkt_node) => {
                    // Either Authority Citation or URI

                    match wkt_node.keyword {
                        Keywords::Citation => {
                            // Already have citation
                            if authority_citation.is_some() {
                                return Err(WktParseError::TooManyKeyword(Keywords::Citation));
                            }

                            // Authority citation must appear before URI
                            if id_uri.is_some() {
                                return Err(WktParseError::IncorrectKeywordOrder);
                            }

                            // Parse citation
                            authority_citation = match Citation::from_nodes(vec![wkt_node]) {
                                Ok(x) => Some(x.result),
                                Err(y) => return Err(y),
                            };
                        }
                        Keywords::Uri => {
                            // Already have URI
                            if id_uri.is_some() {
                                return Err(WktParseError::TooManyKeyword(Keywords::Uri));
                            }

                            id_uri = match Uri::from_nodes(vec![wkt_node]) {
                                Ok(x) => Some(x.result),
                                Err(y) => return Err(y),
                            }
                        }
                        _ => {
                            return Err(WktParseError::IncorrectKeyword {
                                expected: vec![Keywords::Citation, Keywords::Uri].into(),
                                found: wkt_node.keyword.clone(),
                            });
                        }
                    }
                }
            }
        }

        let id = Id {
            authority_name,
            authority_unique_identifier,
            version,
            authority_citation,
            id_uri,
        };

        return Ok(WktResult {
            result: id,
            consumed: 1,
        });
    }
}
