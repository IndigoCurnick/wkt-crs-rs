use crate::{
    ast::{WktArg, WktNode},
    dynamic_crs::{
        deformation_model::DeformationModel, frame_reference_epoch::FrameReferenceEpoch,
    },
    error::WktParseError,
    keywords::DYNAMIC,
};

#[derive(Debug, PartialEq)]
pub struct DynamicCrs {
    pub frame_reference_epoch: FrameReferenceEpoch,
    pub deformation_model_id: Option<DeformationModel>,
}

impl TryFrom<&WktNode> for DynamicCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != DYNAMIC {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![DYNAMIC.into()].into(),
                found: value.keyword.clone(),
            });
        }

        if !(value.args.len() == 1 || value.args.len() == 2) {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["1".into(), "2".into()].into(),
                found: value.args.len(),
            });
        }

        let frame_reference_epoch = match &value.args[0] {
            WktArg::Node(node) => FrameReferenceEpoch::try_from(node)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let deformation_model_id = match value.args.get(1) {
            Some(x) => match x {
                WktArg::Node(node) => Some(DeformationModel::try_from(node)?),
                _ => return Err(WktParseError::ExpectedNode),
            },
            None => None,
        };

        return Ok(DynamicCrs {
            frame_reference_epoch,
            deformation_model_id,
        });
    }
}
