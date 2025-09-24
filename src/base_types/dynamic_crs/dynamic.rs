use crate::{
    arity::match_arity,
    ast::{Parse, WktNode},
    base_types::{DeformationModelId, FrameEpoch},
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct DynamicCrs {
    pub frame_reference_epoch: FrameEpoch,
    pub deformation_model_id: Option<DeformationModelId>,
}

impl WktBaseType for DynamicCrs {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        // Take 1

        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::Dynamic])?;
        match_arity(node.args.len(), 1, 2)?;

        let frame_reference_epoch = node.args[0].parse()?;

        let deformation_model_id = match node.args.get(1) {
            Some(x) => Some(x.parse()?),
            None => None,
        };

        let crs = DynamicCrs {
            frame_reference_epoch,
            deformation_model_id,
        };

        let res = WktBaseTypeResult {
            consumed: 1,
            result: crs,
        };

        Ok(res)
    }
}
