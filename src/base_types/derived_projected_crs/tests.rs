use crate::{
	ast::parse_wkt,
	base_types::{
		AngleUnit, Axis, BaseGeodeticCrs, BaseProjectedCrs,
		BaseStaticGeographicCrs, CoordinateSystem, DerivingConversion,
		Ellipsoid, GeodeticReferenceFrame, Id, LengthUnit, MapProjection,
		Method, OrdinalDateTimeCoordinateSystem, Parameter, ScaleUnit,
		derived_projected_crs::DerivedProjectedCrs,
	},
	compound_types::{
		GeodeticData, ScopeExtentIdentifierRemark, SpatialUnit, Unit,
	},
	data_types::NumText,
	enumerations::{
		AxisDirection, Dimension, OperationParameterWrapper,
		OrdinalDateTimeCsType,
	},
	types::WktBaseType,
};

const EXAMPLE1: &str = r#"DERIVEDPROJCRS["Gulf of Mexico",
    BASEPROJCRS["NAD27 / Texas South Central",
        BASEGEOGCRS["NAD27",
            DATUM["North American Datum 1927",
                ELLIPSOID["Clarke 1866",20925832.164,294.978,
                    LENGTHUNIT["US survey foot",0.304]
                ]
            ]
        ],
        CONVERSION["Texas South Central",
            METHOD["Lambert Conic Conformal",ID["EPSG",9802]],
            PARAMETER["Latitude of false origin",27.83,
                ANGLEUNIT["degree",0.017],ID["EPSG",8821]],
            ]
        ],
        DERIVINGCONVERSION["Gulf of Mexico",
            METHOD["P6",ID["EPSG",1049]],
            PARAMETER["Bin grid origin",5000,SCALEUNIT["Bin",1.0],ID["EPSG",8733]],
        ],
        CS[ordinal,2],
            AXIS["Inline (I)",northNorthWest],
            AXIS["Crossline (J)",westSouthWest]
        ]
"#;

#[test]
fn test_derived_projected_crs() {
	let correct = DerivedProjectedCrs {
		derived_crs_name: "Gulf of Mexico".to_string(),
		base_projected_crs: BaseProjectedCrs {
			base_crs_name: "NAD27 / Texas South Central".to_string(),
			base_geodetic_crs: BaseGeodeticCrs::BaseStaticGeographicCrs(
				BaseStaticGeographicCrs {
					base_crs_name: "NAD27".to_string(),
					geodetic_data: GeodeticData::GeodeticReferenceFrame(
						GeodeticReferenceFrame {
							datum_name: "North American Datum 1927".to_string(),
							ellipsoid: Ellipsoid {
								ellipsoid_name: "Clarke 1866".to_string(),
								semi_major_axis: 20925832.164,
								inverse_flattening: 294.978,
								length_unit: Some(LengthUnit {
									unit_name: "US survey foot".to_string(),
									conversion_factor: 0.304,
									identifier: None,
								}),
								identifier: None,
							},
							anchor: None,
							identifier: None,
							prime_meridian: None,
						},
					),
					ellipsoidal_cs_unit: None,
					identifier: None,
				},
			),
			map_projection: MapProjection {
				map_projection_name: "Texas South Central".to_string(),
				map_projection_method: Method {
					method_name: "Lambert Conic Conformal".to_string(),
					identifier: Some(Id {
						authority_name: "EPSG".to_string(),
						authority_unique_identifier: NumText::Int(9802),
						version: None,
						authority_citation: None,
						id_uri: None,
					}),
				},
				map_projection_parameters: Some(vec![Parameter {
					parameter_name: "Latitude of false origin".to_string(),
					parameter_value: 27.83,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::AngleUnit(AngleUnit {
							unit_name: "degree".to_string(),
							conversion_factor: 0.017,
							identifier: None,
						}),
					)),
					identifier: Some(Id {
						authority_name: "EPSG".to_string(),
						authority_unique_identifier: NumText::Int(8821),
						version: None,
						authority_citation: None,
						id_uri: None,
					}),
				}]),
				identifier: None,
			},
			identifier: None,
		},
		deriving_conversion: DerivingConversion {
			deriving_conversion_name: "Gulf of Mexico".to_string(),
			operation_method: Method {
				method_name: "P6".to_string(),
				identifier: Some(Id {
					authority_name: "EPSG".to_string(),
					authority_unique_identifier: NumText::Int(1049),
					version: None,
					authority_citation: None,
					id_uri: None,
				}),
			},
			operation_parameter: Some(vec![
				OperationParameterWrapper::OperationParameter(Parameter {
					parameter_name: "Bin grid origin".to_string(),
					parameter_value: 5000.0,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::ScaleUnit(ScaleUnit {
							unit_name: "Bin".to_string(),
							conversion_factor: 1.0,
							identifier: None,
						}),
					)),
					identifier: Some(Id {
						authority_name: "EPSG".to_string(),
						authority_unique_identifier: NumText::Int(8733),
						version: None,
						authority_citation: None,
						id_uri: None,
					}),
				}),
			]),
			identifier: None,
		},
		coordinate_system: CoordinateSystem::OrdinalDateTimeCS(
			OrdinalDateTimeCoordinateSystem {
				ordinal_date_time_cs_type: OrdinalDateTimeCsType::Ordinal,
				dimension: Dimension::Two,
				identifier: None,
				ordinal_date_time_axis: vec![
					Axis {
						axis_name_abbreviation: "Inline (I)".to_string(),
						axis_direction: AxisDirection::NorthNorthWest,
						axis_order: None,
						identifier: None,
						unit: None,
					},
					Axis {
						axis_name_abbreviation: "Crossline (J)".to_string(),
						axis_direction: AxisDirection::WestSouthWest,
						axis_order: None,
						identifier: None,
						unit: None,
					},
				],
			},
		),
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			identifier: None,
			remark: None,
			usage: None,
		},
	};

	let ast = parse_wkt(EXAMPLE1);

	assert_eq!(ast.len(), 1);

	let derived = DerivedProjectedCrs::from_nodes(&ast).unwrap();

	assert_eq!(correct, derived.result);
}
