use crate::{
    ast::parse_wkt,
    base_types::{
        AngleUnit, CompoundCrs, CoordinateSystem, DatumAnchor, DynamicCrs, DynamicGeographicCrs,
        Ellipsoid, FrameEpoch, GeodeticCrs, GeodeticReferenceFrame, GeographicCrs, LengthUnit,
        Order, OrdinalDateTimeAxis, OrdinalDateTimeCoordinateSystem, ParametricCrs,
        ParametricDatum, ParametricUnit, PrimeMeridian, SpatialAxis, SpatialCoordinateSystem,
        StaticGeographicCrs, StaticVerticalCrs, TemporalCountMeasureAxis,
        TemporalCountMeasureCoordinateSystem, TemporalDatum, TimeCrs, VerticalCrs,
        VerticalReferenceFrame,
    },
    compound_types::{
        GeodeticData, ScopeExtentIdentifierRemark, SingleCrs, SpatialUnit, Unit, VerticalFrameDatum,
    },
    enumerations::{
        AxisDirection, Dimension, OrdinalDateTimeCsType, SpatialCsType, TemporalCountMeasureCsType,
    },
    types::WktBaseType,
};

const EXAMPLE1: &str = r#"COMPOUNDCRS["NAD83 + NAVD88",
    GEOGCRS["NAD83",
        DATUM["North American Datum 1983",
            ELLIPSOID["GRS 1980",6378137,298.25,
                LENGTHUNIT["metre",1.0]]
        ],
            PRIMEMERIDIAN["Greenwhich",0],
        CS[ellipsoidal,2],
            AXIS["latitude",north,ORDER[1]],
            AXIS["longitude",east,ORDER[2]],
            ANGLEUNIT["degree",0.017]
    ],
    VERTCRS["NAVD88",
        VDATUM["North American Vertical Datum 1983"],
        CS[vertical,1],
            AXIS["gravity-related height (H)",up],
            LENGTHUNIT["metre",1]
    ]
]
"#;

const EXAMPLE2: &str = r#"COMPOUNDCRS["ICAO layer 0",
    GEOGRAPHICCRS["WGS 84",
        DYNAMIC[FRAMEEPOCH[2005]],
        DATUM["World Geodetic System 1984",
            ELLIPSOID["WGS 84",6378137,298.25,
                LENGTHUNIT["metre",1.0]]],
        CS[ellipsoidal,2],
            AXIS["latitude",north,ORDER[1]],
            AXIS["longitude",east,ORDER[2]],
            ANGLEUNIT["degree",0.017]
    ],
    PARAMETRICCRS["WMO standard atmosphere",
        PARAMETRICDATUM["Mean Sea Level",
            ANCHOR["Mean Sea Level = 1013.25 hPa"]],
        CS[parametric,1],
            AXIS["pressure (P)",unspecified],
            PARAMETRICUNIT["HectoPascal",100]
    ]
]
"#;

const EXAMPLE3: &str = r#"COMPOUNDCRS["2D GPS position with civil time in ISO 8601 format",
    GEOGCRS["WGS 84 (G1762)",
        DYNAMIC[FRAMEEPOCH[2005]],
        DATUM["World Geodetic System 1984 (G1762)",
            ELLIPSOID["WGS 84",6378137,298.25]],
        CS[ellipsoidal,2],
            AXIS["(lat)",north,ORDER[1]],
            AXIS["(lon)",east,ORDER[2]],
            ANGLEUNIT["degree",0.017]
    ],
    TIMECRS["DateTime",
        TDATUM["Gregorian Calendar"],
    CS[TemporalDateTime,1],AXIS["Time (T)",future]
    ]
]
"#;

#[test]
fn test_compound_crs() {
    test_example_1();
    test_example_2();
    test_example_3();
}

fn test_example_1() {
    let correct = CompoundCrs {
        compound_crs_name: "NAD83 + NAVD88".into(),
        crs_one: SingleCrs::GeodeticCrs(GeodeticCrs::GeographicCrs(
            GeographicCrs::StaticGeographicCrs(StaticGeographicCrs {
                crs_name: "NAD83".into(),
                frame: GeodeticData::GeodeticReferenceFrame(GeodeticReferenceFrame {
                    datum_name: "North American Datum 1983".into(),
                    ellipsoid: Ellipsoid {
                        ellipsoid_name: "GRS 1980".into(),
                        semi_major_axis: 6378137.0,
                        inverse_flattening: 298.25,
                        length_unit: Some(LengthUnit {
                            conversion_factor: 1.0,
                            unit_name: "metre".into(),
                        }),
                    },
                    anchor: None,
                    identifier: None,
                    prime_meridian: Some(PrimeMeridian {
                        prime_meridian_name: "Greenwhich".into(),
                        irm_longitude: 0.0,
                        angle_unit: None,
                        identifier: None,
                    }),
                }),
                coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
                    spatial_cs_type: SpatialCsType::Ellipsoidal,
                    dimension: Dimension::Two,
                    identifier: None,
                    spatial_axis: vec![
                        SpatialAxis {
                            axis_name_abbreviation: "latitude".into(),
                            axis_direction: AxisDirection::North(None),
                            axis_order: Some(Order(1)),
                            spatial_unit: None,
                            identifier: None,
                        },
                        SpatialAxis {
                            axis_name_abbreviation: "longitude".into(),
                            axis_direction: AxisDirection::East,
                            axis_order: Some(Order(2)),
                            spatial_unit: None,
                            identifier: None,
                        },
                    ],
                    cs_unit: Some(Unit::SpatialUnit(SpatialUnit::AngleUnit(AngleUnit {
                        unit_name: "degree".into(),
                        conversion_factor: 0.017,
                        identifier: None,
                    }))),
                }),
                scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
                    usage: None,
                    identifier: None,
                    remark: None,
                },
            }),
        )),
        crs_two: SingleCrs::VerticalCrs(VerticalCrs::StaticVerticalCrs(StaticVerticalCrs {
            crs_name: "NAVD88".into(),
            vertical_frame_datum: VerticalFrameDatum::VerticalReferenceFrame(
                VerticalReferenceFrame {
                    datum_name: "North American Vertical Datum 1983".into(),
                    datum_anchor: None,
                    identifier: None,
                },
            ),
            coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
                spatial_cs_type: SpatialCsType::Vertical,
                dimension: Dimension::One,
                identifier: None,
                spatial_axis: vec![SpatialAxis {
                    axis_name_abbreviation: "gravity-related height (H)".into(),
                    axis_direction: AxisDirection::Up,
                    axis_order: None,
                    identifier: None,
                    spatial_unit: None,
                }],
                cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                    unit_name: "metre".into(),
                    conversion_factor: 1.0,
                }))),
            }),
            geoid_model_id: None,
            scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
                usage: None,
                identifier: None,
                remark: None,
            },
        })),
        additional_crs: None,
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: None,
            identifier: None,
            remark: None,
        },
    };

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let acc = CompoundCrs::from_nodes(&ast).unwrap();

    assert_eq!(correct, acc.result);
}

fn test_example_2() {
    let correct = CompoundCrs {
        compound_crs_name: "ICAO layer 0".into(),
        crs_one: SingleCrs::GeodeticCrs(GeodeticCrs::GeographicCrs(
            GeographicCrs::DynamicGeographicCrs(DynamicGeographicCrs {
                crs_name: "WGS 84".into(),
                dynamic_crs: DynamicCrs {
                    frame_reference_epoch: FrameEpoch(2005.0),
                    deformation_model_id: None,
                },
                geodetic_reference_frame: GeodeticReferenceFrame {
                    datum_name: "World Geodetic System 1984".into(),
                    ellipsoid: Ellipsoid {
                        ellipsoid_name: "WGS 84".into(),
                        semi_major_axis: 6378137.0,
                        inverse_flattening: 298.25,
                        length_unit: Some(LengthUnit {
                            unit_name: "metre".into(),
                            conversion_factor: 1.0,
                        }),
                    },
                    anchor: None,
                    identifier: None,
                    prime_meridian: None,
                },
                coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
                    spatial_cs_type: SpatialCsType::Ellipsoidal,
                    dimension: Dimension::Two,
                    identifier: None,
                    spatial_axis: vec![
                        SpatialAxis {
                            axis_name_abbreviation: "latitude".into(),
                            axis_direction: AxisDirection::North(None),
                            axis_order: Some(Order(1)),
                            spatial_unit: None,
                            identifier: None,
                        },
                        SpatialAxis {
                            axis_name_abbreviation: "longitude".into(),
                            axis_direction: AxisDirection::East,
                            axis_order: Some(Order(2)),
                            spatial_unit: None,
                            identifier: None,
                        },
                    ],
                    cs_unit: Some(Unit::SpatialUnit(SpatialUnit::AngleUnit(AngleUnit {
                        unit_name: "degree".into(),
                        conversion_factor: 0.017,
                        identifier: None,
                    }))),
                }),
                scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
                    usage: None,
                    identifier: None,
                    remark: None,
                },
            }),
        )),
        crs_two: SingleCrs::ParametricCrs(ParametricCrs {
            crs_name: "WMO standard atmosphere".into(),
            parametric_datum: ParametricDatum {
                datum_name: "Mean Sea Level".into(),
                datum_anchor: Some(DatumAnchor("Mean Sea Level = 1013.25 hPa".into())),
                identifier: None,
            },
            coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
                spatial_cs_type: SpatialCsType::Parametric,
                dimension: Dimension::One,
                identifier: None,
                spatial_axis: vec![SpatialAxis {
                    axis_name_abbreviation: "pressure (P)".into(),
                    axis_direction: AxisDirection::Unspecified,
                    axis_order: None,
                    spatial_unit: None,
                    identifier: None,
                }],
                cs_unit: Some(Unit::SpatialUnit(SpatialUnit::ParametricUnit(
                    ParametricUnit {
                        unit_name: "HectoPascal".into(),
                        conversion_factor: 100.0,
                        identifier: None,
                    },
                ))),
            }),
            scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
                usage: None,
                identifier: None,
                remark: None,
            },
        }),
        additional_crs: None,
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: None,
            identifier: None,
            remark: None,
        },
    };

    let ast = parse_wkt(EXAMPLE2);

    assert_eq!(ast.len(), 1);

    let acc = CompoundCrs::from_nodes(&ast).unwrap();

    assert_eq!(correct, acc.result);
}

fn test_example_3() {
    let correct = CompoundCrs {
        compound_crs_name: "2D GPS position with civil time in ISO 8601 format".into(),
        crs_one: SingleCrs::GeodeticCrs(GeodeticCrs::GeographicCrs(
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
                        length_unit: None,
                    },
                    anchor: None,
                    identifier: None,
                    prime_meridian: None,
                },
                coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
                    spatial_cs_type: SpatialCsType::Ellipsoidal,
                    dimension: Dimension::Two,
                    identifier: None,
                    spatial_axis: vec![
                        SpatialAxis {
                            axis_name_abbreviation: "(lat)".into(),
                            axis_direction: AxisDirection::North(None),
                            axis_order: Some(Order(1)),
                            spatial_unit: None,
                            identifier: None,
                        },
                        SpatialAxis {
                            axis_name_abbreviation: "(lon)".into(),
                            axis_direction: AxisDirection::East,
                            axis_order: Some(Order(2)),
                            spatial_unit: None,
                            identifier: None,
                        },
                    ],
                    cs_unit: Some(Unit::SpatialUnit(SpatialUnit::AngleUnit(AngleUnit {
                        unit_name: "degree".into(),
                        conversion_factor: 0.017,
                        identifier: None,
                    }))),
                }),
                scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
                    usage: None,
                    identifier: None,
                    remark: None,
                },
            }),
        )),
        crs_two: SingleCrs::TimeCrs(TimeCrs {
            crs_name: "DateTime".into(),
            temporal_datum: TemporalDatum {
                datum_name: "Gregorian Calendar".into(),
                calendar: None,
                temporal_origin: None,
                identifier: None,
            },
            coordinate_system: CoordinateSystem::OrdinalDateTimeCS(
                OrdinalDateTimeCoordinateSystem {
                    ordinal_date_time_cs_type: OrdinalDateTimeCsType::TemporalDateTime,
                    dimension: Dimension::One,
                    identifier: None,
                    ordinal_date_time_axis: vec![OrdinalDateTimeAxis {
                        axis_name_abbreviation: "Time (T)".into(),
                        axis_direction: AxisDirection::Future,
                        axis_order: None,

                        identifier: None,
                    }],
                },
            ),
            scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
                usage: None,
                identifier: None,
                remark: None,
            },
        }),
        additional_crs: None,
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: None,
            identifier: None,
            remark: None,
        },
    };

    let ast = parse_wkt(EXAMPLE3);

    assert_eq!(ast.len(), 1);

    let acc = CompoundCrs::from_nodes(&ast).unwrap();

    assert_eq!(correct, acc.result);
}
