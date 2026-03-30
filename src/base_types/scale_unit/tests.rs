use crate::{
	ast::parse_wkt, base_types::scale_unit::ScaleUnit, types::WktBaseType,
};

const SU_EXAMPLE1: &str = r#"SCALEUNIT["parts per million",1E-06]"#;

#[test]
fn test_scale_unit() {
	let correct = ScaleUnit {
		conversion_factor: 1e-6,
		identifier: None,
		unit_name: "parts per million".to_string(),
	};

	let ast = parse_wkt(SU_EXAMPLE1).unwrap();
	assert_eq!(ast.len(), 1);
	let su = ScaleUnit::from_nodes(&ast).unwrap();

	assert_eq!(correct, su.result);
}
