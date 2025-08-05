use crate::{
    ast::{WktArg, WktNode},
    coordinate_system::CoordinateSystem,
    datum::GeodeticReferenceFrameDatum,
    dynamic_crs::DynamicCrs,
    ensemble::GeodeticDatumEnsemble,
    error::WktParseError,
    geodetic_crs::GeodeticData,
    keywords::{DATUM, ENSEMBLE, GEODETICDATUM, GEOGCRS, GEOGRAPHICCRS, PRIMEM, TRF},
    prime_meridian::PrimeMeridian,
    scope_extent_identifier_remark::ScopeExtentIdentifierRemark,
};

#[derive(Debug, PartialEq)]
pub struct StaticGeographicCrs {
    pub crs_name: String,
    pub frame: GeodeticData,
    pub coordinate_system: CoordinateSystem,
    pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl TryFrom<&WktNode> for StaticGeographicCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == GEOGCRS || value.keyword == GEOGRAPHICCRS) {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![GEOGCRS.into(), GEOGRAPHICCRS.into()].into(),
                found: value.keyword.clone(),
            });
        }

        if value.args.len() < 4 {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["4+".into()].into(),
                found: value.args.len(),
            });
        }

        let crs_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let prime_meridian = match &value.args[2] {
            WktArg::Node(node) => match PrimeMeridian::try_from(node) {
                Ok(x) => Some(x),
                Err(_) => None,
            },
            _ => None,
        };

        let i = match prime_meridian {
            Some(_) => 3,
            None => 2,
        };

        let geodetic_reference_frame = match &value.args[1] {
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

        // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAARRRRRRRRRRGGGGGGGGGGGGGGHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH
        // PLEASE JUST USE HIERARCHY IT IS PART OF YOUR LANGUAGE
        let coordinate_system = CoordinateSystem::try_from(&value.args[i..value.args.len()])?;

        let args_needed = coordinate_system.needed_args();

        // TODO: Can this fail?
        let scope_extent_identifier_remark = ScopeExtentIdentifierRemark::try_from(
            &value.args[(i + args_needed)..value.args.len()],
        )?;

        return Ok(StaticGeographicCrs {
            crs_name,
            frame: geodetic_reference_frame,
            coordinate_system,
            scope_extent_identifier_remark,
        });
    }
}
