use crate::{
	WktBaseType,
	ast::parse_wkt,
	base_types::{
		Axis, CoordinateSystem, GeoidModelId, Id, LengthUnit,
		SpatialCoordinateSystem, StaticVerticalCrs, VerticalReferenceFrame,
	},
	compound_types::{
		ScopeExtentIdentifierRemark, SpatialUnit, Unit, VerticalFrameDatum,
	},
	enumerations::{AxisDirection, Dimension, SpatialCsType},
};

const EXAMPLE: &str = r#"
VERTCRS[
	"JSLD69 height",
	VDATUM[
		"Japanese Standard Levelling Datum 1969",
		ID["EPSG",5122]
	],
	CS[
		vertical,
		1,
		ID["EPSG",6499]
	],
	AXIS[
		"Gravity-related height (H)",
		up
	],
	LENGTHUNIT[
		"metre",
		1,
		ID["EPSG",9001]
	],
	ID["EPSG",5723]
]
"#;

const EXAMPLE2: &str = r#"
VERTCRS[
	"ISH2004 height",
	VDATUM[
		"Landshaedarkerfi Islands 2004",
		ID["EPSG",1190]
	],
	CS[
		vertical,
		1,
		ID["EPSG",6499]
	],
	AXIS[
		"Gravity-related height (H)",
		up
	],
	LENGTHUNIT[
		"metre",
		1,
		ID["EPSG",9001]
	],
	GEOIDMODEL[
		"ISN93 to ISH2004 height (1)",
		ID["EPSG",9954]
	],
	GEOIDMODEL[
		"ISN2004 to ISH2004 height (1)",
		ID["EPSG",9956]
	],
	GEOIDMODEL[
		"ISN2016 to ISH2004 height (1)",
		ID["EPSG",9958]
	],
	ID["EPSG",8089]
]
"#;

#[test]
fn test_static_vertical_crs_example_1() {
	let correct = StaticVerticalCrs {
		crs_name: "JSLD69 height".to_string(),
		vertical_frame_datum: VerticalFrameDatum::VerticalReferenceFrame(
			VerticalReferenceFrame {
				datum_name: "Japanese Standard Levelling Datum 1969"
					.to_string(),
				datum_anchor: None,
				identifier: Some(Id::new_epsg(5122)),
			},
		),
		coordinate_system: CoordinateSystem::SpatialCS(
			SpatialCoordinateSystem {
				spatial_cs_type: SpatialCsType::Vertical,
				dimension: Dimension::One,
				identifier: Some(Id::new_epsg(6499)),
				spatial_axis: vec![Axis {
					axis_name_abbreviation: "Gravity-related height (H)"
						.to_string(),
					axis_direction: AxisDirection::Up,
					axis_order: None,
					unit: None,
					identifier: None,
				}],
				cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(
					LengthUnit::metre(),
				))),
			},
		),
		geoid_model_id: None,
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: None,
			identifier: Some(vec![Id::new_epsg(5723)]),
			remark: None,
		},
	};

	let ast = parse_wkt(EXAMPLE).unwrap();

	assert_eq!(ast.len(), 1);

	let acc = StaticVerticalCrs::from_nodes(&ast).unwrap();

	assert_eq!(correct, acc.result);
}

#[test]
fn test_static_vertical_crs_example_2() {
	let correct = StaticVerticalCrs {
		crs_name: "ISH2004 height".to_string(),
		vertical_frame_datum: VerticalFrameDatum::VerticalReferenceFrame(
			VerticalReferenceFrame {
				datum_name: "Landshaedarkerfi Islands 2004".to_string(),
				datum_anchor: None,
				identifier: Some(Id::new_epsg(1190)),
			},
		),
		coordinate_system: CoordinateSystem::SpatialCS(
			SpatialCoordinateSystem {
				spatial_cs_type: SpatialCsType::Vertical,
				dimension: Dimension::One,
				identifier: Some(Id::new_epsg(6499)),
				spatial_axis: vec![Axis {
					axis_name_abbreviation: "Gravity-related height (H)"
						.to_string(),
					axis_direction: AxisDirection::Up,
					axis_order: None,
					unit: None,
					identifier: None,
				}],
				cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(
					LengthUnit::metre(),
				))),
			},
		),
		geoid_model_id: Some(vec![
			GeoidModelId {
				geoid_model_name: "ISN93 to ISH2004 height (1)".to_string(),
				identifier: Some(Id::new_epsg(9954)),
			},
			GeoidModelId {
				geoid_model_name: "ISN2004 to ISH2004 height (1)".to_string(),
				identifier: Some(Id::new_epsg(9956)),
			},
			GeoidModelId {
				geoid_model_name: "ISN2016 to ISH2004 height (1)".to_string(),
				identifier: Some(Id::new_epsg(9958)),
			},
		]),
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: None,
			identifier: Some(vec![Id::new_epsg(8089)]),
			remark: None,
		},
	};

	let ast = parse_wkt(EXAMPLE2).unwrap();

	assert_eq!(ast.len(), 1);

	let acc = StaticVerticalCrs::from_nodes(&ast).unwrap();

	assert_eq!(correct, acc.result);
}
