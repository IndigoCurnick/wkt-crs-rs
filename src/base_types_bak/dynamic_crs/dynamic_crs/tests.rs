use crate::{
    ast::parse_wkt,
    dynamic_crs::{
        deformation_model::DeformationModel, frame_reference_epoch::FrameReferenceEpoch,
    },
};

use super::dynamic::DynamicCrs;

const EXAMPLE1: &str = r#"DYNAMIC[FRAMEEPOCH[2010.0]]"#;
const EXAMPLE2: &str = r#"DYNAMIC[FRAMEEPOCH[2010.0],MODEL["NAD83 (CSRS) v6 velocity grid"]]"#;

#[test]
fn test_dynamic_crs() {
    // Example 1

    let correct = DynamicCrs {
        frame_reference_epoch: FrameReferenceEpoch(2010.0),
        deformation_model_id: None,
    };

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let dynamic = DynamicCrs::try_from(&ast[0]).unwrap();

    assert_eq!(correct, dynamic);

    // Example 2

    let correct = DynamicCrs {
        frame_reference_epoch: FrameReferenceEpoch(2010.0),
        deformation_model_id: Some(DeformationModel("NAD83 (CSRS) v6 velocity grid".into())),
    };

    let ast = parse_wkt(EXAMPLE2);

    assert_eq!(ast.len(), 1);

    let dynamic = DynamicCrs::try_from(&ast[0]).unwrap();

    assert_eq!(correct, dynamic);
}
