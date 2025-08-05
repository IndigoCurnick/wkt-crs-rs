use crate::{
    ast::{WktArg, WktNode},
    coordinate_system::CoordinateSystem,
    ensemble::VerticalEnsemble,
    error::WktParseError,
    keywords::{ENSEMBLE, VDATUM, VERTCRS, VERTICALCRS, VERTICALDATUM, VRF},
    scope_extent_identifier_remark::ScopeExtentIdentifierRemark,
    vertical_crs::{
        geoid_model_id::GeoidModelId, vertical_reference_frame::VerticalReferenceFrame,
    },
};

#[derive(Debug, PartialEq)]
pub enum VerticalFrameDatum {
    VerticalReferenceFrame(VerticalReferenceFrame),
    VerticalDatumEnsemble(VerticalEnsemble),
}

impl TryFrom<&WktNode> for VerticalFrameDatum {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        match value.keyword.as_str() {
            ENSEMBLE => {
                return match VerticalEnsemble::try_from(value) {
                    Ok(x) => Ok(VerticalFrameDatum::VerticalDatumEnsemble(x)),
                    Err(y) => Err(y),
                };
            }
            VDATUM | VRF | VERTICALDATUM => {
                return match VerticalReferenceFrame::try_from(value) {
                    Ok(x) => Ok(VerticalFrameDatum::VerticalReferenceFrame(x)),
                    Err(y) => Err(y),
                };
            }
            _ => {
                return Err(WktParseError::IncorrectKeyword {
                    expected: vec![
                        ENSEMBLE.into(),
                        VDATUM.into(),
                        VRF.into(),
                        VERTICALDATUM.into(),
                    ]
                    .into(),
                    found: value.keyword.clone(),
                });
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct StaticVerticalCrs {
    pub crs_name: String,
    pub vertical_frame_datum: VerticalFrameDatum,
    pub coordinate_system: CoordinateSystem,
    pub geoid_model_id: Option<GeoidModelId>,
    pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl TryFrom<&WktNode> for StaticVerticalCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == VERTCRS || value.keyword == VERTICALCRS) {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![VERTCRS.into(), VERTICALCRS.into()].into(),
                found: value.keyword.clone(),
            });
        }

        // TODO: Arity?

        let crs_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let vertical_frame_datum = match &value.args[1] {
            WktArg::Node(node) => VerticalFrameDatum::try_from(node)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let coordinate_system = CoordinateSystem::try_from(&value.args[2..value.args.len()])?;

        let mut i = 2 + coordinate_system.needed_args();

        let geoid_model_id = match value.args.get(i) {
            Some(arg) => {
                match arg {
                    WktArg::Node(node) => match GeoidModelId::try_from(node) {
                        Ok(x) => {
                            i += 1;
                            Some(x)
                        }
                        Err(_) => None, // Probably just something else
                    },
                    _ => None, //Probably just something else
                }
            }
            None => None,
        };

        let scope_extent_identifier_remark =
            ScopeExtentIdentifierRemark::try_from(&value.args[i..value.args.len()])?;

        return Ok(StaticVerticalCrs {
            crs_name,
            vertical_frame_datum,
            coordinate_system,
            geoid_model_id,
            scope_extent_identifier_remark,
        });
    }
}
