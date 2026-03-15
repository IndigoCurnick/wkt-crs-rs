use wkt_crs_rs::{
	WktCrsTypes,
	base_types::{
		AngleUnit, Axis, BaseGeodeticCrs, BaseStaticGeographicCrs,
		CoordinateSystem, Ellipsoid, GeodeticReferenceFrame, Id, LengthUnit,
		MapProjection, MapProjectionMethod, MapProjectionParameter,
		ProjectedCrs, ScaleUnit, SpatialCoordinateSystem,
	},
	compound_types::{
		GeodeticData, MapProjectionParameterUnit, ScopeExtentIdentifierRemark,
		SpatialUnit, Unit,
	},
	data_types::NumText,
	enumerations::{AxisDirection, Dimension, SpatialCsType},
	parse_wkt_crs,
};

const WKT: &'static str = r#"
PROJCRS[
    "NAD83(HARN) / Maine West (ftUS)",
    BASEGEOGCRS[
        "NAD83(HARN)",
        DATUM[
            "NAD83 (High Accuracy Reference Network)",
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
			ID["EPSG",6152]
		],
		ID["EPSG",4152]
	],
	CONVERSION[
		"SPCS83 Maine West zone (US survey foot)",
		METHOD[
			"Transverse Mercator",
			ID["EPSG",9807]
		],
		PARAMETER[
			"Latitude of natural origin",
			42.8333333333336,
			ANGLEUNIT[
				"degree",
				0.0174532925199433,
				ID["EPSG",9102]
			],
			ID["EPSG",8801]
		],
		PARAMETER[
			"Longitude of natural origin",
			-70.1666666666669,
			ANGLEUNIT[
				"degree",
				0.0174532925199433,
				ID["EPSG",9102]
			],
			ID["EPSG",8802]
		],
		PARAMETER[
			"Scale factor at natural origin",
			0.999966667,
			SCALEUNIT[
				"unity",
				1,
				ID["EPSG",9201]
			],
			ID["EPSG",8805]
		],
		PARAMETER[
			"False easting",
			2952750,
			LENGTHUNIT[
				"US survey foot",
				0.304800609601219,
				ID["EPSG",9003]
			],
			ID["EPSG",8806]
		],
		PARAMETER[
			"False northing",
			0,
			LENGTHUNIT[
				"US survey foot",
				0.304800609601219,
				ID["EPSG",9003]
			],
			ID["EPSG",8807]
		],
		ID["EPSG",11834]
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
	ID["EPSG",26856]
]
"#;

#[test]
fn test_epsg_crs_26856() {
	let insider = ProjectedCrs {
		crs_name: "NAD83(HARN) / Maine West (ftUS)".to_string(),
		base_geodetic_crs: BaseGeodeticCrs::BaseStaticGeographicCrs(
			BaseStaticGeographicCrs {
				base_crs_name: "NAD83(HARN)".to_string(),
				geodetic_data: GeodeticData::GeodeticReferenceFrame(
					GeodeticReferenceFrame {
						datum_name: "NAD83 (High Accuracy Reference Network)"
							.to_string(),
						ellipsoid: Ellipsoid {
							ellipsoid_name: "GRS 1980".to_string(),
							semi_major_axis: 6378137.0,
							inverse_flattening: 298.257222101,
							length_unit: Some(LengthUnit {
								unit_name: "metre".to_string(),
								conversion_factor: 1.0,
								identifier: Some(Id {
									authority_name: "EPSG".to_string(),
									authority_unique_identifier: NumText::Int(
										9001,
									),
									version: None,
									authority_citation: None,
									id_uri: None,
								}),
							}),
						},
						anchor: None,
						identifier: Some(Id {
							authority_name: "EPSG".to_string(),
							authority_unique_identifier: NumText::Int(7019),
							version: None,
							authority_citation: None,
							id_uri: None,
						}),
						prime_meridian: None,
					},
				),
				ellipsoidal_cs_unit: None,
				identifier: Some(Id {
					authority_name: "EPSG".to_string(),
					authority_unique_identifier: NumText::Int(6152),
					version: None,
					authority_citation: None,
					id_uri: None,
				}),
			},
		),
		map_projection: MapProjection {
			map_projection_name: "SPCS83 Maine West zone (US survey foot)"
				.to_string(),
			map_projection_method: MapProjectionMethod {
				map_projection_method_name: "Transverse Mercator".to_string(),
				identifier: Some(Id {
					authority_name: "EPSG".to_string(),
					authority_unique_identifier: NumText::Int(9807),
					version: None,
					authority_citation: None,
					id_uri: None,
				}),
			},
			map_projection_parameters: Some(vec![
				MapProjectionParameter {
					parameter_name: "Latitude of natural origin".to_string(),
					parameter_value: 42.8333333333336,
					map_projection_parameter_unit: Some(
						MapProjectionParameterUnit::AngleUnit(AngleUnit {
							unit_name: "degree".to_string(),
							conversion_factor: 0.0174532925199433,
							identifier: Some(Id {
								authority_name: "EPSG".to_string(),
								authority_unique_identifier: NumText::Int(9102),
								version: None,
								authority_citation: None,
								id_uri: None,
							}),
						}),
					),
					identifier: Some(Id {
						authority_name: "EPSG".to_string(),
						authority_unique_identifier: NumText::Int(8801),
						version: None,
						authority_citation: None,
						id_uri: None,
					}),
				},
				MapProjectionParameter {
					parameter_name: "Longitude of natural origin".to_string(),
					parameter_value: -70.1666666666669,
					map_projection_parameter_unit: Some(
						MapProjectionParameterUnit::AngleUnit(AngleUnit {
							unit_name: "degree".to_string(),
							conversion_factor: 0.0174532925199433,
							identifier: Some(Id {
								authority_name: "EPSG".to_string(),
								authority_unique_identifier: NumText::Int(9102),
								version: None,
								authority_citation: None,
								id_uri: None,
							}),
						}),
					),
					identifier: Some(Id {
						authority_name: "EPSG".to_string(),
						authority_unique_identifier: NumText::Int(8802),
						version: None,
						authority_citation: None,
						id_uri: None,
					}),
				},
				MapProjectionParameter {
					parameter_name: "Scale factor at natural origin"
						.to_string(),
					parameter_value: 0.999966667,
					map_projection_parameter_unit: Some(
						MapProjectionParameterUnit::ScaleUnit(ScaleUnit {
							unit_name: "unity".to_string(),
							conversion_factor: 1.0,
							identifier: Some(Id {
								authority_name: "EPSG".to_string(),
								authority_unique_identifier: NumText::Int(9201),
								version: None,
								authority_citation: None,
								id_uri: None,
							}),
						}),
					),
					identifier: Some(Id {
						authority_name: "EPSG".to_string(),
						authority_unique_identifier: NumText::Int(8805),
						version: None,
						authority_citation: None,
						id_uri: None,
					}),
				},
				MapProjectionParameter {
					parameter_name: "False easting".to_string(),
					parameter_value: 2952750.0,
					map_projection_parameter_unit: Some(
						MapProjectionParameterUnit::LengthUnit(LengthUnit {
							unit_name: "US survey foot".to_string(),
							conversion_factor: 0.304800609601219,
							identifier: Some(Id {
								authority_name: "EPSG".to_string(),
								authority_unique_identifier: NumText::Int(9003),
								version: None,
								authority_citation: None,
								id_uri: None,
							}),
						}),
					),
					identifier: Some(Id {
						authority_name: "EPSG".to_string(),
						authority_unique_identifier: NumText::Int(8806),
						version: None,
						authority_citation: None,
						id_uri: None,
					}),
				},
				MapProjectionParameter {
					parameter_name: "False northing".to_string(),
					parameter_value: 0.0,
					map_projection_parameter_unit: Some(
						MapProjectionParameterUnit::LengthUnit(LengthUnit {
							unit_name: "US survey foot".to_string(),
							conversion_factor: 0.304800609601219,
							identifier: Some(Id {
								authority_name: "EPSG".to_string(),
								authority_unique_identifier: NumText::Int(9003),
								version: None,
								authority_citation: None,
								id_uri: None,
							}),
						}),
					),
					identifier: Some(Id {
						authority_name: "EPSG".to_string(),
						authority_unique_identifier: NumText::Int(8807),
						version: None,
						authority_citation: None,
						id_uri: None,
					}),
				},
			]),
			identifier: Some(Id {
				authority_name: "EPSG".to_string(),
				authority_unique_identifier: NumText::Int(11834),
				version: None,
				authority_citation: None,
				id_uri: None,
			}),
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
				authority_unique_identifier: NumText::Int(26856),
				version: None,
				authority_citation: None,
				id_uri: None,
			}]),
			remark: None,
		},
	};

	let correct = vec![WktCrsTypes::ProjectedCrs(insider)];

	let ast = parse_wkt_crs(WKT).unwrap();

	assert_eq!(correct, ast);
}
