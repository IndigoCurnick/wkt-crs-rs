use crate::{
	ast::parse_wkt,
	base_types::{
		Axis, CoordinateSystem, DeformationModelId, DynamicCrs, FrameEpoch,
		GeoidModelId, Id, LengthUnit, SpatialCoordinateSystem,
		VerticalReferenceFrame,
		vertical_crs::{DynamicVerticalCrs, StaticVerticalCrs, VerticalCrs},
	},
	compound_types::{
		ScopeExtentIdentifierRemark, SpatialUnit, Unit, VerticalFrameDatum,
	},
	data_types::NumText,
	enumerations::{AxisDirection, Dimension, SpatialCsType},
	types::WktBaseType,
};

const EXAMPLE1: &str = r#"VERTCRS["NAVD88",
    VDATUM["North American Vertical Datum 1988"],
    CS[vertical,1],
        AXIS["gravity-related height (H)",up],
        LENGTHUNIT["metre",1.0]
]
"#;

const EXAMPLE2: &str = r#"VERTCRS["CGVD2013",
    VRF["Canadian Geodetic Vertical Datum of 2013"],
    CS[vertical,1],
        AXIS["gravity-related height (H)",up],
        LENGTHUNIT["metre",1.0],
    GEOIDMODEL["CGG2013",ID["EPSG",6648]]
]
"#;

const EXAMPLE3: &str = r#"VERTCRS["RH2000",
    DYNAMIC[FRAMEEPOCH[2000.0],MODEL["NKG2016LU"]],
    VDATUM["Rikets Hojdsystem 2000"],
    CS[vertical,1],
        AXIS["gravity-related height (H)",up],
        LENGTHUNIT["metre",1.0]
]
"#;

#[test]
fn test_vertical_crs() {
	test_example_1();
	test_example_2();
	test_example_3();
}

fn test_example_1() {
	let correct = VerticalCrs::StaticVerticalCrs(StaticVerticalCrs {
		crs_name: "NAVD88".into(),
		vertical_frame_datum: VerticalFrameDatum::VerticalReferenceFrame(
			VerticalReferenceFrame {
				datum_name: "North American Vertical Datum 1988".into(),
				datum_anchor: None,
				identifier: None,
			},
		),
		coordinate_system: CoordinateSystem::SpatialCS(
			SpatialCoordinateSystem {
				spatial_cs_type: SpatialCsType::Vertical,
				dimension: Dimension::One,
				identifier: None,
				spatial_axis: vec![Axis {
					axis_name_abbreviation: "gravity-related height (H)"
						.to_string(),
					axis_direction: AxisDirection::Up,
					axis_order: None,
					unit: None,
					identifier: None,
				}],
				cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(
					LengthUnit {
						unit_name: "metre".into(),
						conversion_factor: 1.0,
						identifier: None,
					},
				))),
			},
		),
		geoid_model_id: None,
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: None,
			identifier: None,
			remark: None,
		},
	});

	let ast = parse_wkt(EXAMPLE1);

	assert_eq!(ast.len(), 1);

	let vert = VerticalCrs::from_nodes(&ast).unwrap();

	assert_eq!(vert.result, correct);
}

fn test_example_2() {
	let correct = VerticalCrs::StaticVerticalCrs(StaticVerticalCrs {
		crs_name: "CGVD2013".into(),
		vertical_frame_datum: VerticalFrameDatum::VerticalReferenceFrame(
			VerticalReferenceFrame {
				datum_name: "Canadian Geodetic Vertical Datum of 2013".into(),
				datum_anchor: None,
				identifier: None,
			},
		),
		coordinate_system: CoordinateSystem::SpatialCS(
			SpatialCoordinateSystem {
				spatial_cs_type: SpatialCsType::Vertical,
				dimension: Dimension::One,
				identifier: None,
				spatial_axis: vec![Axis {
					axis_name_abbreviation: "gravity-related height (H)"
						.to_string(),
					axis_direction: AxisDirection::Up,
					axis_order: None,
					unit: None,
					identifier: None,
				}],
				cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(
					LengthUnit {
						unit_name: "metre".into(),
						conversion_factor: 1.0,
						identifier: None,
					},
				))),
			},
		),
		geoid_model_id: Some(GeoidModelId {
			geoid_model_name: "CGG2013".to_string(),
			identifier: Some(Id {
				authority_name: "EPSG".into(),
				authority_unique_identifier: NumText::Int(6648),
				version: None,
				authority_citation: None,
				id_uri: None,
			}),
		}),
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: None,
			identifier: None,
			remark: None,
		},
	});

	let ast = parse_wkt(EXAMPLE2);

	assert_eq!(ast.len(), 1);

	let vert = VerticalCrs::from_nodes(&ast).unwrap();

	assert_eq!(vert.result, correct);
}

fn test_example_3() {
	let correct = VerticalCrs::DynamicVerticalCrs(DynamicVerticalCrs {
		crs_name: "RH2000".into(),
		dynamic_crs: DynamicCrs {
			frame_reference_epoch: FrameEpoch(2000.0),
			deformation_model_id: Some(DeformationModelId("NKG2016LU".into())),
		},
		vertical_reference_frame: VerticalReferenceFrame {
			datum_name: "Rikets Hojdsystem 2000".to_string(),
			datum_anchor: None,
			identifier: None,
		},
		coordinate_system: CoordinateSystem::SpatialCS(
			SpatialCoordinateSystem {
				spatial_cs_type: SpatialCsType::Vertical,
				dimension: Dimension::One,
				identifier: None,
				spatial_axis: vec![Axis {
					axis_name_abbreviation: "gravity-related height (H)"
						.to_string(),
					axis_direction: AxisDirection::Up,
					axis_order: None,
					unit: None,
					identifier: None,
				}],
				cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(
					LengthUnit {
						unit_name: "metre".into(),
						conversion_factor: 1.0,
						identifier: None,
					},
				))),
			},
		),
		geoid_model_id: None,
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: None,
			identifier: None,
			remark: None,
		},
	});

	let ast = parse_wkt(EXAMPLE3);

	assert_eq!(ast.len(), 1);

	let vert = VerticalCrs::from_nodes(&ast).unwrap();

	assert_eq!(vert.result, correct);
}
