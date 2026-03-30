use crate::{
	ast::parse_wkt,
	base_types::{
		Axis, CoordinateSystem, Id, LengthUnit, Order, SpatialCoordinateSystem,
	},
	compound_types::{SpatialUnit, Unit},
	data_types::NumText,
	enumerations::{AxisDirection, Dimension, SpatialCsType},
	types::WktBaseType,
};

const EXAMPLE1: &str = r#"CS[Cartesian,3],
                AXIS["(X)",geocentricX,ORDER[1]],
                AXIS["(Y)",geocentricY,ORDER[2]],
                AXIS["(Z)",geocentricZ,ORDER[3]],
                LENGTHUNIT["metre",1.0]
"#;

const EXAMPLE2: &str = r#"
CS[
	Cartesian,
	2,
	ID["EPSG",4497]
],
AXIS[
	"Easting (X)",
	east
],
AXIS[
	"Northing (Y)",
	north
],
LENGTHUNIT[
	"US survey foot",
	0.304800609601219,
	ID["EPSG",9003]
]
"#;

#[test]
fn test_spatial_coordinate_system() {
	test_example_1();
	test_example_2();
}

fn test_example_1() {
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
	let ast = parse_wkt(EXAMPLE1).unwrap();

	assert_eq!(ast.len(), 5);

	let acc = CoordinateSystem::from_nodes(&ast).unwrap();

	assert_eq!(correct, acc.result);
}

fn test_example_2() {
	let correct = SpatialCoordinateSystem {
		spatial_cs_type: SpatialCsType::Cartesian,
		dimension: Dimension::Two,
		identifier: Some(Id {
			authority_name: "EPSG".to_string(),
			authority_unique_identifier: NumText::Int(4497),
			version: None,
			authority_citation: None,
			id_uri: None,
		}),
		spatial_axis: vec![
			Axis {
				axis_name_abbreviation: "Easting (X)".to_string(),
				axis_direction: AxisDirection::East,
				axis_order: None,
				unit: None,
				identifier: None,
			},
			Axis {
				axis_name_abbreviation: "Northing (Y)".to_string(),
				axis_direction: AxisDirection::North(None),
				axis_order: None,
				unit: None,
				identifier: None,
			},
		],
		cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
			unit_name: "US survey foot".to_string(),
			conversion_factor: 0.304800609601219,
			identifier: Some(Id {
				authority_name: "EPSG".to_string(),
				authority_unique_identifier: NumText::Int(9003),
				version: None,
				authority_citation: None,
				id_uri: None,
			}),
		}))),
	};

	let ast = parse_wkt(EXAMPLE2).unwrap();

	assert_eq!(ast.len(), 4);

	let acc = SpatialCoordinateSystem::from_nodes(&ast).unwrap();

	assert_eq!(correct, acc.result);
}
