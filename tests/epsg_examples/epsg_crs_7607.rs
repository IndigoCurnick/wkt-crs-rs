use wkt_crs_rs::{
	WktCrsTypes,
	base_types::{
		AnchorEpoch, AngleUnit, Axis, BaseGeodeticCrs, BaseStaticGeographicCrs,
		CoordinateSystem, DatumAnchor, Ellipsoid, GeodeticReferenceFrame, Id,
		LengthUnit, MapProjection, Method, Parameter, ProjectedCrs, ScaleUnit,
		SpatialCoordinateSystem,
	},
	compound_types::{
		GeodeticData, ScopeExtentIdentifierRemark, SpatialUnit, Unit,
	},
	data_types::NumText,
	enumerations::{AxisDirection, Dimension, SpatialCsType},
	parse_wkt_crs,
};

const EXAMPLE: &str = r#"
PROJCRS[
	"NAD83(2011) / WISCRS Grant (ftUS)",
	BASEGEOGCRS[
		"NAD83(2011)",
		DATUM[
			"NAD83 (National Spatial Reference System 2011)",
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
			ANCHOREPOCH[2010],
			ID["EPSG",1116]
		],
		ID["EPSG",6318]
	],
	CONVERSION[
		"WISCRS Grant County (ftUS)",
		METHOD[
			"Transverse Mercator",
			ID["EPSG",9807]
		],
		PARAMETER[
			"Latitude of natural origin",
			41.4111111111114,
			ANGLEUNIT[
				"degree",
				0.0174532925199433,
				ID["EPSG",9102]
			],
			ID["EPSG",8801]
		],
		PARAMETER[
			"Longitude of natural origin",
			-90.8000000000003,
			ANGLEUNIT[
				"degree",
				0.0174532925199433,
				ID["EPSG",9102]
			],
			ID["EPSG",8802]
		],
		PARAMETER[
			"Scale factor at natural origin",
			1.0000349452,
			SCALEUNIT[
				"unity",
				1,
				ID["EPSG",9201]
			],
			ID["EPSG",8805]
		],
		PARAMETER[
			"False easting",
			794999.998,
			LENGTHUNIT[
				"US survey foot",
				0.304800609601219,
				ID["EPSG",9003]
			],
			ID["EPSG",8806]
		],
		PARAMETER[
			"False northing",
			0.033,
			LENGTHUNIT[
				"US survey foot",
				0.304800609601219,
				ID["EPSG",9003]
			],
			ID["EPSG",8807]
		],
		ID["EPSG",7497]
	],
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
	],
	ID["EPSG",7607]
]
"#;

#[test]
fn test_epsg_crs_7607() {
	let correct = ProjectedCrs {
		crs_name: "NAD83(2011) / WISCRS Grant (ftUS)".to_string(),
		base_geodetic_crs: BaseGeodeticCrs::BaseStaticGeographicCrs(
			BaseStaticGeographicCrs {
				base_crs_name: "NAD83(2011)".to_string(),
				geodetic_data: GeodeticData::GeodeticReferenceFrame(
					GeodeticReferenceFrame {
						datum_name:
							"NAD83 (National Spatial Reference System 2011)"
								.to_string(),
						ellipsoid: Ellipsoid {
							ellipsoid_name: "GRS 1980".to_string(),
							semi_major_axis: 6378137.0,
							inverse_flattening: 298.257222101,
							length_unit: Some(LengthUnit {
								unit_name: "metre".to_string(),
								conversion_factor: 1.0,
								identifier: Some(Id::new_epsg(9001)),
							}),
							identifier: Some(Id::new_epsg(7019)),
						},
						anchor: None,
						anchor_epoch: Some(AnchorEpoch(2010.0)),
						identifier: Some(Id::new_epsg(1116)),
						prime_meridian: None,
					},
				),
				ellipsoidal_cs_unit: None,
				identifier: Some(Id::new_epsg(6318)),
			},
		),
		map_projection: MapProjection {
			map_projection_name: "WISCRS Grant County (ftUS)".to_string(),
			map_projection_method: Method {
				method_name: "Transverse Mercator".to_string(),
				identifier: Some(Id::new_epsg(9807)),
			},
			map_projection_parameters: Some(vec![
				Parameter {
					parameter_name: "Latitude of natural origin".to_string(),
					parameter_value: 41.4111111111114,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::AngleUnit(AngleUnit {
							unit_name: "degree".to_string(),
							conversion_factor: 0.0174532925199433,
							identifier: Some(Id::new_epsg(9102)),
						}),
					)),
					identifier: Some(Id::new_epsg(8801)),
				},
				Parameter {
					parameter_name: "Longitude of natural origin".to_string(),
					parameter_value: -90.8000000000003,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::AngleUnit(AngleUnit {
							unit_name: "degree".to_string(),
							conversion_factor: 0.0174532925199433,
							identifier: Some(Id::new_epsg(9102)),
						}),
					)),
					identifier: Some(Id::new_epsg(8802)),
				},
				Parameter {
					parameter_name: "Scale factor at natural origin"
						.to_string(),
					parameter_value: 1.0000349452,
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
					parameter_value: 794999.998,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::LengthUnit(LengthUnit {
							unit_name: "US survey foot".to_string(),
							conversion_factor: 0.304800609601219,
							identifier: Some(Id::new_epsg(9003)),
						}),
					)),
					identifier: Some(Id::new_epsg(8806)),
				},
				Parameter {
					parameter_name: "False northing".to_string(),
					parameter_value: 0.033,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::LengthUnit(LengthUnit {
							unit_name: "US survey foot".to_string(),
							conversion_factor: 0.304800609601219,
							identifier: Some(Id::new_epsg(9003)),
						}),
					)),
					identifier: Some(Id::new_epsg(8807)),
				},
			]),
			identifier: Some(Id::new_epsg(7497)),
		},
		coordinate_system: CoordinateSystem::SpatialCS(
			SpatialCoordinateSystem {
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
				cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(
					LengthUnit {
						unit_name: "US survey foot".to_string(),
						conversion_factor: 0.304800609601219,
						identifier: Some(Id {
							authority_name: "EPSG".to_string(),
							authority_unique_identifier: NumText::Int(9003),
							version: None,
							authority_citation: None,
							id_uri: None,
						}),
					},
				))),
			},
		),
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: None,
			identifier: Some(vec![Id {
				authority_name: "EPSG".to_string(),
				authority_unique_identifier: NumText::Int(7607),
				version: None,
				authority_citation: None,
				id_uri: None,
			}]),
			remark: None,
		},
	};

	let correct = vec![WktCrsTypes::ProjectedCrs(correct)];

	let ast = parse_wkt_crs(EXAMPLE).unwrap();

	assert_eq!(correct, ast);
}
