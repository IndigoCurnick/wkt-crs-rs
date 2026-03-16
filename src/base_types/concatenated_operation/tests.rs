use crate::{
	ast::parse_wkt,
	base_types::{
		AngleUnit, AreaDescription, Axis, ConcatenatedOperation,
		CoordinateSystem, Ellipsoid, Extent, GeodeticCrs,
		GeodeticReferenceFrame, Id, LengthUnit, MapProjection, Method,
		OperationAccuracy, Order, Parameter, Scope, SourceCrs,
		SpatialCoordinateSystem, StaticGeodeticCrs, TargetCrs, Usage,
	},
	compound_types::{
		CoordinateReferenceSystem, GeodeticData, ScopeExtentIdentifierRemark,
		SingleCrs, SpatialUnit, Step, Unit,
	},
	data_types::NumText,
	enumerations::{AxisDirection, Dimension, SpatialCsType},
	types::WktBaseType,
};

const EXAMPLE1: &str = r#"CONCATENATEDOPERATION["xxxx to zzzz",
    SOURCECRS[
        GEODCRS["Tokyo",
            DATUM["Tokyo 1918",
                ELLIPSOID["Bessel 1841",6377397.155,299.1528128,
                    LENGTHUNIT["metre",1.0]]],
            CS[Cartesian,3],
                AXIS["(X)",geocentricX,ORDER[1]],
                AXIS["(Y)",geocentricY,ORDER[2]],
                AXIS["(Z)",geocentricZ,ORDER[3]],
                LENGTHUNIT["metre",1.0]
        ]
    ],
    TARGETCRS[
        GEODCRS["JGD2000",
            DATUM["Japanese Geodetic Datum 2000",
                ELLIPSOID["GRS 1980",6378137.0,298.257222101,LENGTHUNIT["metre",1.0]]],
            CS[Cartesian,3],
                AXIS["(X)",geocentricX],
                AXIS["(Y)",geocentricY],
                AXIS["(Z)",geocentricZ],
                LENGTHUNIT["metre",1.0]
        ]
    ],
    STEP[
        CONVERSION["Kyrgyzstan zone 3",
            METHOD["Transverse Mercator",ID["EPSG",9807]],
            PARAMETER["Latitude of natural origin",0,
                ANGLEUNIT["degree",0.0174532925199433,
                ID["EPSG",9102]],
            ID["EPSG",8801]],
        PARAMETER["Longitude of natural origin",74.516666666667,
            ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],
            ID["EPSG",8802]],
            ID["EPSG",7689]]
        ],
    OPERATIONACCURACY[5],
    USAGE[SCOPE["Concatendated operation scope description."],
        AREA["Concatenated operation area description."]]
]
"#;

#[test]
fn test_concatenated_operation() {
	let correct = ConcatenatedOperation {
		operation_name: "xxxx to zzzz".to_string(),
		operation_version: None,
		source_crs: SourceCrs {
			coordinate_system: CoordinateReferenceSystem::SingleCrs(
				SingleCrs::GeodeticCrs(GeodeticCrs::StaticGeodeticCrs(
					StaticGeodeticCrs {
						crs_name: "Tokyo".into(),
						frame: GeodeticData::GeodeticReferenceFrame(
							GeodeticReferenceFrame {
								datum_name: "Tokyo 1918".into(),
								ellipsoid: Ellipsoid {
									ellipsoid_name: "Bessel 1841".into(),
									semi_major_axis: 6377397.155,
									inverse_flattening: 299.1528128,
									length_unit: Some(LengthUnit {
										unit_name: "metre".into(),
										conversion_factor: 1.0,
										identifier: None,
									}),
								},
								anchor: None,
								identifier: None,
								prime_meridian: None,
							},
						),
						coordinate_system: CoordinateSystem::SpatialCS(
							SpatialCoordinateSystem {
								spatial_cs_type: SpatialCsType::Cartesian,
								dimension: Dimension::Three,
								identifier: None,
								spatial_axis: vec![
									Axis {
										axis_name_abbreviation: "(X)".into(),
										axis_direction:
											AxisDirection::GeocentricX,
										axis_order: Some(Order(1)),
										unit: None,
										identifier: None,
									},
									Axis {
										axis_name_abbreviation: "(Y)".into(),
										axis_direction:
											AxisDirection::GeocentricY,
										axis_order: Some(Order(2)),
										unit: None,
										identifier: None,
									},
									Axis {
										axis_name_abbreviation: "(Z)".into(),
										axis_direction:
											AxisDirection::GeocentricZ,
										axis_order: Some(Order(3)),
										unit: None,
										identifier: None,
									},
								],
								cs_unit: Some(Unit::SpatialUnit(
									SpatialUnit::LengthUnit(LengthUnit {
										unit_name: "metre".into(),
										conversion_factor: 1.0,
										identifier: None,
									}),
								)),
							},
						),
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
		target_crs: TargetCrs {
			coordinate_system: CoordinateReferenceSystem::SingleCrs(
				SingleCrs::GeodeticCrs(GeodeticCrs::StaticGeodeticCrs(
					StaticGeodeticCrs {
						crs_name: "JGD2000".into(),
						frame: GeodeticData::GeodeticReferenceFrame(
							GeodeticReferenceFrame {
								datum_name: "Japanese Geodetic Datum 2000"
									.into(),
								ellipsoid: Ellipsoid {
									ellipsoid_name: "GRS 1980".into(),
									semi_major_axis: 6378137.0,
									inverse_flattening: 298.257222101,
									length_unit: Some(LengthUnit {
										unit_name: "metre".into(),
										conversion_factor: 1.0,
										identifier: None,
									}),
								},
								anchor: None,
								identifier: None,
								prime_meridian: None,
							},
						),
						coordinate_system: CoordinateSystem::SpatialCS(
							SpatialCoordinateSystem {
								spatial_cs_type: SpatialCsType::Cartesian,
								dimension: Dimension::Three,
								identifier: None,
								spatial_axis: vec![
									Axis {
										axis_name_abbreviation: "(X)".into(),
										axis_direction:
											AxisDirection::GeocentricX,
										axis_order: None,
										unit: None,
										identifier: None,
									},
									Axis {
										axis_name_abbreviation: "(Y)".into(),
										axis_direction:
											AxisDirection::GeocentricY,
										axis_order: None,
										unit: None,
										identifier: None,
									},
									Axis {
										axis_name_abbreviation: "(Z)".into(),
										axis_direction:
											AxisDirection::GeocentricZ,
										axis_order: None,
										unit: None,
										identifier: None,
									},
								],
								cs_unit: Some(Unit::SpatialUnit(
									SpatialUnit::LengthUnit(LengthUnit {
										unit_name: "metre".into(),
										conversion_factor: 1.0,
										identifier: None,
									}),
								)),
							},
						),
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
		step: vec![Step::MapProjection(MapProjection {
			map_projection_name: "Kyrgyzstan zone 3".to_string(),
			map_projection_method: Method {
				method_name: "Transverse Mercator".to_string(),
				identifier: Some(Id {
					authority_name: "EPSG".to_string(),
					authority_unique_identifier: NumText::Int(9807),
					version: None,
					authority_citation: None,
					id_uri: None,
				}),
			},
			map_projection_parameters: Some(vec![
				Parameter {
					parameter_name: "Latitude of natural origin".to_string(),
					parameter_value: 0.0,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::AngleUnit(AngleUnit {
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
					)),
					identifier: Some(Id {
						authority_name: "EPSG".to_string(),
						authority_unique_identifier: NumText::Int(8801),
						version: None,
						authority_citation: None,
						id_uri: None,
					}),
				},
				Parameter {
					parameter_name: "Longitude of natural origin".to_string(),
					parameter_value: 74.516666666667,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::AngleUnit(AngleUnit {
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
					)),
					identifier: Some(Id {
						authority_name: "EPSG".to_string(),
						authority_unique_identifier: NumText::Int(8802),
						version: None,
						authority_citation: None,
						id_uri: None,
					}),
				},
			]),
			identifier: Some(Id {
				authority_name: "EPSG".to_string(),
				authority_unique_identifier: NumText::Int(7689),
				version: None,
				authority_citation: None,
				id_uri: None,
			}),
		})],
		operation_accuracy: Some(OperationAccuracy(5.0)),
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: Some(vec![Usage {
				scope: Scope(
					"Concatendated operation scope description.".into(),
				),
				extent: Extent {
					area_description: Some(AreaDescription(
						"Concatenated operation area description.".into(),
					)),
					geographic_bounding_box: None,
					temporal_extent: None,
					vertical_extent: None,
				},
			}]),
			identifier: None,
			remark: None,
		},
	};

	let ast = parse_wkt(EXAMPLE1);
	assert_eq!(ast.len(), 1);

	let acc = ConcatenatedOperation::from_nodes(&ast).unwrap();

	assert_eq!(correct, acc.result);
}
