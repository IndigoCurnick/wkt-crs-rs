use horologium::Temporal;
use time::{Date, Month};

use crate::{
	ast::parse_wkt,
	base_types::{
		AngleUnit, AreaDescription, Axis, CoordinateSystem, DynamicCrs,
		DynamicGeographicCrs, Ellipsoid, Extent, FrameEpoch,
		GeodeticReferenceFrame, GeographicBoundingBox, GeographicCrs, Id,
		LengthUnit, Order, PrimeMeridian, Remark, Scope,
		SpatialCoordinateSystem, StaticGeographicCrs, TemporalExtent, Uri,
		Usage, geodetic_crs::static_geodetic_crs::StaticGeodeticCrs,
	},
	compound_types::{
		GeodeticData, ScopeExtentIdentifierRemark, SpatialUnit, Unit,
	},
	data_types::{DateOrString, NumText},
	enumerations::{AxisDirection, Dimension, SpatialCsType},
	types::WktBaseType,
};

use super::GeodeticCrs;

const EXAMPLE1: &str = r#"GEODCRS["JGD2000",
    DATUM["Japanese Geodetic Datum 2000",
        ELLIPSOID["GRS 1980",6378137,298.25]
    ],
    CS[Cartesian,3],
        AXIS["(X)",geocentricX],
        AXIS["(Y)",geocentricY],
        AXIS["(Z)",geocentricZ],
        LENGTHUNIT["metre",1.0],
    USAGE[SCOPE["Geodesy, topographic mapping and cadastre"],
        AREA["Japan"],
        BBOX[17.09,122.38,46.05,157.64],
        TIMEEXTENT[2002-04-01,2011-10-21]
    ],
    ID["EPSG",4946,URI["urn:ogc:def:crs:EPSG:4946"]],
    REMARK["Some remark"]
]
"#;

const EXAMPLE2: &str = r#"GEOGCRS["WGS 84 (G1762)",
    DYNAMIC[FRAMEEPOCH[2005.0]],
    TRF["World Geodetic System 1984 (G1762)",
        ELLIPSOID["WGS 84",6378137,298.25,LENGTHUNIT["metre",1.0]
        ]
    ],
    CS[ellipsoidal,3],
        AXIS["(lat)",north,ANGLEUNIT["degree",0.017]],
        AXIS["(lon)",east,ANGLEUNIT["degree",0.017]],
        AXIS["ellipsoidal height (h)",up,LENGTHUNIT["metre",1.0]]
]
"#;

const EXAMPLE3: &str = r#"GEOGRAPHICCRS["NAD83",
    DATUM["North American Datum 1983",
        ELLIPSOID["GRS 1980",6378137,298.257,LENGTHUNIT["metre",1.0]]
    ],
    CS[ellipsoidal,2],
        AXIS["latitude",north],
        AXIS["longitude",east],
        ANGLEUNIT["degree",0.017],
    ID["EPSG",4269],
    REMARK["1986 realisation"]
]
"#;

const EXAMPLE4: &str = r#"GEOGCRS["NTF (Paris)",
    DATUM["Nouvelle Triangulation Francaise",
        ELLIPSOID["Clarke 1880 (IGN)",6378249.2,293.466]
    ],
    PRIMEM["Paris",2.5969],
    CS[ellipsoidal,2],
        AXIS["latitude",north,ORDER[1]],
        AXIS["longitude",east,ORDER[2]],
        ANGLEUNIT["grad",0.0157],
    REMARK["Nouvelle Triangulation Française"]
]
"#;

#[test]
fn test_geodetic_crs() {
	test_example_1();
	test_example_2();
	test_example_3();
	test_example_4()
}

fn test_example_1() {
	let correct = GeodeticCrs::StaticGeodeticCrs(StaticGeodeticCrs {
		crs_name: "JGD2000".into(),
		frame: GeodeticData::GeodeticReferenceFrame(GeodeticReferenceFrame {
			datum_name: "Japanese Geodetic Datum 2000".into(),
			ellipsoid: Ellipsoid {
				ellipsoid_name: "GRS 1980".into(),
				semi_major_axis: 6378137.0,
				inverse_flattening: 298.25,
				length_unit: None,
			},
			anchor: None,
			identifier: None,
			prime_meridian: None,
		}),
		coordinate_system: CoordinateSystem::SpatialCS(
			SpatialCoordinateSystem {
				spatial_cs_type: SpatialCsType::Cartesian,
				dimension: Dimension::Three,
				identifier: None,
				spatial_axis: vec![
					Axis {
						axis_name_abbreviation: "(X)".into(),
						axis_direction: AxisDirection::GeocentricX,
						axis_order: None,
						unit: None,
						identifier: None,
					},
					Axis {
						axis_name_abbreviation: "(Y)".into(),
						axis_direction: AxisDirection::GeocentricY,
						axis_order: None,
						unit: None,
						identifier: None,
					},
					Axis {
						axis_name_abbreviation: "(Z)".into(),
						axis_direction: AxisDirection::GeocentricZ,
						axis_order: None,
						unit: None,
						identifier: None,
					},
				],
				cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(
					LengthUnit {
						unit_name: "metre".into(),
						conversion_factor: 1.0,
						identifier: None,
					},
				))),
			},
		),
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: Some(vec![Usage {
				scope: Scope(
					"Geodesy, topographic mapping and cadastre".into(),
				),
				extent: Extent {
					area_description: Some(AreaDescription("Japan".into())),
					geographic_bounding_box: Some(GeographicBoundingBox {
						lower_left_latitude: 17.09,
						lower_left_longitude: 122.38,
						upper_right_latitude: 46.05,
						upper_right_longitude: 157.64,
					}),
					vertical_extent: None,
					temporal_extent: Some(TemporalExtent {
						from: DateOrString::Date(Temporal::CalendarDay(
							Date::from_calendar_date(2002, Month::April, 1)
								.unwrap(),
						)),
						to: DateOrString::Date(Temporal::CalendarDay(
							Date::from_calendar_date(2011, Month::October, 21)
								.unwrap(),
						)),
					}),
				},
			}]),
			identifier: Some(vec![Id {
				authority_name: "EPSG".into(),
				authority_unique_identifier: NumText::Int(4946),
				version: None,
				authority_citation: None,
				id_uri: Some(Uri("urn:ogc:def:crs:EPSG:4946".into())),
			}]),
			remark: Some(Remark("Some remark".into())),
		},
	});

	let ast = parse_wkt(EXAMPLE1);

	let geo = GeodeticCrs::from_nodes(&ast).unwrap();

	assert_eq!(geo.result, correct);
}

fn test_example_2() {
	let correct = GeodeticCrs::GeographicCrs(
		GeographicCrs::DynamicGeographicCrs(DynamicGeographicCrs {
			crs_name: "WGS 84 (G1762)".into(),
			dynamic_crs: DynamicCrs {
				frame_reference_epoch: FrameEpoch(2005.0),
				deformation_model_id: None,
			},
			geodetic_reference_frame: GeodeticReferenceFrame {
				datum_name: "World Geodetic System 1984 (G1762)".into(),
				ellipsoid: Ellipsoid {
					ellipsoid_name: "WGS 84".into(),
					semi_major_axis: 6378137.0,
					inverse_flattening: 298.25,
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
			coordinate_system: CoordinateSystem::SpatialCS(
				SpatialCoordinateSystem {
					spatial_cs_type: SpatialCsType::Ellipsoidal,
					dimension: Dimension::Three,
					identifier: None,
					spatial_axis: vec![
						Axis {
							axis_name_abbreviation: "(lat)".into(),
							axis_direction: AxisDirection::North(None),
							axis_order: None,
							unit: Some(Unit::SpatialUnit(
								SpatialUnit::AngleUnit(AngleUnit {
									unit_name: "degree".into(),
									conversion_factor: 0.017,
									identifier: None,
								}),
							)),
							identifier: None,
						},
						Axis {
							axis_name_abbreviation: "(lon)".into(),
							axis_direction: AxisDirection::East,
							axis_order: None,
							unit: Some(Unit::SpatialUnit(
								SpatialUnit::AngleUnit(AngleUnit {
									unit_name: "degree".into(),
									conversion_factor: 0.017,
									identifier: None,
								}),
							)),
							identifier: None,
						},
						Axis {
							axis_name_abbreviation: "ellipsoidal height (h)"
								.into(),
							axis_direction: AxisDirection::Up,
							axis_order: None,
							unit: Some(Unit::SpatialUnit(
								SpatialUnit::LengthUnit(LengthUnit {
									unit_name: "metre".into(),
									conversion_factor: 1.0,
									identifier: None,
								}),
							)),
							identifier: None,
						},
					],
					cs_unit: None,
				},
			),
			scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
				usage: None,
				identifier: None,
				remark: None,
			},
		}),
	);

	let ast = parse_wkt(EXAMPLE2);

	let geo = GeodeticCrs::from_nodes(&ast).unwrap();

	assert_eq!(geo.result, correct);
}

fn test_example_3() {
	let correct = GeodeticCrs::GeographicCrs(
		GeographicCrs::StaticGeographicCrs(StaticGeographicCrs {
			crs_name: "NAD83".into(),
			frame: GeodeticData::GeodeticReferenceFrame(
				GeodeticReferenceFrame {
					datum_name: "North American Datum 1983".into(),
					ellipsoid: Ellipsoid {
						ellipsoid_name: "GRS 1980".into(),
						semi_major_axis: 6378137.0,
						inverse_flattening: 298.257,
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
					spatial_cs_type: SpatialCsType::Ellipsoidal,
					dimension: Dimension::Two,
					identifier: None,
					spatial_axis: vec![
						Axis {
							axis_name_abbreviation: "latitude".into(),
							axis_direction: AxisDirection::North(None),
							axis_order: None,
							unit: None,
							identifier: None,
						},
						Axis {
							axis_name_abbreviation: "longitude".into(),
							axis_direction: AxisDirection::East,
							axis_order: None,
							unit: None,
							identifier: None,
						},
					],
					cs_unit: Some(Unit::SpatialUnit(SpatialUnit::AngleUnit(
						AngleUnit {
							unit_name: "degree".into(),
							conversion_factor: 0.017,
							identifier: None,
						},
					))),
				},
			),
			scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
				usage: None,
				identifier: Some(vec![Id {
					authority_name: "EPSG".into(),
					authority_unique_identifier: NumText::Int(4269),
					authority_citation: None,
					id_uri: None,
					version: None,
				}]),
				remark: Some(Remark("1986 realisation".into())),
			},
		}),
	);

	let ast = parse_wkt(EXAMPLE3);

	let geo = GeodeticCrs::from_nodes(&ast).unwrap();

	assert_eq!(geo.result, correct);
}

fn test_example_4() {
	let correct = GeodeticCrs::GeographicCrs(
		GeographicCrs::StaticGeographicCrs(StaticGeographicCrs {
			crs_name: "NTF (Paris)".into(),
			frame: GeodeticData::GeodeticReferenceFrame(
				GeodeticReferenceFrame {
					datum_name: "Nouvelle Triangulation Francaise".into(),
					ellipsoid: Ellipsoid {
						ellipsoid_name: "Clarke 1880 (IGN)".into(),
						semi_major_axis: 6378249.2,
						inverse_flattening: 293.466,
						length_unit: None,
					},
					anchor: None,
					identifier: None,
					prime_meridian: Some(PrimeMeridian {
						prime_meridian_name: "Paris".into(),
						irm_longitude: 2.5969,
						angle_unit: None,
						identifier: None,
					}),
				},
			),
			coordinate_system: CoordinateSystem::SpatialCS(
				SpatialCoordinateSystem {
					spatial_cs_type: SpatialCsType::Ellipsoidal,
					dimension: Dimension::Two,
					identifier: None,
					spatial_axis: vec![
						Axis {
							axis_name_abbreviation: "latitude".into(),
							axis_direction: AxisDirection::North(None),
							axis_order: Some(Order(1)),
							unit: None,
							identifier: None,
						},
						Axis {
							axis_name_abbreviation: "longitude".into(),
							axis_direction: AxisDirection::East,
							axis_order: Some(Order(2)),
							unit: None,
							identifier: None,
						},
					],
					cs_unit: Some(Unit::SpatialUnit(SpatialUnit::AngleUnit(
						AngleUnit {
							unit_name: "grad".into(),
							conversion_factor: 0.0157,
							identifier: None,
						},
					))),
				},
			),
			scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
				usage: None,
				identifier: None,
				remark: Some(Remark("Nouvelle Triangulation Française".into())),
			},
		}),
	);

	let ast = parse_wkt(EXAMPLE4);

	let geo = GeodeticCrs::from_nodes(&ast).unwrap();

	assert_eq!(geo.result, correct);
}
