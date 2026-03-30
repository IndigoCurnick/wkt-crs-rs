use crate::{
	ast::parse_wkt,
	base_types::{
		AbridgedCoordinateTransformation, AngleUnit, Axis, BoundCrs,
		CoordinateSystem, Ellipsoid, GeodeticCrs, GeodeticReferenceFrame, Id,
		Method, OperationParameterFile, SourceCrs, SpatialCoordinateSystem,
		StaticGeodeticCrs, TargetCrs,
	},
	compound_types::{
		CoordinateReferenceSystem, GeodeticData, ScopeExtentIdentifierRemark,
		SingleCrs, SpatialUnit, Unit,
	},
	data_types::NumText,
	enumerations::{
		AxisDirection, Dimension, OperationParameterWrapper, SpatialCsType,
	},
	types::WktBaseType,
};

const EXAMPLE1: &str = r#"BOUNDCRS[
    SOURCECRS[
        GEODCRS["NAD27",
            DATUM["North American Datum 1927",
                ELLIPSOID["Clarke 1866",6378206.4,294.97]
            ],
            CS[ellipsoidal,2]
            AXIS["latitude",north],AXIS["longitude",east],
            ANGLEUNIT["degree",0.017]
        ]
    ],
    TARGETCRS[
        GEODCRS["NAD83",
            DATUM["North American Datum 1983",
                ELLIPSOID["GRS 1980",6378137,298.25]
            ],
            CS[ellipsoidal,2],
            AXIS["latitude",north],AXIS["longitude",east],
            ANGLEUNIT["degree",0.017]        
        ]
    ],
    ABRIDGEDTRANSFORMATION["NAD27 to NAD83 Alaska",
        METHOD["NADCON",ID["EPSG",9613]],
        PARAMETERFILE["Latitude difference file","alaska.las"],
        PARAMETERFILE["Longitude difference file","alaska.los"]]
]
"#;

#[test]
fn test_bound_crs() {
	let correct = BoundCrs {
		source_crs: SourceCrs {
			coordinate_system: CoordinateReferenceSystem::SingleCrs(
				SingleCrs::GeodeticCrs(GeodeticCrs::StaticGeodeticCrs(
					StaticGeodeticCrs {
						crs_name: "NAD27".into(),
						frame: GeodeticData::GeodeticReferenceFrame(
							GeodeticReferenceFrame {
								datum_name: "North American Datum 1927".into(),
								ellipsoid: Ellipsoid {
									ellipsoid_name: "Clarke 1866".into(),
									semi_major_axis: 6378206.4,
									inverse_flattening: 294.97,
									length_unit: None,
									identifier: None,
								},
								anchor: None,
								anchor_epoch: None,
								identifier: None,
								prime_meridian: None,
							},
						),
						coordinate_system: CoordinateSystem::SpatialCS(
							SpatialCoordinateSystem {
								spatial_cs_type: SpatialCsType::Ellipsoidal,
								dimension: Dimension::Two,
								identifier: None,
								spatial_axis: vec![
									Axis {
										axis_name_abbreviation: "latitude"
											.into(),
										axis_direction: AxisDirection::North(
											None,
										),
										axis_order: None,
										unit: None,
										identifier: None,
									},
									Axis {
										axis_name_abbreviation: "longitude"
											.into(),
										axis_direction: AxisDirection::East,
										axis_order: None,
										unit: None,
										identifier: None,
									},
								],
								cs_unit: Some(Unit::SpatialUnit(
									SpatialUnit::AngleUnit(AngleUnit {
										unit_name: "degree".into(),
										conversion_factor: 0.017,
										identifier: None,
									}),
								)),
							},
						),
						defining_transformation_id: None,
						scope_extent_identifier_remark:
							ScopeExtentIdentifierRemark {
								usage: None,
								identifier: None,
								remark: None,
							},
					},
				)),
			),
		},
		target_crs: TargetCrs {
			coordinate_system: CoordinateReferenceSystem::SingleCrs(
				SingleCrs::GeodeticCrs(GeodeticCrs::StaticGeodeticCrs(
					StaticGeodeticCrs {
						crs_name: "NAD83".into(),
						frame: GeodeticData::GeodeticReferenceFrame(
							GeodeticReferenceFrame {
								datum_name: "North American Datum 1983".into(),
								ellipsoid: Ellipsoid {
									ellipsoid_name: "GRS 1980".into(),
									semi_major_axis: 6378137.0,
									inverse_flattening: 298.25,
									length_unit: None,
									identifier: None,
								},
								anchor: None,
								anchor_epoch: None,
								identifier: None,
								prime_meridian: None,
							},
						),
						coordinate_system: CoordinateSystem::SpatialCS(
							SpatialCoordinateSystem {
								spatial_cs_type: SpatialCsType::Ellipsoidal,
								dimension: Dimension::Two,
								identifier: None,
								spatial_axis: vec![
									Axis {
										axis_name_abbreviation: "latitude"
											.into(),
										axis_direction: AxisDirection::North(
											None,
										),
										axis_order: None,
										unit: None,
										identifier: None,
									},
									Axis {
										axis_name_abbreviation: "longitude"
											.into(),
										axis_direction: AxisDirection::East,
										axis_order: None,
										unit: None,
										identifier: None,
									},
								],
								cs_unit: Some(Unit::SpatialUnit(
									SpatialUnit::AngleUnit(AngleUnit {
										unit_name: "degree".into(),
										conversion_factor: 0.017,
										identifier: None,
									}),
								)),
							},
						),
						defining_transformation_id: None,
						scope_extent_identifier_remark:
							ScopeExtentIdentifierRemark {
								identifier: None,
								remark: None,
								usage: None,
							},
					},
				)),
			),
		},
		abridged_coordinate_transformation: AbridgedCoordinateTransformation {
			operation_name: "NAD27 to NAD83 Alaska".into(),
			operation_version: None,
			operation_method: Method {
				method_name: "NADCON".into(),
				identifier: Some(Id {
					authority_name: "EPSG".into(),
					authority_unique_identifier: NumText::Int(9613),
					version: None,
					authority_citation: None,
					id_uri: None,
				}),
			},
			operation_parameter_wrapper: Some(vec![
				OperationParameterWrapper::OperationParameterFile(
					OperationParameterFile {
						parameter_name: "Latitude difference file".into(),
						parameter_file_name: "alaska.las".into(),
						identifier: None,
					},
				),
				OperationParameterWrapper::OperationParameterFile(
					OperationParameterFile {
						parameter_name: "Longitude difference file".into(),
						parameter_file_name: "alaska.los".into(),
						identifier: None,
					},
				),
			]),
			scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
				usage: None,
				identifier: None,
				remark: None,
			},
		},
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: None,
			identifier: None,
			remark: None,
		},
	};

	let ast = parse_wkt(EXAMPLE1).unwrap();

	assert_eq!(ast.len(), 1);

	let order = BoundCrs::from_nodes(&ast).unwrap();

	assert_eq!(correct, order.result);
}
