use crate::{
    ast::{WktArg, WktNode},
    datum::datum_anchor::DatumAnchor,
    ellipsoid::Ellipsoid,
    error::WktParseError,
    keywords::{ANCHOR, DATUM, GEODETICDATUM, ID, TRF},
    prime_meridian::PrimeMeridian,
    scope_extent_identifier_remark::Id,
};

#[derive(Debug, PartialEq)]
pub struct GeodeticReferenceFrameDatum {
    pub datum_name: String,
    pub ellipsoid: Ellipsoid,
    pub anchor: Option<DatumAnchor>,
    pub identifier: Option<Id>, // TODO: technically multiple allowed
    pub prime_meridian: Option<PrimeMeridian>,
}

impl TryFrom<(&WktNode, Option<PrimeMeridian>)> for GeodeticReferenceFrameDatum {
    type Error = WktParseError;

    fn try_from(value: (&WktNode, Option<PrimeMeridian>)) -> Result<Self, Self::Error> {
        let (value, pm) = value;

        if !(value.keyword == DATUM || value.keyword == TRF || value.keyword == GEODETICDATUM) {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![DATUM.into(), TRF.into(), GEODETICDATUM.into()].into(),
                found: value.keyword.clone(),
            });
        }

        if !(value.args.len() == 2 || value.args.len() == 3 || value.args.len() == 4) {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["2".into(), "3".into(), "4".into()].into(),
                found: value.args.len(),
            });
        }

        let datum_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let ellipsoid = match &value.args[1] {
            WktArg::Node(node) => Ellipsoid::try_from(node)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let mut anchor = None;
        let mut identifier = None;

        for i in 2..value.args.len() {
            let arg = &value.args[i];

            match arg {
                WktArg::Node(node) => match node.keyword.as_str() {
                    ANCHOR => {
                        if anchor.is_some() {
                            return Err(WktParseError::TooManyKeyword(ANCHOR.to_string()));
                        }

                        anchor = Some(DatumAnchor::try_from(node)?);
                    }
                    ID => {
                        if identifier.is_some() {
                            return Err(WktParseError::TooManyKeyword(ID.to_string()));
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

        return Ok(GeodeticReferenceFrameDatum {
            datum_name,
            ellipsoid,
            anchor,
            identifier,
            prime_meridian: pm,
        });
    }
}
