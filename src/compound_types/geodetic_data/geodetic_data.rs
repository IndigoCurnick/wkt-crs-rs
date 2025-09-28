use crate::{
    ast::{WktArg, WktNode},
    base_types::{GeodeticDatumEnsemble, GeodeticReferenceFrame, GeographicBoundingBox},
    error::WktParseError,
    types::{WktBaseType, WktBaseTypeResult, WktInlineResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub enum GeodeticData {
    GeodeticReferenceFrame(GeodeticReferenceFrame),
    GeodeticDatumEnsemble(GeodeticDatumEnsemble),
}

impl WktInlineType for GeodeticData {
    fn from_args<'a, I>(wkt_args: I) -> Result<crate::types::WktInlineResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a crate::ast::WktArg>,
    {
        let iter: Vec<&'a WktArg> = wkt_args.into_iter().collect();

        if let Ok(res) = GeodeticReferenceFrame::from_args(iter.clone()) {
            return Ok(WktInlineResult {
                consumed: res.consumed,
                result: Self::GeodeticReferenceFrame(res.result),
            });
        }

        if let Ok(res) = GeodeticDatumEnsemble::from_args(iter.clone()) {
            return Ok(WktInlineResult {
                consumed: res.consumed,
                result: Self::GeodeticDatumEnsemble(res.result),
            });
        }

        return Err(WktParseError::CouldNotDetermineType);
    }
}

impl WktBaseType for GeodeticData {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let iter: Vec<&'a WktNode> = wkt_nodes.into_iter().collect();

        if let Ok(res) = GeodeticReferenceFrame::from_nodes(iter.clone()) {
            return Ok(WktBaseTypeResult {
                consumed: res.consumed,
                result: Self::GeodeticReferenceFrame(res.result),
            });
        }

        if let Ok(res) = GeodeticDatumEnsemble::from_nodes(iter.clone()) {
            return Ok(WktBaseTypeResult {
                consumed: res.consumed,
                result: Self::GeodeticDatumEnsemble(res.result),
            });
        }

        return Err(WktParseError::CouldNotDetermineType);
    }
}
