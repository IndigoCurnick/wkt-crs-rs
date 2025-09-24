use crate::{
    ast::{Parse, WktArg, WktNode},
    base_types::{DatumEnsembleAccuracy, DatumEnsembleMember, Ellipsoid, Id, PrimeMeridian},
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct GeodeticDatumEnsemble {
    pub datum_ensemble_name: String,
    pub datum_ensemble_member: Vec<DatumEnsembleMember>,
    pub ellipsoid: Ellipsoid,
    pub datum_ensemble_accuracy: DatumEnsembleAccuracy,
    pub identifier: Option<Id>,
    pub prime_meridian: PrimeMeridian,
}

impl WktBaseType for GeodeticDatumEnsemble {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let mut iterator = wkt_nodes.into_iter();

        // First Part
        let base_node = match iterator.next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&base_node.keyword, vec![Keywords::Ensemble])?;
        // match_arity(node.args.len(), 1, 2)?; // TODO How to handle arity in this case?

        let datum_ensemble_name = base_node.args[0].parse()?;

        let mut datum_ensemble_member = vec![];
        let mut ellipsoid = None;
        let mut datum_ensemble_accuracy = None;
        let mut identifier = None;

        for i in 2..base_node.args.len() {
            let this_value = &base_node.args[i];

            match this_value {
                WktArg::Node(node) => {
                    match node.keyword {
                        Keywords::Member => {
                            // Parameters must come before identifier

                            if ellipsoid.is_some()
                                || datum_ensemble_accuracy.is_some()
                                || identifier.is_some()
                            {
                                return Err(WktParseError::IncorrectKeywordOrder);
                            }

                            let param = node.parse()?;

                            datum_ensemble_member.push(param);
                        }
                        Keywords::Ellipsoid | Keywords::Spheroid => {
                            if datum_ensemble_accuracy.is_some() || identifier.is_some() {
                                return Err(WktParseError::IncorrectKeywordOrder);
                            }

                            if ellipsoid.is_some() {
                                return Err(WktParseError::TooManyKeyword(Keywords::Ellipsoid));
                            }

                            ellipsoid = Some(node.parse()?)
                        }
                        Keywords::EnsembleAccuracy => {
                            if identifier.is_some() {
                                return Err(WktParseError::IncorrectKeywordOrder);
                            }

                            if datum_ensemble_accuracy.is_some() {
                                return Err(WktParseError::TooManyKeyword(
                                    Keywords::EnsembleAccuracy,
                                ));
                            }

                            datum_ensemble_accuracy = Some(node.parse()?);
                        }
                        Keywords::Id => {
                            if identifier.is_some() {
                                return Err(WktParseError::TooManyKeyword(Keywords::Id));
                            }

                            identifier = Some(node.parse()?);
                        }
                        _ => {
                            return Err(WktParseError::IncorrectKeyword {
                                expected: vec![
                                    Keywords::Member,
                                    Keywords::Ellipsoid,
                                    Keywords::Spheroid,
                                    Keywords::EnsembleAccuracy,
                                    Keywords::Id,
                                ]
                                .into(),
                                found: node.keyword.clone(),
                            });
                        }
                    }
                }
                _ => return Err(WktParseError::ExpectedNode),
            }
        }

        // Prime Meridian

        let prime_node = match iterator.next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        let prime_meridian = prime_node.parse()?;

        // Final validation

        let datum_ensemble_accuracy = match datum_ensemble_accuracy {
            Some(x) => x,
            None => return Err(WktParseError::TooFewKeyword(Keywords::EnsembleAccuracy)),
        };

        let ellipsoid = match ellipsoid {
            Some(x) => x,
            None => return Err(WktParseError::TooFewKeyword(Keywords::Ellipsoid)),
        };

        let res = GeodeticDatumEnsemble {
            datum_ensemble_name,
            datum_ensemble_member,
            datum_ensemble_accuracy,
            ellipsoid,
            identifier,
            prime_meridian,
        };

        Ok(WktBaseTypeResult {
            result: res,
            consumed: 2,
        })
    }
}
