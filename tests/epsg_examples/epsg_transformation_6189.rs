use wkt_crs_rs::{
	WktCrsTypes,
	base_types::{
		AngleUnit, Axis, CoordinateOperation, CoordinateSystem,
		DatumEnsembleAccuracy, DatumEnsembleMember, Ellipsoid, GeodeticCrs,
		GeodeticDatumEnsemble, GeodeticReferenceFrame, GeographicCrs, Id,
		LengthUnit, Method, OperationAccuracy, OperationParameterFile,
		OperationVersion, SourceCrs, SpatialCoordinateSystem,
		StaticGeodeticCrs, StaticGeographicCrs, TargetCrs,
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

const WKT: &str = r#"
COORDINATEOPERATION[
	"Datum 73 to ETRS89 (6)",
	VERSION["IGP-Prt 0.1m"],
	SOURCECRS[
		GEOGCRS[
			"Datum 73",
			DATUM[
				"Datum 73",
				ELLIPSOID[
					"International 1924",
					6378388,
					297,
					LENGTHUNIT[
						"metre",
						1,
						ID["EPSG",9001]
					],
					ID["EPSG",7022]
				],
				ID["EPSG",6274]
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
			ID["EPSG",4274]
		]
	],
	TARGETCRS[
		GEOGCRS[
			"ETRS89",
			ENSEMBLE[
				"European Terrestrial Reference System 1989 ensemble", 
				MEMBER[
					"European Terrestrial Reference Frame 1989", 
					ID["EPSG",1178]
				], 
				MEMBER[
					"European Terrestrial Reference Frame 1990", 
					ID["EPSG",1179]
				], 
				MEMBER[
					"European Terrestrial Reference Frame 1991", 
					ID["EPSG",1180]
					],
				MEMBER[
					"European Terrestrial Reference Frame 1992", 
					ID["EPSG",1181]
				], 
				MEMBER[
					"European Terrestrial Reference Frame 1993", 
					ID["EPSG",1182]
				], 
				MEMBER[
					"European Terrestrial Reference Frame 1994", 
					ID["EPSG",1183]
				], 
				MEMBER[
					"European Terrestrial Reference Frame 1996", 
					ID["EPSG",1184]
				], 
				MEMBER[
					"European Terrestrial Reference Frame 1997", 
					ID["EPSG",1185]
				], 
				MEMBER[
					"European Terrestrial Reference Frame 2000", 
					ID["EPSG",1186]
				], 
				MEMBER[
					"European Terrestrial Reference Frame 2005", 
					ID["EPSG",1204]
				], 
				MEMBER[
					"European Terrestrial Reference Frame 2014", 
					ID["EPSG",1206]
				], 
				MEMBER[
					"European Terrestrial Reference Frame 2020", 
					ID["EPSG",1382]
				],
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
				ENSEMBLEACCURACY[0.1],
				ID["EPSG",6258]
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
			ID["EPSG",4258]
		]
	],
	METHOD[
		"NTv2",
		ID["EPSG",9615]
	],
	PARAMETERFILE[
		"Latitude and longitude difference file",
		"D73_ETRS89_geo.gsb",
		ID["EPSG",8656]
	],
	OPERATIONACCURACY[0.1],
	ID["EPSG",6189]
]
"#;

#[test]
fn test_epsg_transformation_6189() {
	let insider = CoordinateOperation {
		operation_name: "Datum 73 to ETRS89 (6)".to_string(),
		operation_version: Some(OperationVersion("IGP-Prt 0.1m".to_string())),
		source_crs: SourceCrs {
		coordinate_system: CoordinateReferenceSystem::SingleCrs(
			SingleCrs::GeodeticCrs(GeodeticCrs::GeographicCrs(
			GeographicCrs::StaticGeographicCrs(StaticGeographicCrs {
				crs_name: "Datum 73".to_string(),
				frame: GeodeticData::GeodeticReferenceFrame(
				GeodeticReferenceFrame {
					datum_name: "Datum 73".to_string(),
					ellipsoid: Ellipsoid {
					ellipsoid_name: "International 1924".to_string(),
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
					identifier: Some(Id::new_epsg(6274)),
					prime_meridian: None,
				},
				),
				coordinate_system: CoordinateSystem::SpatialCS(
				SpatialCoordinateSystem {
					spatial_cs_type: SpatialCsType::Ellipsoidal,
					dimension: Dimension::Two,
					identifier: Some(Id::new_epsg(6422)),
					spatial_axis: vec![
					Axis {
						axis_name_abbreviation: "Geodetic latitude (Lat)"
						.to_string(),
						axis_direction: AxisDirection::North(None),
						axis_order: None,
						unit: None,
						identifier: None,
					},
					Axis {
						axis_name_abbreviation: "Geodetic longitude (Lon)"
						.to_string(),
						axis_direction: AxisDirection::East,
						axis_order: None,
						unit: None,
						identifier: None,
					},
					],
					cs_unit: Some(Unit::SpatialUnit(SpatialUnit::AngleUnit(
					AngleUnit {
						unit_name: "degree".to_string(),
						conversion_factor: 0.0174532925199433,
						identifier: Some(Id::new_epsg(9102)),
					},
					))),
				},
				),
				defining_transformation_id: None,
				scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
				usage: None,
				identifier: Some(vec![Id::new_epsg(4274)]),
				remark: None,
				},
			}),
			)),
		),
		},
		target_crs: TargetCrs {
		coordinate_system: CoordinateReferenceSystem::SingleCrs(
			SingleCrs::GeodeticCrs(GeodeticCrs::StaticGeodeticCrs(
			StaticGeodeticCrs {
				crs_name: "ETRS89".to_string(),
				frame: GeodeticData::GeodeticDatumEnsemble(GeodeticDatumEnsemble {
				datum_ensemble_name:
					"European Terrestrial Reference System 1989 ensemble"
					.to_string(),
				datum_ensemble_member: vec![
					DatumEnsembleMember {
					ensemble_member_name:
						"European Terrestrial Reference Frame 1989".to_string(),
					identifier: Some(Id::new_epsg(1178)),
					},
					DatumEnsembleMember {
					ensemble_member_name:
						"European Terrestrial Reference Frame 1990".to_string(),
					identifier: Some(Id::new_epsg(1179)),
					},
					DatumEnsembleMember {
					ensemble_member_name:
						"European Terrestrial Reference Frame 1991".to_string(),
					identifier: Some(Id::new_epsg(1180)),
					},
					DatumEnsembleMember {
					ensemble_member_name:
						"European Terrestrial Reference Frame 1992".to_string(),
					identifier: Some(Id::new_epsg(1181)),
					},
					DatumEnsembleMember {
					ensemble_member_name:
						"European Terrestrial Reference Frame 1993".to_string(),
					identifier: Some(Id::new_epsg(1182)),
					},
					DatumEnsembleMember {
					ensemble_member_name:
						"European Terrestrial Reference Frame 1994".to_string(),
					identifier: Some(Id::new_epsg(1183)),
					},
					DatumEnsembleMember {
					ensemble_member_name:
						"European Terrestrial Reference Frame 1996".to_string(),
					identifier: Some(Id::new_epsg(1184)),
					},
					DatumEnsembleMember {
					ensemble_member_name:
						"European Terrestrial Reference Frame 1997".to_string(),
					identifier: Some(Id::new_epsg(1185)),
					},
					DatumEnsembleMember {
					ensemble_member_name:
						"European Terrestrial Reference Frame 2000".to_string(),
					identifier: Some(Id::new_epsg(1186)),
					},
					DatumEnsembleMember {
					ensemble_member_name:
						"European Terrestrial Reference Frame 2005".to_string(),
					identifier: Some(Id::new_epsg(1204)),
					},
					DatumEnsembleMember {
					ensemble_member_name:
						"European Terrestrial Reference Frame 2014".to_string(),
					identifier: Some(Id::new_epsg(1206)),
					},
					DatumEnsembleMember {
					ensemble_member_name:
						"European Terrestrial Reference Frame 2020".to_string(),
					identifier: Some(Id::new_epsg(1382)),
					},
				],
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
				datum_ensemble_accuracy: DatumEnsembleAccuracy(0.1),
				identifier: Some(Id::new_epsg(7019)),
				prime_meridian: None,
				}),
				coordinate_system: CoordinateSystem::SpatialCS(
				SpatialCoordinateSystem {
					spatial_cs_type: SpatialCsType::Ellipsoidal,
					dimension: Dimension::Two,
					identifier: Some(Id::new_epsg(6422)),
					spatial_axis: vec![
					Axis {
						axis_name_abbreviation: "Geodetic latitude (Lat)"
						.to_string(),
						axis_direction: AxisDirection::North(None),
						axis_order: None,
						unit: None,
						identifier: None,
					},
					Axis {
						axis_name_abbreviation: "Geodetic longitude (Lon)"
						.to_string(),
						axis_direction: AxisDirection::East,
						axis_order: None,
						unit: None,
						identifier: None,
					},
					],
					cs_unit: Some(Unit::SpatialUnit(SpatialUnit::AngleUnit(
					AngleUnit {
						unit_name: "degree".to_string(),
						conversion_factor: 0.0174532925199433,
						identifier: Some(Id::new_epsg(9102)),
					},
					))),
				},
				),
				defining_transformation_id: None,
				scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
				usage: None,
				identifier: None,
				remark: None,
				},
			},
			)),
		),
		},
		operation_method: Method {
		method_name: "NTv2".to_string(),
		identifier: Some(Id::new_epsg(9615)),
		},
		operation_parameter_wrapper: Some(vec![
		OperationParameterWrapper::OperationParameterFile(
			OperationParameterFile {
			parameter_name: "Latitude and longitude difference file".to_string(),
			parameter_file_name: "D73_ETRS89_geo.gsb".to_string(),
			identifier: Some(Id::new_epsg(8656)),
			},
		),
		]),
		interpolation_crs: None,
		operation_accuracy: Some(OperationAccuracy(0.1)),
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
		usage: None,
		identifier: Some(vec![Id::new_epsg(6189)]),
		remark: None,
		},
	};

	let correct = vec![WktCrsTypes::CoordinateOperation(insider)];

	let ast = parse_wkt_crs(WKT).unwrap();

	assert_eq!(correct, ast);
}
