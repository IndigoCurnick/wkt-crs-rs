use crate::{
    ast::WktNode,
    base_types::{GeodeticDatumEnsemble, GeodeticReferenceFrame},
    error::WktParseError,
    types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub enum GeodeticData {
    GeodeticReferenceFrame(GeodeticReferenceFrame),
    GeodeticDatumEnsemble(GeodeticDatumEnsemble),
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
