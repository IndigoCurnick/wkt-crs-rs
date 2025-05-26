use crate::{
    ast::{WktArg, WktNode},
    ellipsoid::Ellipsoid,
    ensemble::{ensemble_accuracy::EnsembleAccuracy, ensemble_member::EnsembleMember},
    error::WktParseError,
    keywords::{ELLIPSOID, ENSEMBLE, ENSEMBLEACCURACY, ID, MEMBER, SPHEROID},
    prime_meridian::PrimeMeridian,
    scope_extent_identifier_remark::Id,
};

#[derive(Debug, PartialEq)]
pub struct GeodeticEnsemble {
    pub datum_ensemble_name: String,
    pub datum_ensemble_member: Vec<EnsembleMember>,
    pub ellipsoid: Ellipsoid,
    pub datum_ensemble_accuracy: EnsembleAccuracy,
    pub prime_meridian: PrimeMeridian,
}

impl TryFrom<(&WktNode, PrimeMeridian)> for GeodeticEnsemble {
    type Error = WktParseError;

    fn try_from(value: (&WktNode, PrimeMeridian)) -> Result<Self, Self::Error> {
        let (value, pm) = value;

        if value.keyword != ENSEMBLE {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![ENSEMBLE.into()].into(),
                found: value.keyword.clone(),
            });
        }

        if value.args.len() < 3 {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["3+".into()].into(),
                found: value.args.len(),
            });
        }

        let datum_ensemble_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let mut datum_ensemble_member = vec![];
        let mut ellipsoid_op = None;
        let mut datum_ensemble_accuracy_op = None;
        let mut identifier = None;

        for i in 1..value.args.len() {
            let this_arg = &value.args[i];

            match this_arg {
                WktArg::Node(node) => match node.keyword.as_str() {
                    MEMBER => {
                        // Before acc and id
                        if datum_ensemble_accuracy_op.is_some()
                            || identifier.is_some()
                            || ellipsoid_op.is_some()
                        {
                            return Err(WktParseError::IncorrectKeywordOrder);
                        }

                        datum_ensemble_member.push(EnsembleMember::try_from(node)?);
                    }
                    ELLIPSOID | SPHEROID => {
                        // After members, before acc/id
                        if datum_ensemble_member.is_empty()
                            || datum_ensemble_accuracy_op.is_some()
                            || identifier.is_some()
                        {
                            return Err(WktParseError::IncorrectKeywordOrder);
                        }

                        if ellipsoid_op.is_some() {
                            return Err(WktParseError::TooManyKeyword(ELLIPSOID.into()));
                        }

                        ellipsoid_op = Some(Ellipsoid::try_from(node)?);
                    }
                    ENSEMBLEACCURACY => {
                        // Before ID / After members ellipsoid
                        if datum_ensemble_member.is_empty()
                            || identifier.is_some()
                            || ellipsoid_op.is_none()
                        {
                            return Err(WktParseError::IncorrectKeywordOrder);
                        }

                        if datum_ensemble_accuracy_op.is_some() {
                            return Err(WktParseError::TooManyKeyword(ENSEMBLEACCURACY.into()));
                        }

                        datum_ensemble_accuracy_op = Some(EnsembleAccuracy::try_from(node)?);
                    }
                    ID => {
                        // After member and accuracy and ellipsoid
                        if datum_ensemble_member.is_empty()
                            || datum_ensemble_accuracy_op.is_none()
                            || ellipsoid_op.is_none()
                        {
                            return Err(WktParseError::IncorrectKeywordOrder);
                        }

                        if identifier.is_some() {
                            return Err(WktParseError::TooManyKeyword(ID.into()));
                        }

                        identifier = Some(Id::try_from(node)?);
                    }
                    _ => {
                        return Err(WktParseError::IncorrectKeyword {
                            expected: vec![
                                MEMBER.into(),
                                ENSEMBLEACCURACY.into(),
                                ID.into(),
                                ELLIPSOID.into(),
                                SPHEROID.into(),
                            ]
                            .into(),
                            found: node.keyword.clone(),
                        });
                    }
                },
                _ => return Err(WktParseError::ExpectedNode),
            }
        }

        let datum_ensemble_accuracy = match datum_ensemble_accuracy_op {
            Some(x) => x,
            None => return Err(WktParseError::TooFewKeyword(ENSEMBLEACCURACY.into())),
        };

        let ellipsoid = match ellipsoid_op {
            Some(x) => x,
            None => return Err(WktParseError::TooFewKeyword(ELLIPSOID.into())),
        };

        return Ok(GeodeticEnsemble {
            datum_ensemble_name,
            datum_ensemble_member,
            ellipsoid,
            datum_ensemble_accuracy,
            prime_meridian: pm,
        });
    }
}
