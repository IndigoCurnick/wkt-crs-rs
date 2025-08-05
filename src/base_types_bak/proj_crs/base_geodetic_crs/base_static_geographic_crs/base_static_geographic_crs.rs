use crate::{
    ast::{WktArg, WktNode},
    datum::GeodeticReferenceFrameDatum,
    ensemble::GeodeticDatumEnsemble,
    error::WktParseError,
    geodetic_crs::GeodeticData,
    keywords::{ANGLEUNIT, BASEGEOGCRS, DATUM, ENSEMBLE, GEODETICDATUM, ID, PRIMEM, TRF, UNIT},
    prime_meridian::PrimeMeridian,
    scope_extent_identifier_remark::Id,
    units::AngleUnit,
};

#[derive(Debug, PartialEq)]
pub struct BaseStaticGeographicCrs {
    pub base_crs_name: String,
    pub geodetic_data: GeodeticData,
    pub ellipsoidal_cs_unit: Option<AngleUnit>,
    pub identifier: Option<Id>,
}

impl TryFrom<&WktNode> for BaseStaticGeographicCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != BASEGEOGCRS {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![BASEGEOGCRS.into()].into(),
                found: value.keyword.clone(),
            });
        }

        // TODO: What is the arity?

        let base_crs_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let prime_meridian = match value.args.get(2) {
            Some(arg) => match arg {
                WktArg::Node(node) => match PrimeMeridian::try_from(node) {
                    Ok(x) => Some(x),
                    Err(_) => None,
                },
                _ => None,
            },
            None => None,
        };

        let i = match prime_meridian {
            Some(_) => 3,
            None => 2,
        };

        let geodetic_data = match &value.args[1] {
            WktArg::Node(node) => match node.keyword.as_str() {
                DATUM | TRF | GEODETICDATUM => GeodeticData::GeodeticReferenceFrame(
                    GeodeticReferenceFrameDatum::try_from((node, prime_meridian))?,
                ),
                ENSEMBLE => {
                    let pm_for_real = match prime_meridian {
                        Some(x) => x,
                        None => return Err(WktParseError::TooFewKeyword(PRIMEM.into())),
                    };
                    GeodeticData::GeodeticDatumEnsemble(GeodeticDatumEnsemble::try_from((
                        node,
                        pm_for_real,
                    ))?)
                }
                _ => {
                    return Err(WktParseError::IncorrectKeyword {
                        expected: vec![
                            DATUM.into(),
                            TRF.into(),
                            GEODETICDATUM.into(),
                            ENSEMBLE.into(),
                        ]
                        .into(),
                        found: node.keyword.clone(),
                    });
                }
            },
            _ => return Err(WktParseError::ExpectedNode),
        };

        let mut ellipsoidal_cs_unit = None;
        let mut identifier = None;

        for j in i..value.args.len() {
            let this_arg = &value.args[j];

            match this_arg {
                WktArg::Node(node) => match node.keyword.as_str() {
                    ANGLEUNIT | UNIT => {
                        if ellipsoidal_cs_unit.is_some() {
                            return Err(WktParseError::TooManyKeyword(ANGLEUNIT.to_string()));
                        }

                        ellipsoidal_cs_unit = Some(AngleUnit::try_from(node)?)
                    }
                    ID => {
                        if identifier.is_some() {
                            return Err(WktParseError::TooManyKeyword(ID.to_string()));
                        }

                        identifier = Some(Id::try_from(node)?)
                    }
                    _ => {
                        return Err(WktParseError::IncorrectKeyword {
                            expected: vec![ANGLEUNIT.into(), UNIT.into(), ID.into()].into(),
                            found: node.keyword.clone(),
                        });
                    }
                },
                _ => return Err(WktParseError::ExpectedNode),
            }
        }

        return Ok(BaseStaticGeographicCrs {
            base_crs_name,
            geodetic_data,
            ellipsoidal_cs_unit,
            identifier,
        });
    }
}
