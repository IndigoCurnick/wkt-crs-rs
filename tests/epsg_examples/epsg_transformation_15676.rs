use wkt_crs_rs::{
	WktCrsTypes,
	base_types::{
		AngleUnit, Axis, CompoundCrs, CoordinateOperation, CoordinateSystem,
		DatumEnsembleAccuracy, DatumEnsembleMember, DynamicCrs,
		DynamicGeographicCrs, Ellipsoid, GeodeticCrs, GeodeticDatumEnsemble,
		GeodeticReferenceFrame, GeographicCrs, Id, LengthUnit, Method,
		OperationAccuracy, OperationVersion, Parameter, SourceCrs,
		SpatialCoordinateSystem, StaticGeographicCrs, StaticVerticalCrs,
		TargetCrs, VerticalCrs, VerticalReferenceFrame,
	},
	compound_types::{
		CoordinateReferenceSystem, GeodeticData, ScopeExtentIdentifierRemark,
		SingleCrs, SpatialUnit, Unit, VerticalFrameDatum,
	},
	enumerations::{
		AxisDirection, Dimension, OperationParameterWrapper, SpatialCsType,
	},
	parse_wkt_crs,
};

const EXAMPLE: &str = r#"
COORDINATEOPERATION[
	"Tokyo + JSLD height to WGS 84 (87)",
	VERSION["GSI-Jpn 340137"],
	SOURCECRS[
		COMPOUNDCRS[
			"Tokyo + JSLD69 height",
			GEOGCRS[
				"Tokyo",
				DATUM[
					"Tokyo",
					ELLIPSOID[
						"Bessel 1841",
						6377397.155,
						299.1528128,
						LENGTHUNIT[
							"metre",
							1,
							ID["EPSG",9001]
						],
						ID["EPSG",7004]
					],
					ID["EPSG",6301]
				],
				CS[
					ellipsoidal,
					2,
					ID["EPSG",6422]
				],
				AXIS[
					"Geodetic latitude (Lat)",
					north
				],
				AXIS[
					"Geodetic longitude (Lon)",
					east
				],
				ANGLEUNIT[
					"degree",
					0.0174532925199433,
					ID["EPSG",9102]
				],
				ID["EPSG",4301]
			],
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
			],
			ID["EPSG",7414]
		]
	],
		TARGETCRS[
			GEOGCRS[
				"WGS 84",
				ENSEMBLE[
					"World Geodetic System 1984 ensemble", 
					MEMBER[
						"World Geodetic System 1984 (Transit)", 
						ID["EPSG",1166]
					],
					MEMBER[
						"World Geodetic System 1984 (G730)", 
						ID["EPSG",1152]
					],
					MEMBER[
						"World Geodetic System 1984 (G873)", 
						ID["EPSG",1153]
					],
					MEMBER[
						"World Geodetic System 1984 (G1150)", 
						ID["EPSG",1154]
					],
					MEMBER[
						"World Geodetic System 1984 (G1674)",
						ID["EPSG",1155]
					],
					MEMBER[
						"World Geodetic System 1984 (G1762)",
						ID["EPSG",1156]
					],
					MEMBER[
						"World Geodetic System 1984 (G2139)",
						ID["EPSG",1309]
					],
					MEMBER[
						"World Geodetic System 1984 (G2296)",
						ID["EPSG",1383]
					],
					ELLIPSOID[
						"WGS 84",
						6378137,
						298.257223563,
						LENGTHUNIT[
							"metre",
							1,
							ID["EPSG",9001]
						],
						ID["EPSG",7030]
					],
					ENSEMBLEACCURACY[2],
					ID["EPSG",6326]
				],
				CS[
					ellipsoidal,
					3,
					ID["EPSG",6423]
				],
				AXIS[
					"Geodetic latitude (Lat)",
					north,
					ANGLEUNIT[
						"degree",
						0.0174532925199433,
						ID["EPSG",9102]
					]
				],
				AXIS[
					"Geodetic longitude (Lon)",
					east,
					ANGLEUNIT[
						"degree",
						0.0174532925199433,
						ID["EPSG",9102]
					]
				],
				AXIS[
					"Ellipsoidal height (h)",
					up,
					LENGTHUNIT[
					"metre",
					1,
					ID["EPSG",9001]
				]
			],
			ID["EPSG",4979]
		]
	],
	METHOD[
		"Geographic2D with Height Offsets",
		ID["EPSG",9618]
	],
	PARAMETER[
		"Latitude offset",
		11.9,
		ANGLEUNIT[
			"arc-second",
			0.0000048481368111,
			ID["EPSG",9104]
		],
		ID["EPSG",8601]
	],
	PARAMETER[
		"Longitude offset",
		-10.7,
		ANGLEUNIT[
			"arc-second",
			0.0000048481368111,
			ID["EPSG",9104]
		],
		ID["EPSG",8602]
	],
	PARAMETER[
		"Geoid height",
		39.3,
		LENGTHUNIT[
			"metre",
			1,
			ID["EPSG",9001]
		],
		ID["EPSG",8604]
	],
	OPERATIONACCURACY[1],
	ID["EPSG",15676]
]
"#;

#[test]
fn test_epsg_transformation_15676() {
	let source_crs = SourceCrs {
		coordinate_system: CoordinateReferenceSystem::CompoundCrs(
			CompoundCrs {
				compound_crs_name: "Tokyo + JSLD69 height".to_string(),
				crs_one: SingleCrs::GeodeticCrs(GeodeticCrs::GeographicCrs(
					GeographicCrs::StaticGeographicCrs(StaticGeographicCrs {
						crs_name: "Tokyo".to_string(),
						frame: GeodeticData::GeodeticReferenceFrame(
							GeodeticReferenceFrame {
								datum_name: "Tokyo".to_string(),
								ellipsoid: Ellipsoid {
									ellipsoid_name: "Bessel 1841".to_string(),
									semi_major_axis: 6377397.155,
									inverse_flattening: 299.1528128,
									length_unit: Some(LengthUnit::metre()),
									identifier: Some(Id::new_epsg(7004)),
								},
								prime_meridian: None,
								anchor: None,
								anchor_epoch: None,
								identifier: Some(Id::new_epsg(6301)),
							},
						),
						coordinate_system: CoordinateSystem::SpatialCS(
							SpatialCoordinateSystem {
								spatial_cs_type: SpatialCsType::Ellipsoidal,
								dimension: Dimension::Two,
								identifier: Some(Id::new_epsg(6422)),
								spatial_axis: vec![
									Axis {
										axis_name_abbreviation:
											"Geodetic latitude (Lat)"
												.to_string(),
										axis_direction: AxisDirection::North(
											None,
										),
										axis_order: None,
										unit: None,
										identifier: None,
									},
									Axis {
										axis_name_abbreviation:
											"Geodetic longitude (Lon)"
												.to_string(),
										axis_direction: AxisDirection::East,
										axis_order: None,
										unit: None,
										identifier: None,
									},
								],
								cs_unit: Some(Unit::SpatialUnit(
									SpatialUnit::AngleUnit(AngleUnit::degree()),
								)),
							},
						),
						defining_transformation_id: None,
						scope_extent_identifier_remark:
							ScopeExtentIdentifierRemark {
								usage: None,
								identifier: Some(vec![Id::new_epsg(4301)]),
								remark: None,
							},
					}),
				)),
				crs_two: SingleCrs::VerticalCrs(
					VerticalCrs::StaticVerticalCrs(StaticVerticalCrs {
						crs_name: "JSLD69 height".to_string(),
						vertical_frame_datum:
							VerticalFrameDatum::VerticalReferenceFrame(
								VerticalReferenceFrame {
									datum_name:
										"Japanese Standard Levelling Datum 1969"
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
									axis_name_abbreviation:
										"Gravity-related height (H)".to_string(),
									axis_direction: AxisDirection::Up,
									axis_order: None,
									unit: None,
									identifier: None,
								}],
								cs_unit: Some(Unit::SpatialUnit(
									SpatialUnit::LengthUnit(LengthUnit::metre()),
								)),
							},
						),
						geoid_model_id: None,
						scope_extent_identifier_remark:
							ScopeExtentIdentifierRemark {
								usage: None,
								identifier: Some(vec![Id::new_epsg(5723)]),
								remark: None,
							},
					}),
				),
				additional_crs: None,
				scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
					usage: None,
					identifier: Some(vec![Id::new_epsg(7414)]),
					remark: None,
				},
			},
		),
	};

	let target_crs = TargetCrs {
		coordinate_system: CoordinateReferenceSystem::SingleCrs(
			SingleCrs::GeodeticCrs(GeodeticCrs::GeographicCrs(
				GeographicCrs::StaticGeographicCrs(StaticGeographicCrs {
					crs_name: "WGS 84".to_string(),
					frame: GeodeticData::GeodeticDatumEnsemble(
						GeodeticDatumEnsemble {
							datum_ensemble_name:
								"World Geodetic System 1984 ensemble"
									.to_string(),
							datum_ensemble_member: vec![
								DatumEnsembleMember {
									ensemble_member_name:
										"World Geodetic System 1984 (Transit)"
											.to_string(),
									identifier: Some(Id::new_epsg(1166)),
								},
								DatumEnsembleMember {
									ensemble_member_name:
										"World Geodetic System 1984 (G730)"
											.to_string(),
									identifier: Some(Id::new_epsg(1152)),
								},
								DatumEnsembleMember {
									ensemble_member_name:
										"World Geodetic System 1984 (G873)"
											.to_string(),
									identifier: Some(Id::new_epsg(1153)),
								},
								DatumEnsembleMember {
									ensemble_member_name:
										"World Geodetic System 1984 (G1150)"
											.to_string(),
									identifier: Some(Id::new_epsg(1154)),
								},
								DatumEnsembleMember {
									ensemble_member_name:
										"World Geodetic System 1984 (G1674)"
											.to_string(),
									identifier: Some(Id::new_epsg(1155)),
								},
								DatumEnsembleMember {
									ensemble_member_name:
										"World Geodetic System 1984 (G1762)"
											.to_string(),
									identifier: Some(Id::new_epsg(1156)),
								},
								DatumEnsembleMember {
									ensemble_member_name:
										"World Geodetic System 1984 (G2139)"
											.to_string(),
									identifier: Some(Id::new_epsg(1309)),
								},
								DatumEnsembleMember {
									ensemble_member_name:
										"World Geodetic System 1984 (G2296)"
											.to_string(),
									identifier: Some(Id::new_epsg(1383)),
								},
							],
							ellipsoid: Ellipsoid {
								ellipsoid_name: "WGS 84".to_string(),
								semi_major_axis: 6378137.0,
								inverse_flattening: 298.257223563,
								length_unit: Some(LengthUnit::metre()),
								identifier: Some(Id::new_epsg(7030)),
							},
							datum_ensemble_accuracy: DatumEnsembleAccuracy(2.0),
							identifier: Some(Id::new_epsg(6326)),
							prime_meridian: None,
						},
					),
					coordinate_system: CoordinateSystem::SpatialCS(
						SpatialCoordinateSystem {
							spatial_cs_type: SpatialCsType::Ellipsoidal,
							dimension: Dimension::Three,
							identifier: Some(Id::new_epsg(6423)),
							spatial_axis: vec![
								Axis {
									axis_name_abbreviation:
										"Geodetic latitude (Lat)".to_string(),
									axis_direction: AxisDirection::North(None),
									axis_order: None,
									unit: Some(Unit::SpatialUnit(
										SpatialUnit::AngleUnit(
											AngleUnit::degree(),
										),
									)),
									identifier: None,
								},
								Axis {
									axis_name_abbreviation:
										"Geodetic longitude (Lon)".to_string(),
									axis_direction: AxisDirection::East,
									axis_order: None,
									unit: Some(Unit::SpatialUnit(
										SpatialUnit::AngleUnit(
											AngleUnit::degree(),
										),
									)),
									identifier: None,
								},
								Axis {
									axis_name_abbreviation:
										"Ellipsoidal height (h)".to_string(),
									axis_direction: AxisDirection::Up,
									axis_order: None,
									unit: Some(Unit::SpatialUnit(
										SpatialUnit::LengthUnit(
											LengthUnit::metre(),
										),
									)),
									identifier: None,
								},
							],
							cs_unit: None,
						},
					),
					defining_transformation_id: None,
					scope_extent_identifier_remark:
						ScopeExtentIdentifierRemark {
							usage: None,
							identifier: Some(vec![Id::new_epsg(4979)]),
							remark: None,
						},
				}),
			)),
		),
	};

	let inner = WktCrsTypes::CoordinateOperation(CoordinateOperation {
		operation_name: "Tokyo + JSLD height to WGS 84 (87)".to_string(),
		operation_version: Some(OperationVersion("GSI-Jpn 340137".to_string())),
		source_crs: source_crs,
		target_crs: target_crs,
		operation_method: Method {
			method_name: "Geographic2D with Height Offsets".to_string(),
			identifier: Some(Id::new_epsg(9618)),
		},
		operation_parameter_wrapper: Some(vec![
			OperationParameterWrapper::OperationParameter(Parameter {
				parameter_name: "Latitude offset".to_string(),
				parameter_value: 11.9,
				parameter_unit: Some(Unit::SpatialUnit(
					SpatialUnit::AngleUnit(AngleUnit {
						unit_name: "arc-second".to_string(),
						conversion_factor: 0.0000048481368111,
						identifier: Some(Id::new_epsg(9104)),
					}),
				)),
				identifier: Some(Id::new_epsg(8601)),
			}),
			OperationParameterWrapper::OperationParameter(Parameter {
				parameter_name: "Longitude offset".to_string(),
				parameter_value: -10.7,
				parameter_unit: Some(Unit::SpatialUnit(
					SpatialUnit::AngleUnit(AngleUnit {
						unit_name: "arc-second".to_string(),
						conversion_factor: 0.0000048481368111,
						identifier: Some(Id::new_epsg(9104)),
					}),
				)),
				identifier: Some(Id::new_epsg(8602)),
			}),
			OperationParameterWrapper::OperationParameter(Parameter {
				parameter_name: "Geoid height".to_string(),
				parameter_value: 39.3,
				parameter_unit: Some(Unit::SpatialUnit(
					SpatialUnit::LengthUnit(LengthUnit::metre()),
				)),
				identifier: Some(Id::new_epsg(8604)),
			}),
		]),
		interpolation_crs: None,
		operation_accuracy: Some(OperationAccuracy(1.0)),
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: None,
			identifier: Some(vec![Id::new_epsg(15676)]),
			remark: None,
		},
	});

	let correct = vec![inner];

	let ast = parse_wkt_crs(EXAMPLE).unwrap();

	assert_eq!(correct, ast);
}
