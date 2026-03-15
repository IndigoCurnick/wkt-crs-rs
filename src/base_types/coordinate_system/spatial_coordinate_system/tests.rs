use crate::{
	ast::parse_wkt,
	base_types::{
		Axis, CoordinateSystem, LengthUnit, Order, SpatialCoordinateSystem,
	},
	compound_types::{SpatialUnit, Unit},
	enumerations::{AxisDirection, Dimension, SpatialCsType},
	types::WktBaseType,
};

const EXAMPLE1: &str = r#"CS[Cartesian,3],
                AXIS["(X)",geocentricX,ORDER[1]],
                AXIS["(Y)",geocentricY,ORDER[2]],
                AXIS["(Z)",geocentricZ,ORDER[3]],
                LENGTHUNIT["metre",1.0]
"#;

#[test]
fn test_spatial_coordinate_system() {
	let correct = CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
		spatial_cs_type: SpatialCsType::Cartesian,
		dimension: Dimension::Three,
		identifier: None,
		spatial_axis: vec![
			Axis {
				axis_name_abbreviation: "(X)".into(),
				axis_direction: AxisDirection::GeocentricX,
				axis_order: Some(Order(1)),
				unit: None,
				identifier: None,
			},
			Axis {
				axis_name_abbreviation: "(Y)".into(),
				axis_direction: AxisDirection::GeocentricY,
				axis_order: Some(Order(2)),
				unit: None,
				identifier: None,
			},
			Axis {
				axis_name_abbreviation: "(Z)".into(),
				axis_direction: AxisDirection::GeocentricZ,
				axis_order: Some(Order(3)),
				unit: None,
				identifier: None,
			},
		],
		cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
			unit_name: "metre".into(),
			conversion_factor: 1.0,
			identifier: None,
		}))),
	});
	let ast = parse_wkt(EXAMPLE1);

	assert_eq!(ast.len(), 5);

	let acc = CoordinateSystem::from_nodes(&ast).unwrap();

	assert_eq!(correct, acc.result);
}
