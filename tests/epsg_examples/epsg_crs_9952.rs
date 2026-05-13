use std::thread::Scope;

use wkt_crs_rs::{
	WktCrsTypes,
	base_types::{
		AngleUnit, Axis, BaseDynamicGeographicCrs, BaseGeodeticCrs,
		BaseStaticGeographicCrs, CompoundCrs, CoordinateSystem, Ellipsoid,
		GeodeticReferenceFrame, GeoidModelId, Id, LengthUnit, MapProjection,
		Method, Parameter, ProjectedCrs, SpatialCoordinateSystem,
		StaticVerticalCrs, VerticalCrs, VerticalReferenceFrame,
	},
	compound_types::{
		GeodeticData, ScopeExtentIdentifierRemark, SingleCrs, SpatialUnit,
		Unit, VerticalFrameDatum,
	},
	enumerations::{AxisDirection, Dimension, SpatialCsType},
	parse_wkt_crs,
};

const EXAMPLE: &str = r#"
COMPOUNDCRS[
	"ISN2004 / Lambert 2004 + ISH2004 height",
	PROJCRS[
		"ISN2004 / Lambert 2004",
		BASEGEOGCRS[
			"ISN2004",
			DATUM[
				"Islands Net 2004",
				ELLIPSOID[
					"GRS 1980",
					6378137,
					298.257222101,
					LENGTHUNIT[
						"metre",
						1,
						ID["EPSG",9001]
					],
					ID["EPSG",7019]
				],
				ID["EPSG",1060]
			],
			ID["EPSG",5324]
		],
		CONVERSION[
			"Iceland Lambert 2004",
			METHOD[
				"Lambert Conic Conformal (2SP)",
				ID["EPSG",9802]
			],
			PARAMETER[
				"Latitude of false origin",
				65,
				ANGLEUNIT[
					"degree",
					0.0174532925199433,
					ID["EPSG",9102]
				],
				ID["EPSG",8821]
			],
			PARAMETER[
				"Longitude of false origin",
				-19,
				ANGLEUNIT[
					"degree",
					0.0174532925199433,
					ID["EPSG",9102]
				],
				ID["EPSG",8822]
			],
			PARAMETER[
				"Latitude of 1st standard parallel",
				64.2500000000003,
				ANGLEUNIT[
					"degree",
					0.0174532925199433,
					ID["EPSG",9102]
				],
				ID["EPSG",8823]
			],
			PARAMETER[
				"Latitude of 2nd standard parallel",
				65.7500000000003,
				ANGLEUNIT[
					"degree",
					0.0174532925199433,
					ID["EPSG",9102]
				],
				ID["EPSG",8824]
			],
			PARAMETER[
				"Easting at false origin",
				1700000,
				LENGTHUNIT[
					"metre",
					1,
					ID["EPSG",9001]
				],
				ID["EPSG",8826]
			],
			PARAMETER[
				"Northing at false origin",
				300000,
				LENGTHUNIT[
					"metre",
					1,
					ID["EPSG",9001]
				],
				ID["EPSG",8827]
			],
			ID["EPSG",5326]
		],
		CS[
			Cartesian,
			2,
			ID["EPSG",4499]
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
			"metre",
			1,
			ID["EPSG",9001]
		],
		ID["EPSG",5325]
	],
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
	],
	ID["EPSG",9952]
]
"#;

#[test]
fn test_epsg_crs_9952() {
	let projected_crs = ProjectedCrs {
		crs_name: "ISN2004 / Lambert 2004".to_string(),
		base_geodetic_crs: BaseGeodeticCrs::BaseStaticGeographicCrs(
			BaseStaticGeographicCrs {
				base_crs_name: "ISN2004".to_string(),
				geodetic_data: GeodeticData::GeodeticReferenceFrame(
					GeodeticReferenceFrame {
						datum_name: "Islands Net 2004".to_string(),
						ellipsoid: Ellipsoid {
							ellipsoid_name: "GRS 1980".to_string(),
							semi_major_axis: 6378137.0,
							inverse_flattening: 298.257222101,
							length_unit: Some(LengthUnit::metre()),
							identifier: Some(Id::new_epsg(7019)),
						},
						anchor: None,
						anchor_epoch: None,
						identifier: Some(Id::new_epsg(1060)),
						prime_meridian: None,
					},
				),
				ellipsoidal_cs_unit: None,
				identifier: Some(Id::new_epsg(5324)),
			},
		),
		map_projection: MapProjection {
			map_projection_name: "Iceland Lambert 2004".to_string(),
			map_projection_method: Method {
				method_name: "Lambert Conic Conformal (2SP)".to_string(),
				identifier: Some(Id::new_epsg(9802)),
			},
			map_projection_parameters: Some(vec![
				Parameter {
					parameter_name: "Latitude of false origin".to_string(),
					parameter_value: 65.0,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::AngleUnit(AngleUnit::degree()),
					)),
					identifier: Some(Id::new_epsg(8821)),
				},
				Parameter {
					parameter_name: "Longitude of false origin".to_string(),
					parameter_value: -19.0,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::AngleUnit(AngleUnit::degree()),
					)),
					identifier: Some(Id::new_epsg(8822)),
				},
				Parameter {
					parameter_name: "Latitude of 1st standard parallel"
						.to_string(),
					parameter_value: 64.2500000000003,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::AngleUnit(AngleUnit::degree()),
					)),
					identifier: Some(Id::new_epsg(8823)),
				},
				Parameter {
					parameter_name: "Latitude of 2nd standard parallel"
						.to_string(),
					parameter_value: 65.7500000000003,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::AngleUnit(AngleUnit::degree()),
					)),
					identifier: Some(Id::new_epsg(8824)),
				},
				Parameter {
					parameter_name: "Easting at false origin".to_string(),
					parameter_value: 1700000.0,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::LengthUnit(LengthUnit::metre()),
					)),
					identifier: Some(Id::new_epsg(8826)),
				},
				Parameter {
					parameter_name: "Northing at false origin".to_string(),
					parameter_value: 300000.0,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::LengthUnit(LengthUnit::metre()),
					)),
					identifier: Some(Id::new_epsg(8827)),
				},
			]),
			identifier: Some(Id::new_epsg(5326)),
		},
		coordinate_system: CoordinateSystem::SpatialCS(
			SpatialCoordinateSystem {
				spatial_cs_type: SpatialCsType::Cartesian,
				dimension: Dimension::Two,
				identifier: Some(Id::new_epsg(4499)),
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
				cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(
					LengthUnit::metre(),
				))),
			},
		),
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: None,
			identifier: Some(vec![Id::new_epsg(5325)]),
			remark: None,
		},
	};

	let vertical_crs = VerticalCrs::StaticVerticalCrs(StaticVerticalCrs {
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
	});

	let correct = CompoundCrs {
		compound_crs_name: "ISN2004 / Lambert 2004 + ISH2004 height"
			.to_string(),
		crs_one: SingleCrs::ProjectedCrs(projected_crs),
		crs_two: SingleCrs::VerticalCrs(vertical_crs),
		additional_crs: None,
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: None,
			identifier: Some(vec![Id::new_epsg(9952)]),
			remark: None,
		},
	};

	let correct = vec![WktCrsTypes::CompoundCrs(correct)];

	let ast = parse_wkt_crs(EXAMPLE).unwrap();

	assert_eq!(correct, ast);
}
