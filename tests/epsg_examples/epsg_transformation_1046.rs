use wkt_crs_rs::{
	WktCrsTypes,
	base_types::{
		AngleUnit, Axis, BaseGeodeticCrs, BaseStaticGeographicCrs,
		CoordinateOperation, CoordinateSystem, DefiningTransformation,
		Ellipsoid, GeodeticReferenceFrame, Id, LengthUnit, MapProjection,
		Method, OperationAccuracy, OperationVersion, Parameter, ProjectedCrs,
		ScaleUnit, SourceCrs, SpatialCoordinateSystem, TargetCrs,
	},
	compound_types::{
		CoordinateReferenceSystem, GeodeticData, ScopeExtentIdentifierRemark,
		SingleCrs, SpatialUnit, Unit,
	},
	enumerations::{
		AxisDirection, Dimension, OperationParameterWrapper, SpatialCsType,
	},
	parse_wkt_crs,
};

const EXAMPLE: &str = r#"
COORDINATEOPERATION[
	"Amersfoort / RD New to ED50 / UTM zone 31N (2)",
	VERSION["NAM-Nld"],
	SOURCECRS[
		PROJCRS[
			"Amersfoort / RD New",
			BASEGEOGCRS[
				"Amersfoort",
				DATUM[
					"Amersfoort",
					ELLIPSOID[
						"Bessel 1841",
						6377397.155,
						299.1528128,
						LENGTHUNIT["metre",1,ID["EPSG",9001]],
						ID["EPSG",7004]
					],
					ID["EPSG",6289]
				],
				DEFININGTRANSFORMATION[
					"Amersfoort to ETRS89 (9)",
					ID["EPSG",9282]
				],
				ID["EPSG",4289]
			],
			CONVERSION[
				"RD New",
				METHOD[
					"Oblique Stereographic",
					ID["EPSG",9809]
				],
				PARAMETER[
					"Latitude of natural origin",
					52.1561605555558,
					ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],
					ID["EPSG",8801]
				],
				PARAMETER[
					"Longitude of natural origin",
					5.38763888888917,
					ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],
					ID["EPSG",8802]
				],
				PARAMETER[
					"Scale factor at natural origin",
					0.9999079,
					SCALEUNIT["unity",1,ID["EPSG",9201]],
					ID["EPSG",8805]
				],
				PARAMETER[
					"False easting",
					155000,
					LENGTHUNIT["metre",1,ID["EPSG",9001]],
					ID["EPSG",8806]
				],
				PARAMETER[
					"False northing",
					463000,
					LENGTHUNIT["metre",1,ID["EPSG",9001]],
					ID["EPSG",8807]
				],
				ID["EPSG",19914]
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
			LENGTHUNIT["metre",1,ID["EPSG",9001]],
			ID["EPSG",28992]
		]
	],
	TARGETCRS[
		PROJCRS[
			"ED50 / UTM zone 31N",
			BASEGEOGCRS[
				"ED50",
				DATUM[
					"European Datum 1950",
					ELLIPSOID[
						"International 1924",
						6378388,
						297,
						LENGTHUNIT["metre",1,ID["EPSG",9001]],
						ID["EPSG",7022]
					],
					ID["EPSG",6230]
				],
				ID["EPSG",4230]
			],
			CONVERSION[
				"UTM zone 31N",
				METHOD[
					"Transverse Mercator",
					ID["EPSG",9807]
				],
				PARAMETER[
					"Latitude of natural origin",
					0,
					ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],
					ID["EPSG",8801]
				],
				PARAMETER[
					"Longitude of natural origin",
					3,
					ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],
					ID["EPSG",8802]
				],
				PARAMETER[
					"Scale factor at natural origin",
					0.9996,
					SCALEUNIT["unity",1,ID["EPSG",9201]],
					ID["EPSG",8805]
				],
				PARAMETER[
					"False easting",
					500000,
					LENGTHUNIT["metre",1,ID["EPSG",9001]],
					ID["EPSG",8806]
				],
				PARAMETER[
					"False northing",
					0,
					LENGTHUNIT["metre",1,ID["EPSG",9001]],
					ID["EPSG",8807]
				],
				ID["EPSG",16031]
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
			LENGTHUNIT["metre",1,ID["EPSG",9001]],
			ID["EPSG",23031]
		]
	],
	METHOD[
		"Complex polynomial of degree 4",
		ID["EPSG",9653]
	],
	PARAMETER[
		"Ordinate 1 of evaluation point in source CRS",
		155000,
		LENGTHUNIT["metre",1,ID["EPSG",9001]],
		ID["EPSG",8619]
	],
	PARAMETER[
		"Ordinate 2 of evaluation point in source CRS",
		463000,
		LENGTHUNIT["metre",1,ID["EPSG",9001]],
		ID["EPSG",8620]
	],
	OPERATIONACCURACY[1],
	ID["EPSG",1046]
]
"#;

#[test]
fn test_epsg_transformation_1046() {
	let src_crs = SourceCrs {
		coordinate_system: CoordinateReferenceSystem::SingleCrs(
			SingleCrs::ProjectedCrs(ProjectedCrs {
				crs_name: "Amersfoort / RD New".to_string(),
				base_geodetic_crs: BaseGeodeticCrs::BaseStaticGeographicCrs(
					BaseStaticGeographicCrs {
						base_crs_name: "Amersfoort".to_string(),
						geodetic_data: GeodeticData::GeodeticReferenceFrame(
							GeodeticReferenceFrame {
								datum_name: "Amersfoort".to_string(),
								ellipsoid: Ellipsoid {
									ellipsoid_name: "Bessel 1841".to_string(),
									semi_major_axis: 6377397.155,
									inverse_flattening: 299.1528128,
									length_unit: Some(LengthUnit::metre()),
									identifier: Some(Id::new_epsg(7004)),
								},
								anchor: None,
								anchor_epoch: None,
								identifier: Some(Id::new_epsg(6289)),
								prime_meridian: None,
							},
						),
						ellipsoidal_cs_unit: None,
						defining_transformation: Some(vec![
							DefiningTransformation {
								defining_transformation_name:
									"Amersfoort to ETRS89 (9)".to_string(),
								identifier: Some(Id::new_epsg(9282)),
							},
						]),
						identifier: Some(Id::new_epsg(4289)),
					},
				),
				map_projection: MapProjection {
					map_projection_name: "RD New".to_string(),
					map_projection_method: Method {
						method_name: "Oblique Stereographic".to_string(),
						identifier: Some(Id::new_epsg(9809)),
					},
					map_projection_parameters: Some(vec![
						Parameter {
							parameter_name: "Latitude of natural origin"
								.to_string(),
							parameter_value: 52.1561605555558,
							parameter_unit: Some(Unit::SpatialUnit(
								SpatialUnit::AngleUnit(AngleUnit::degree()),
							)),
							identifier: Some(Id::new_epsg(8801)),
						},
						Parameter {
							parameter_name: "Longitude of natural origin"
								.to_string(),
							parameter_value: 5.38763888888917,
							parameter_unit: Some(Unit::SpatialUnit(
								SpatialUnit::AngleUnit(AngleUnit::degree()),
							)),
							identifier: Some(Id::new_epsg(8802)),
						},
						Parameter {
							parameter_name: "Scale factor at natural origin"
								.to_string(),
							parameter_value: 0.9999079,
							parameter_unit: Some(Unit::SpatialUnit(
								SpatialUnit::ScaleUnit(ScaleUnit::unity()),
							)),
							identifier: Some(Id::new_epsg(8805)),
						},
						Parameter {
							parameter_name: "False easting".to_string(),
							parameter_value: 155000.0,
							parameter_unit: Some(Unit::SpatialUnit(
								SpatialUnit::LengthUnit(LengthUnit::metre()),
							)),
							identifier: Some(Id::new_epsg(8806)),
						},
						Parameter {
							parameter_name: "False northing".to_string(),
							parameter_value: 463000.0,
							parameter_unit: Some(Unit::SpatialUnit(
								SpatialUnit::LengthUnit(LengthUnit::metre()),
							)),
							identifier: Some(Id::new_epsg(8807)),
						},
					]),
					identifier: Some(Id::new_epsg(19914)),
				},
				coordinate_system: CoordinateSystem::SpatialCS(
					SpatialCoordinateSystem {
						spatial_cs_type: SpatialCsType::Cartesian,
						dimension: Dimension::Two,
						identifier: Some(Id::new_epsg(4499)),
						spatial_axis: vec![
							Axis {
								axis_name_abbreviation: "Easting (X)"
									.to_string(),
								axis_direction: AxisDirection::East,
								axis_order: None,
								unit: None,
								identifier: None,
							},
							Axis {
								axis_name_abbreviation: "Northing (Y)"
									.to_string(),
								axis_direction: AxisDirection::North(None),
								axis_order: None,
								unit: None,
								identifier: None,
							},
						],
						cs_unit: Some(Unit::SpatialUnit(
							SpatialUnit::LengthUnit(LengthUnit::metre()),
						)),
					},
				),
				scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
					usage: None,
					identifier: Some(vec![Id::new_epsg(28992)]),
					remark: None,
				},
			}),
		),
	};

	let target_crs = TargetCrs {
		coordinate_system: CoordinateReferenceSystem::SingleCrs(
			SingleCrs::ProjectedCrs(ProjectedCrs {
				crs_name: "ED50 / UTM zone 31N".to_string(),
				base_geodetic_crs: BaseGeodeticCrs::BaseStaticGeographicCrs(
					BaseStaticGeographicCrs {
						base_crs_name: "ED50".to_string(),
						geodetic_data: GeodeticData::GeodeticReferenceFrame(
							GeodeticReferenceFrame {
								datum_name: "European Datum 1950".to_string(),
								ellipsoid: Ellipsoid {
									ellipsoid_name: "International 1924"
										.to_string(),
									semi_major_axis: 6378388.0,
									inverse_flattening: 297.0,
									length_unit: Some(LengthUnit {
										unit_name: "metre".to_string(),
										conversion_factor: 1.0,
										identifier: Some(Id::new_epsg(9001)),
									}),
									identifier: Some(Id::new_epsg(7022)),
								},
								anchor: None,
								anchor_epoch: None,
								identifier: Some(Id::new_epsg(6230)),
								prime_meridian: None,
							},
						),
						ellipsoidal_cs_unit: None,
						defining_transformation: None,
						identifier: Some(Id::new_epsg(4230)),
					},
				),
				map_projection: MapProjection {
					map_projection_name: "UTM zone 31N".to_string(),
					map_projection_method: Method {
						method_name: "Transverse Mercator".to_string(),
						identifier: Some(Id::new_epsg(9807)),
					},
					map_projection_parameters: Some(vec![
						Parameter {
							parameter_name: "Latitude of natural origin"
								.to_string(),
							parameter_value: 0.0,
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
							parameter_name: "Longitude of natural origin"
								.to_string(),
							parameter_value: 3.0,
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
								SpatialUnit::LengthUnit(LengthUnit {
									unit_name: "metre".to_string(),
									conversion_factor: 1.0,
									identifier: Some(Id::new_epsg(9001)),
								}),
							)),
							identifier: Some(Id::new_epsg(8806)),
						},
						Parameter {
							parameter_name: "False northing".to_string(),
							parameter_value: 0.0,
							parameter_unit: Some(Unit::SpatialUnit(
								SpatialUnit::LengthUnit(LengthUnit {
									unit_name: "metre".to_string(),
									conversion_factor: 1.0,
									identifier: Some(Id::new_epsg(9001)),
								}),
							)),
							identifier: Some(Id::new_epsg(8807)),
						},
					]),
					identifier: Some(Id::new_epsg(16031)),
				},
				coordinate_system: CoordinateSystem::SpatialCS(
					SpatialCoordinateSystem {
						spatial_cs_type: SpatialCsType::Cartesian,
						dimension: Dimension::Two,
						identifier: Some(Id::new_epsg(4400)),
						spatial_axis: vec![
							Axis {
								axis_name_abbreviation: "Easting (E)"
									.to_string(),
								axis_direction: AxisDirection::East,
								axis_order: None,
								unit: None,
								identifier: None,
							},
							Axis {
								axis_name_abbreviation: "Northing (N)"
									.to_string(),
								axis_direction: AxisDirection::North(None),
								axis_order: None,
								unit: None,
								identifier: None,
							},
						],
						cs_unit: Some(Unit::SpatialUnit(
							SpatialUnit::LengthUnit(LengthUnit::metre()),
						)),
					},
				),
				scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
					usage: None,
					identifier: Some(vec![Id::new_epsg(23031)]),
					remark: None,
				},
			}),
		),
	};

	let coord_op = CoordinateOperation {
		operation_name: "Amersfoort / RD New to ED50 / UTM zone 31N (2)"
			.to_string(),
		operation_version: Some(OperationVersion("NAM-Nld".to_string())),
		source_crs: src_crs,
		target_crs: target_crs,
		operation_method: Method {
			method_name: "Complex polynomial of degree 4".to_string(),
			identifier: Some(Id::new_epsg(9653)),
		},
		operation_parameter_wrapper: Some(vec![
			OperationParameterWrapper::OperationParameter(Parameter {
				parameter_name: "Ordinate 1 of evaluation point in source CRS"
					.to_string(),
				parameter_value: 155000.0,
				parameter_unit: Some(Unit::SpatialUnit(
					SpatialUnit::LengthUnit(LengthUnit::metre()),
				)),
				identifier: Some(Id::new_epsg(8619)),
			}),
			OperationParameterWrapper::OperationParameter(Parameter {
				parameter_name: "Ordinate 2 of evaluation point in source CRS"
					.to_string(),
				parameter_value: 463000.0,
				parameter_unit: Some(Unit::SpatialUnit(
					SpatialUnit::LengthUnit(LengthUnit::metre()),
				)),
				identifier: Some(Id::new_epsg(8620)),
			}),
		]),
		interpolation_crs: None,
		operation_accuracy: Some(OperationAccuracy(1.0)),
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: None,
			identifier: Some(vec![Id::new_epsg(1046)]),
			remark: None,
		},
	};

	let correct = vec![WktCrsTypes::CoordinateOperation(coord_op)];

	let ast = parse_wkt_crs(EXAMPLE).unwrap();

	assert_eq!(correct, ast);
}
