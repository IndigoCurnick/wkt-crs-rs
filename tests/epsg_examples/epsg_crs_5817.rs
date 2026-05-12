use wkt_crs_rs::{
	WktCrsTypes,
	base_types::{
		Axis, CoordinateSystem, EngineeringCrs, EngineeringDatum, Id,
		LengthUnit, SpatialCoordinateSystem,
	},
	compound_types::{ScopeExtentIdentifierRemark, SpatialUnit, Unit},
	enumerations::{AxisDirection, Dimension, SpatialCsType},
	parse_wkt_crs,
};

const EXAMPLE: &str = r#"
ENGCRS[
	"Tombak LNG plant",
	EDATUM[
		"Tombak LNG plant",
		ID["EPSG",9314]
	],
	CS[
		Cartesian,
		2,
		ID["EPSG",6510]
	],
	AXIS[
		"Plant East (x)",
		northEast
	],
	AXIS[
		"Plant North (y)",
		northWest
	],
	LENGTHUNIT[
		"metre",
		1,
		ID["EPSG",9001]
	],
	ID["EPSG",5817]
]
"#;

#[test]
fn test_epsg_crs_5817() {
	let correct = EngineeringCrs {
		crs_name: "Tombak LNG plant".to_string(),
		engineering_datum: EngineeringDatum {
			datum_name: "Tombak LNG plant".to_string(),
			datum_anchor: None,
			identifier: Some(Id::new_epsg(9314)),
		},
		coordinate_system: CoordinateSystem::SpatialCS(
			SpatialCoordinateSystem {
				spatial_cs_type: SpatialCsType::Cartesian,
				dimension: Dimension::Two,
				identifier: Some(Id::new_epsg(6510)),
				spatial_axis: vec![
					Axis {
						axis_name_abbreviation: "Plant East (x)".to_string(),
						axis_direction: AxisDirection::NorthEast,
						axis_order: None,
						unit: None,
						identifier: None,
					},
					Axis {
						axis_name_abbreviation: "Plant North (y)".to_string(),
						axis_direction: AxisDirection::NorthWest,
						axis_order: None,
						unit: None,
						identifier: None,
					},
				],
				cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(
					LengthUnit::metre(),
				))),
			},
		),
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: None,
			identifier: Some(vec![Id::new_epsg(5817)]),
			remark: None,
		},
	};

	let correct = vec![WktCrsTypes::EngineeringCrs(correct)];

	let ast = parse_wkt_crs(EXAMPLE).unwrap();

	assert_eq!(correct, ast);
}
