use crate::{
	ast::parse_wkt,
	base_types::{DeformationModelId, FrameEpoch},
	types::WktBaseType,
};

use super::dynamic::DynamicCrs;

const EXAMPLE1: &str = r#"DYNAMIC[FRAMEEPOCH[2010.0]]"#;
const EXAMPLE2: &str =
	r#"DYNAMIC[FRAMEEPOCH[2010.0],MODEL["NAD83 (CSRS) v6 velocity grid"]]"#;

#[test]
fn test_dynamic_crs() {
	// Example 1

	let correct = DynamicCrs {
		frame_reference_epoch: FrameEpoch(2010.0),
		deformation_model_id: None,
	};

	let ast = parse_wkt(EXAMPLE1);

	assert_eq!(ast.len(), 1);

	let dynamic = DynamicCrs::from_nodes(&ast).unwrap();

	assert_eq!(correct, dynamic.result);

	// Example 2

	let correct = DynamicCrs {
		frame_reference_epoch: FrameEpoch(2010.0),
		deformation_model_id: Some(DeformationModelId(
			"NAD83 (CSRS) v6 velocity grid".into(),
		)),
	};

	let ast = parse_wkt(EXAMPLE2);

	assert_eq!(ast.len(), 1);

	let dynamic = DynamicCrs::from_nodes(&ast).unwrap();

	assert_eq!(correct, dynamic.result);
}
