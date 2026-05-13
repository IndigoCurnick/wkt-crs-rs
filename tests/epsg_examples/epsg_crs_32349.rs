use wkt_crs_rs::{
	WktCrsTypes,
	base_types::{
		AngleUnit, Axis, BaseDynamicGeographicCrs, BaseGeodeticCrs,
		CoordinateSystem, DynamicCrs, Ellipsoid, FrameEpoch,
		GeodeticReferenceFrame, Id, LengthUnit, MapProjection, Method,
		Parameter, ProjectedCrs, ScaleUnit, SpatialCoordinateSystem,
	},
	compound_types::{ScopeExtentIdentifierRemark, SpatialUnit, Unit},
	enumerations::{AxisDirection, Dimension, SpatialCsType},
	parse_wkt_crs,
};

const EXAMPLE: &str = r#"
PROJCRS[
	"WGS 72 / UTM zone 49S",
	BASEGEOGCRS[
		"WGS 72", 
		DYNAMIC[
			FRAMEEPOCH[1972.0]
		],
		DATUM[
			"World Geodetic System 1972",
			ELLIPSOID[
				"WGS 72",
				6378135,
				298.26,
				LENGTHUNIT[
					"metre",
					1,
					ID["EPSG",9001]
				],
				ID["EPSG",7043]
			],
			ID["EPSG",6322]
		],
		ID["EPSG",4322]
	],
	CONVERSION[
		"UTM zone 49S",
		METHOD[
			"Transverse Mercator",
			ID["EPSG",9807]
		],
		PARAMETER[
			"Latitude of natural origin",
			0,
			ANGLEUNIT[
				"degree",
				0.0174532925199433,
				ID["EPSG",9102]
			],
			ID["EPSG",8801]
		],
		PARAMETER[
			"Longitude of natural origin",
			111,
			ANGLEUNIT[
				"degree",
				0.0174532925199433,
				ID["EPSG",9102]
			],
			ID["EPSG",8802]
		],
		PARAMETER[
			"Scale factor at natural origin",
			0.9996,
			SCALEUNIT[
				"unity",
				1,
				ID["EPSG",9201]
			],
			ID["EPSG",8805]
		],
		PARAMETER[
			"False easting",
			500000,
			LENGTHUNIT[
				"metre",
				1,
				ID["EPSG",9001]
			],
			ID["EPSG",8806]
		],
		PARAMETER[
			"False northing",
			10000000,
			LENGTHUNIT[
				"metre",
				1,
				ID["EPSG",9001]
			],
			ID["EPSG",8807]
		],
		ID["EPSG",16149]
	],
	CS[
		Cartesian,
		2,
		ID["EPSG",4400]
	],
	AXIS[
		"Easting (E)",
		east
	],
	AXIS[
		"Northing (N)",
		north
	],
	LENGTHUNIT[
		"metre",
		1,
		ID["EPSG",9001]
	],
	ID["EPSG",32349]
]
"#;

#[test]
fn test_epsg_crs_32349() {
	let proj = ProjectedCrs {
		crs_name: "WGS 72 / UTM zone 49S".to_string(),
		base_geodetic_crs: BaseGeodeticCrs::BaseDynamicGeographicCrs(
			BaseDynamicGeographicCrs {
				base_crs_name: "WGS 72".to_string(),
				dynamic_crs: DynamicCrs {
					frame_reference_epoch: FrameEpoch(1972.0),
					deformation_model_id: None,
				},
				geodetic_data: GeodeticReferenceFrame {
					datum_name: "World Geodetic System 1972".to_string(),
					ellipsoid: Ellipsoid {
						ellipsoid_name: "WGS 72".to_string(),
						semi_major_axis: 6378135.0,
						inverse_flattening: 298.26,
						length_unit: Some(LengthUnit::metre()),
						identifier: Some(Id::new_epsg(7043)),
					},
					anchor: None,
					anchor_epoch: None,
					identifier: Some(Id::new_epsg(6322)),
					prime_meridian: None,
				},
				ellipsoidal_cs_unit: None,
				identifier: Some(Id::new_epsg(4322)),
			},
		),
		map_projection: MapProjection {
			map_projection_name: "UTM zone 49S".to_string(),
			map_projection_method: Method {
				method_name: "Transverse Mercator".to_string(),
				identifier: Some(Id::new_epsg(9807)),
			},
			map_projection_parameters: Some(vec![
				Parameter {
					parameter_name: "Latitude of natural origin".to_string(),
					parameter_value: 0.0,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::AngleUnit(AngleUnit::degree()),
					)),
					identifier: Some(Id::new_epsg(8801)),
				},
				Parameter {
					parameter_name: "Longitude of natural origin".to_string(),
					parameter_value: 111.0,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::AngleUnit(AngleUnit::degree()),
					)),
					identifier: Some(Id::new_epsg(8802)),
				},
				Parameter {
					parameter_name: "Scale factor at natural origin"
						.to_string(),
					parameter_value: 0.9996,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::ScaleUnit(ScaleUnit {
							unit_name: "unity".to_string(),
							conversion_factor: 1.0,
							identifier: Some(Id::new_epsg(9201)),
						}),
					)),
					identifier: Some(Id::new_epsg(8805)),
				},
				Parameter {
					parameter_name: "False easting".to_string(),
					parameter_value: 500000.0,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::LengthUnit(LengthUnit::metre()),
					)),
					identifier: Some(Id::new_epsg(8806)),
				},
				Parameter {
					parameter_name: "False northing".to_string(),
					parameter_value: 10000000.0,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::LengthUnit(LengthUnit::metre()),
					)),
					identifier: Some(Id::new_epsg(8807)),
				},
			]),
			identifier: Some(Id::new_epsg(16149)),
		},
		coordinate_system: CoordinateSystem::SpatialCS(
			SpatialCoordinateSystem {
				spatial_cs_type: SpatialCsType::Cartesian,
				dimension: Dimension::Two,
				identifier: Some(Id::new_epsg(4400)),
				spatial_axis: vec![
					Axis {
						axis_name_abbreviation: "Easting (E)".to_string(),
						axis_direction: AxisDirection::East,
						axis_order: None,
						unit: None,
						identifier: None,
					},
					Axis {
						axis_name_abbreviation: "Northing (N)".to_string(),
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
			identifier: Some(vec![Id::new_epsg(32349)]),
			remark: None,
		},
	};

	let correct = vec![WktCrsTypes::ProjectedCrs(proj)];

	let ast = parse_wkt_crs(EXAMPLE).unwrap();

	assert_eq!(correct, ast);
}
