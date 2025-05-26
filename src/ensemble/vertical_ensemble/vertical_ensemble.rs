use crate::{
    ast::{WktArg, WktNode},
    ensemble::{ensemble_accuracy::EnsembleAccuracy, ensemble_member::EnsembleMember},
    error::WktParseError,
    keywords::{ENSEMBLE, ENSEMBLEACCURACY, ID, MEMBER},
    scope_extent_identifier_remark::Id,
};

#[derive(Debug, PartialEq)]
pub struct VerticalEnsemble {
    pub datum_ensemble_name: String,
    pub datum_ensemble_member: Vec<EnsembleMember>,
    pub datum_ensemble_accuracy: EnsembleAccuracy,
    pub identifier: Option<Id>, // TODO: Allows multiple
}

impl TryFrom<&WktNode> for VerticalEnsemble {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
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
        let mut datum_ensemble_accuracy_op = None;
        let mut identifier = None;

        for i in 1..value.args.len() {
            let this_arg = &value.args[i];

            match this_arg {
                WktArg::Node(node) => match node.keyword.as_str() {
                    MEMBER => {
                        // Before acc and id
                        if datum_ensemble_accuracy_op.is_some() || identifier.is_some() {
                            return Err(WktParseError::IncorrectKeywordOrder);
                        }

                        datum_ensemble_member.push(EnsembleMember::try_from(node)?);
                    }
                    ENSEMBLEACCURACY => {
                        // Before ID / After members
                        if datum_ensemble_member.is_empty() || identifier.is_some() {
                            return Err(WktParseError::IncorrectKeywordOrder);
                        }

                        if datum_ensemble_accuracy_op.is_some() {
                            return Err(WktParseError::TooManyKeyword(ENSEMBLEACCURACY.into()));
                        }

                        datum_ensemble_accuracy_op = Some(EnsembleAccuracy::try_from(node)?);
                    }
                    ID => {
                        // After member and accuracy
                        if datum_ensemble_member.is_empty() || datum_ensemble_accuracy_op.is_none()
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
                            expected: vec![MEMBER.into(), ENSEMBLEACCURACY.into(), ID.into()]
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

        return Ok(VerticalEnsemble {
            datum_ensemble_name,
            datum_ensemble_member,
            datum_ensemble_accuracy,
            identifier,
        });
    }
}
