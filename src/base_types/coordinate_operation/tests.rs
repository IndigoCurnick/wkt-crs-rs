use crate::{
    ast::parse_wkt,
    base_types::{
        AngleUnit, AreaDescription, Citation, CoordinateSystem, Ellipsoid, Extent, GeodeticCrs,
        GeodeticReferenceFrame, GeographicBoundingBox, Id, InterpolationCrs, LengthUnit,
        OperationAccuracy, OperationMethod, OperationParameter, OperationParameterFile,
        OperationVersion, Order, Remark, Scope, SourceCrs, SpatialAxis, SpatialCoordinateSystem,
        StaticGeodeticCrs, TargetCrs, Uri, Usage,
        coordinate_operation::coordinate_operation::CoordinateOperation,
    },
    compound_types::{
        CoordinateReferenceSystem, GeodeticData, ScopeExtentIdentifierRemark, SingleCrs,
        SpatialUnit, Unit,
    },
    data_types::NumText,
    enumerations::{AxisDirection, Dimension, OperationParameterWrapper, SpatialCsType},
    types::WktBaseType,
};

const EXAMPLE1: &str = r#"COORDINATEOPERATION["Tokyo to JGD2000",VERSION["GSI"],
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
    METHOD["Geocentric translations",ID["EPSG",1031]],
    PARAMETER["X-axis translation",-146.414,
        LENGTHUNIT["metre",1.0],ID["EPSG",8605]],
    PARAMETER["Y-axis translation",507.337,
        LENGTHUNIT["metre",1.0],ID["EPSG",8606]],
    PARAMETER["Z-axis translation",680.507,
        LENGTHUNIT["metre",1.0],ID["EPSG",8607]]
]
"#;

const EXAMPLE2: &str = r#"COORDINATEOPERATION["AGD84 to GDA94",
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
    METHOD["Geocentric translations",ID["EPSG",1031]],
    PARAMETER["X-axis translation",-128.5,LENGTHUNIT["metre",1.0]],
    PARAMETER["Y-axis translation",-53.0,LENGTHUNIT["metre",1.0]],
    PARAMETER["Z-axis translation",153.4,LENGTHUNIT["metre",1.0]],
    OPERATIONACCURACY[5],
    USAGE[SCOPE["Low accuracy applications."],
        AREA["Australia onshore"],BBOX[-43.7,112.85,-9.87,153.68]],
    REMARK["Use NTv2 file for better accuracy"]
]
"#;

const EXAMPLE3: &str = r#"COORDINATEOPERATION["NZGD49 to NZGD2000",
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
    METHOD["NTv2",ID["EPSG",9615]],
    PARAMETERFILE["Latitude and longitude difference file","nzdg2kgrid0005.gsb"],
    ID["EPSG",1568,CITATION["LINZS25000"],
        URI["http://www.linz.govt.nz/geodetic/software-downloads/"]],
    REMARK["Coordinate transformation accuracy 0.1-1.0m"]
]
"#;

const EXAMPLE4: &str = r#"COORDINATEOPERATION["Amersfoort to ETRS98 (3)",
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
    METHOD["Coordinate Frame"],
    PARAMETER["X-axis translation",565.2369,LENGTHUNIT["metre",1.0]],
    PARAMETER["Y-axis translation",50.0087,LENGTHUNIT["metre",1.0]],
    ID["EPSG",15739]
]
"#;

const EXAMPLE5: &str = r#"COORDINATEOPERATION["DHHN92 height to EVRF2007 height",
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
    METHOD["Vertical Offset and Slope",ID["EPSG",1046]],
    PARAMETER["Inclination in latitude",-0.010,
        ANGLEUNIT["arc-second",4.84E-06]],
    PARAMETER["Inclination in longitude",0.002,
        ANGLEUNIT["arc-second",4.84E-06]],
    INTERPOLATIONCRS[
        CS[Cartesian,3],
            AXIS["(X)",geocentricX,ORDER[1]],
            AXIS["(Y)",geocentricY,ORDER[2]],
            AXIS["(Z)",geocentricZ,ORDER[3]],
            LENGTHUNIT["metre",1.0]
    ],
    OPERATIONACCURACY[0.1],
    REMARK["Determined at 427 points."]
]
"#;

#[test]
fn test_coordinate_operation() {
    test_example_1();
    test_example_2();
    test_example_3();
    test_example_4();
    test_example_5();
}

fn test_example_1() {
    let correct = CoordinateOperation {
        operation_name: "Tokyo to JGD2000".into(),
        operation_version: Some(OperationVersion("GSI".into())),
        source_crs: SourceCrs {
            coordinate_system: CoordinateReferenceSystem::SingleCrs(SingleCrs::GeodeticCrs(
                GeodeticCrs::StaticGeodeticCrs(StaticGeodeticCrs {
                    crs_name: "Tokyo".into(),
                    frame: GeodeticData::GeodeticReferenceFrame(GeodeticReferenceFrame {
                        datum_name: "Tokyo 1918".into(),
                        ellipsoid: Ellipsoid {
                            ellipsoid_name: "Bessel 1841".into(),
                            semi_major_axis: 6377397.155,
                            inverse_flattening: 299.1528128,
                            length_unit: Some(LengthUnit {
                                unit_name: "metre".into(),
                                conversion_factor: 1.0,
                            }),
                        },
                        anchor: None,
                        identifier: None,
                        prime_meridian: None,
                    }),
                    coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
                        spatial_cs_type: SpatialCsType::Cartesian,
                        dimension: Dimension::Three,
                        identifier: None,
                        spatial_axis: vec![
                            SpatialAxis {
                                axis_name_abbreviation: "(X)".into(),
                                axis_direction: AxisDirection::GeocentricX,
                                axis_order: Some(Order(1)),
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Y)".into(),
                                axis_direction: AxisDirection::GeocentricY,
                                axis_order: Some(Order(2)),
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Z)".into(),
                                axis_direction: AxisDirection::GeocentricZ,
                                axis_order: Some(Order(3)),
                                spatial_unit: None,
                                identifier: None,
                            },
                        ],
                        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                            unit_name: "metre".into(),
                            conversion_factor: 1.0,
                        }))),
                    }),
                    scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
                        identifier: None,
                        remark: None,
                        usage: None,
                    },
                }),
            )),
        },
        target_crs: TargetCrs {
            coordinate_system: CoordinateReferenceSystem::SingleCrs(SingleCrs::GeodeticCrs(
                GeodeticCrs::StaticGeodeticCrs(StaticGeodeticCrs {
                    crs_name: "JGD2000".into(),
                    frame: GeodeticData::GeodeticReferenceFrame(GeodeticReferenceFrame {
                        datum_name: "Japanese Geodetic Datum 2000".into(),
                        ellipsoid: Ellipsoid {
                            ellipsoid_name: "GRS 1980".into(),
                            semi_major_axis: 6378137.0,
                            inverse_flattening: 298.257222101,
                            length_unit: Some(LengthUnit {
                                unit_name: "metre".into(),
                                conversion_factor: 1.0,
                            }),
                        },
                        anchor: None,
                        identifier: None,
                        prime_meridian: None,
                    }),
                    coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
                        spatial_cs_type: SpatialCsType::Cartesian,
                        dimension: Dimension::Three,
                        identifier: None,
                        spatial_axis: vec![
                            SpatialAxis {
                                axis_name_abbreviation: "(X)".into(),
                                axis_direction: AxisDirection::GeocentricX,
                                axis_order: None,
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Y)".into(),
                                axis_direction: AxisDirection::GeocentricY,
                                axis_order: None,
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Z)".into(),
                                axis_direction: AxisDirection::GeocentricZ,
                                axis_order: None,
                                spatial_unit: None,
                                identifier: None,
                            },
                        ],
                        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                            unit_name: "metre".into(),
                            conversion_factor: 1.0,
                        }))),
                    }),
                    scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
                        usage: None,
                        identifier: None,
                        remark: None,
                    },
                }),
            )),
        },
        operation_method: OperationMethod {
            operation_method_name: "Geocentric translations".into(),
            identifier: Some(Id {
                authority_name: "EPSG".into(),
                authority_unique_identifier: NumText::Int(1031),
                version: None,
                authority_citation: None,
                id_uri: None,
            }),
        },
        operation_parameter_wrapper: Some(vec![
            OperationParameterWrapper::OperationParameter(OperationParameter {
                parameter_name: "X-axis translation".into(),
                parameter_value: -146.414,
                parameter_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                    unit_name: "metre".into(),
                    conversion_factor: 1.0,
                }))),
                identifier: Some(Id {
                    authority_name: "EPSG".into(),
                    authority_unique_identifier: NumText::Int(8605),
                    version: None,
                    authority_citation: None,
                    id_uri: None,
                }),
            }),
            OperationParameterWrapper::OperationParameter(OperationParameter {
                parameter_name: "Y-axis translation".into(),
                parameter_value: 507.337,
                parameter_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                    unit_name: "metre".into(),
                    conversion_factor: 1.0,
                }))),
                identifier: Some(Id {
                    authority_name: "EPSG".into(),
                    authority_unique_identifier: NumText::Int(8606),
                    version: None,
                    authority_citation: None,
                    id_uri: None,
                }),
            }),
            OperationParameterWrapper::OperationParameter(OperationParameter {
                parameter_name: "Z-axis translation".into(),
                parameter_value: 680.507,
                parameter_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                    unit_name: "metre".into(),
                    conversion_factor: 1.0,
                }))),
                identifier: Some(Id {
                    authority_name: "EPSG".into(),
                    authority_unique_identifier: NumText::Int(8607),
                    version: None,
                    authority_citation: None,
                    id_uri: None,
                }),
            }),
        ]),
        interpolation_crs: None,
        operation_accuracy: None,
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: None,
            identifier: None,
            remark: None,
        },
    };

    let ast = parse_wkt(EXAMPLE1);
    assert_eq!(ast.len(), 1);

    let acc = CoordinateOperation::from_nodes(&ast).unwrap();

    assert_eq!(correct, acc.result);
}

fn test_example_2() {
    let correct = CoordinateOperation {
        operation_name: "AGD84 to GDA94".into(),
        operation_version: None,
        source_crs: SourceCrs {
            coordinate_system: CoordinateReferenceSystem::SingleCrs(SingleCrs::GeodeticCrs(
                GeodeticCrs::StaticGeodeticCrs(StaticGeodeticCrs {
                    crs_name: "Tokyo".into(),
                    frame: GeodeticData::GeodeticReferenceFrame(GeodeticReferenceFrame {
                        datum_name: "Tokyo 1918".into(),
                        ellipsoid: Ellipsoid {
                            ellipsoid_name: "Bessel 1841".into(),
                            semi_major_axis: 6377397.155,
                            inverse_flattening: 299.1528128,
                            length_unit: Some(LengthUnit {
                                unit_name: "metre".into(),
                                conversion_factor: 1.0,
                            }),
                        },
                        anchor: None,
                        identifier: None,
                        prime_meridian: None,
                    }),
                    coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
                        spatial_cs_type: SpatialCsType::Cartesian,
                        dimension: Dimension::Three,
                        identifier: None,
                        spatial_axis: vec![
                            SpatialAxis {
                                axis_name_abbreviation: "(X)".into(),
                                axis_direction: AxisDirection::GeocentricX,
                                axis_order: Some(Order(1)),
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Y)".into(),
                                axis_direction: AxisDirection::GeocentricY,
                                axis_order: Some(Order(2)),
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Z)".into(),
                                axis_direction: AxisDirection::GeocentricZ,
                                axis_order: Some(Order(3)),
                                spatial_unit: None,
                                identifier: None,
                            },
                        ],
                        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                            unit_name: "metre".into(),
                            conversion_factor: 1.0,
                        }))),
                    }),
                    scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
                        identifier: None,
                        remark: None,
                        usage: None,
                    },
                }),
            )),
        },
        target_crs: TargetCrs {
            coordinate_system: CoordinateReferenceSystem::SingleCrs(SingleCrs::GeodeticCrs(
                GeodeticCrs::StaticGeodeticCrs(StaticGeodeticCrs {
                    crs_name: "JGD2000".into(),
                    frame: GeodeticData::GeodeticReferenceFrame(GeodeticReferenceFrame {
                        datum_name: "Japanese Geodetic Datum 2000".into(),
                        ellipsoid: Ellipsoid {
                            ellipsoid_name: "GRS 1980".into(),
                            semi_major_axis: 6378137.0,
                            inverse_flattening: 298.257222101,
                            length_unit: Some(LengthUnit {
                                unit_name: "metre".into(),
                                conversion_factor: 1.0,
                            }),
                        },
                        anchor: None,
                        identifier: None,
                        prime_meridian: None,
                    }),
                    coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
                        spatial_cs_type: SpatialCsType::Cartesian,
                        dimension: Dimension::Three,
                        identifier: None,
                        spatial_axis: vec![
                            SpatialAxis {
                                axis_name_abbreviation: "(X)".into(),
                                axis_direction: AxisDirection::GeocentricX,
                                axis_order: None,
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Y)".into(),
                                axis_direction: AxisDirection::GeocentricY,
                                axis_order: None,
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Z)".into(),
                                axis_direction: AxisDirection::GeocentricZ,
                                axis_order: None,
                                spatial_unit: None,
                                identifier: None,
                            },
                        ],
                        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                            unit_name: "metre".into(),
                            conversion_factor: 1.0,
                        }))),
                    }),
                    scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
                        usage: None,
                        identifier: None,
                        remark: None,
                    },
                }),
            )),
        },
        operation_method: OperationMethod {
            operation_method_name: "Geocentric translations".into(),
            identifier: Some(Id {
                authority_name: "EPSG".into(),
                authority_unique_identifier: NumText::Int(1031),
                version: None,
                authority_citation: None,
                id_uri: None,
            }),
        },
        operation_parameter_wrapper: Some(vec![
            OperationParameterWrapper::OperationParameter(OperationParameter {
                parameter_name: "X-axis translation".into(),
                parameter_value: -128.5,
                parameter_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                    unit_name: "metre".into(),
                    conversion_factor: 1.0,
                }))),
                identifier: None,
            }),
            OperationParameterWrapper::OperationParameter(OperationParameter {
                parameter_name: "Y-axis translation".into(),
                parameter_value: -53.0,
                parameter_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                    unit_name: "metre".into(),
                    conversion_factor: 1.0,
                }))),
                identifier: None,
            }),
            OperationParameterWrapper::OperationParameter(OperationParameter {
                parameter_name: "Z-axis translation".into(),
                parameter_value: 153.4,
                parameter_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                    unit_name: "metre".into(),
                    conversion_factor: 1.0,
                }))),
                identifier: None,
            }),
        ]),
        interpolation_crs: None,
        operation_accuracy: Some(OperationAccuracy(5.0)),
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: Some(vec![Usage {
                scope: Scope("Low accuracy applications.".into()),
                extent: Extent {
                    area_description: Some(AreaDescription("Australia onshore".into())),
                    geographic_bounding_box: Some(GeographicBoundingBox {
                        lower_left_latitude: -43.7,
                        lower_left_longitude: 112.85,
                        upper_right_latitude: -9.87,
                        upper_right_longitude: 153.68,
                    }),
                    temporal_extent: None,
                    vertical_extent: None,
                },
            }]),
            identifier: None,
            remark: Some(Remark("Use NTv2 file for better accuracy".into())),
        },
    };

    let ast = parse_wkt(EXAMPLE2);
    assert_eq!(ast.len(), 1);

    let acc = CoordinateOperation::from_nodes(&ast).unwrap();

    assert_eq!(correct, acc.result);
}

fn test_example_3() {
    let correct = CoordinateOperation {
        operation_name: "NZGD49 to NZGD2000".into(),
        operation_version: None,
        source_crs: SourceCrs {
            coordinate_system: CoordinateReferenceSystem::SingleCrs(SingleCrs::GeodeticCrs(
                GeodeticCrs::StaticGeodeticCrs(StaticGeodeticCrs {
                    crs_name: "Tokyo".into(),
                    frame: GeodeticData::GeodeticReferenceFrame(GeodeticReferenceFrame {
                        datum_name: "Tokyo 1918".into(),
                        ellipsoid: Ellipsoid {
                            ellipsoid_name: "Bessel 1841".into(),
                            semi_major_axis: 6377397.155,
                            inverse_flattening: 299.1528128,
                            length_unit: Some(LengthUnit {
                                unit_name: "metre".into(),
                                conversion_factor: 1.0,
                            }),
                        },
                        anchor: None,
                        identifier: None,
                        prime_meridian: None,
                    }),
                    coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
                        spatial_cs_type: SpatialCsType::Cartesian,
                        dimension: Dimension::Three,
                        identifier: None,
                        spatial_axis: vec![
                            SpatialAxis {
                                axis_name_abbreviation: "(X)".into(),
                                axis_direction: AxisDirection::GeocentricX,
                                axis_order: Some(Order(1)),
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Y)".into(),
                                axis_direction: AxisDirection::GeocentricY,
                                axis_order: Some(Order(2)),
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Z)".into(),
                                axis_direction: AxisDirection::GeocentricZ,
                                axis_order: Some(Order(3)),
                                spatial_unit: None,
                                identifier: None,
                            },
                        ],
                        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                            unit_name: "metre".into(),
                            conversion_factor: 1.0,
                        }))),
                    }),
                    scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
                        identifier: None,
                        remark: None,
                        usage: None,
                    },
                }),
            )),
        },
        target_crs: TargetCrs {
            coordinate_system: CoordinateReferenceSystem::SingleCrs(SingleCrs::GeodeticCrs(
                GeodeticCrs::StaticGeodeticCrs(StaticGeodeticCrs {
                    crs_name: "JGD2000".into(),
                    frame: GeodeticData::GeodeticReferenceFrame(GeodeticReferenceFrame {
                        datum_name: "Japanese Geodetic Datum 2000".into(),
                        ellipsoid: Ellipsoid {
                            ellipsoid_name: "GRS 1980".into(),
                            semi_major_axis: 6378137.0,
                            inverse_flattening: 298.257222101,
                            length_unit: Some(LengthUnit {
                                unit_name: "metre".into(),
                                conversion_factor: 1.0,
                            }),
                        },
                        anchor: None,
                        identifier: None,
                        prime_meridian: None,
                    }),
                    coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
                        spatial_cs_type: SpatialCsType::Cartesian,
                        dimension: Dimension::Three,
                        identifier: None,
                        spatial_axis: vec![
                            SpatialAxis {
                                axis_name_abbreviation: "(X)".into(),
                                axis_direction: AxisDirection::GeocentricX,
                                axis_order: None,
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Y)".into(),
                                axis_direction: AxisDirection::GeocentricY,
                                axis_order: None,
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Z)".into(),
                                axis_direction: AxisDirection::GeocentricZ,
                                axis_order: None,
                                spatial_unit: None,
                                identifier: None,
                            },
                        ],
                        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                            unit_name: "metre".into(),
                            conversion_factor: 1.0,
                        }))),
                    }),
                    scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
                        usage: None,
                        identifier: None,
                        remark: None,
                    },
                }),
            )),
        },
        operation_method: OperationMethod {
            operation_method_name: "NTv2".into(),
            identifier: Some(Id {
                authority_name: "EPSG".into(),
                authority_unique_identifier: NumText::Int(9615),
                version: None,
                authority_citation: None,
                id_uri: None,
            }),
        },
        operation_parameter_wrapper: Some(vec![OperationParameterWrapper::OperationParameterFile(
            OperationParameterFile {
                parameter_name: "Latitude and longitude difference file".into(),
                parameter_file_name: "nzdg2kgrid0005.gsb".into(),
                identifier: None,
            },
        )]),
        interpolation_crs: None,
        operation_accuracy: None,
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: None,
            identifier: Some(vec![Id {
                authority_name: "EPSG".into(),
                authority_unique_identifier: NumText::Int(1568),
                version: None,
                authority_citation: Some(Citation("LINZS25000".into())),
                id_uri: Some(Uri(
                    "http://www.linz.govt.nz/geodetic/software-downloads/".into()
                )),
            }]),
            remark: Some(Remark("Coordinate transformation accuracy 0.1-1.0m".into())),
        },
    };

    let ast = parse_wkt(EXAMPLE3);
    assert_eq!(ast.len(), 1);

    let acc = CoordinateOperation::from_nodes(&ast).unwrap();

    assert_eq!(correct, acc.result);
}

fn test_example_4() {
    let correct = CoordinateOperation {
        operation_name: "Amersfoort to ETRS98 (3)".into(),
        operation_version: None,
        source_crs: SourceCrs {
            coordinate_system: CoordinateReferenceSystem::SingleCrs(SingleCrs::GeodeticCrs(
                GeodeticCrs::StaticGeodeticCrs(StaticGeodeticCrs {
                    crs_name: "Tokyo".into(),
                    frame: GeodeticData::GeodeticReferenceFrame(GeodeticReferenceFrame {
                        datum_name: "Tokyo 1918".into(),
                        ellipsoid: Ellipsoid {
                            ellipsoid_name: "Bessel 1841".into(),
                            semi_major_axis: 6377397.155,
                            inverse_flattening: 299.1528128,
                            length_unit: Some(LengthUnit {
                                unit_name: "metre".into(),
                                conversion_factor: 1.0,
                            }),
                        },
                        anchor: None,
                        identifier: None,
                        prime_meridian: None,
                    }),
                    coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
                        spatial_cs_type: SpatialCsType::Cartesian,
                        dimension: Dimension::Three,
                        identifier: None,
                        spatial_axis: vec![
                            SpatialAxis {
                                axis_name_abbreviation: "(X)".into(),
                                axis_direction: AxisDirection::GeocentricX,
                                axis_order: Some(Order(1)),
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Y)".into(),
                                axis_direction: AxisDirection::GeocentricY,
                                axis_order: Some(Order(2)),
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Z)".into(),
                                axis_direction: AxisDirection::GeocentricZ,
                                axis_order: Some(Order(3)),
                                spatial_unit: None,
                                identifier: None,
                            },
                        ],
                        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                            unit_name: "metre".into(),
                            conversion_factor: 1.0,
                        }))),
                    }),
                    scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
                        identifier: None,
                        remark: None,
                        usage: None,
                    },
                }),
            )),
        },
        target_crs: TargetCrs {
            coordinate_system: CoordinateReferenceSystem::SingleCrs(SingleCrs::GeodeticCrs(
                GeodeticCrs::StaticGeodeticCrs(StaticGeodeticCrs {
                    crs_name: "JGD2000".into(),
                    frame: GeodeticData::GeodeticReferenceFrame(GeodeticReferenceFrame {
                        datum_name: "Japanese Geodetic Datum 2000".into(),
                        ellipsoid: Ellipsoid {
                            ellipsoid_name: "GRS 1980".into(),
                            semi_major_axis: 6378137.0,
                            inverse_flattening: 298.257222101,
                            length_unit: Some(LengthUnit {
                                unit_name: "metre".into(),
                                conversion_factor: 1.0,
                            }),
                        },
                        anchor: None,
                        identifier: None,
                        prime_meridian: None,
                    }),
                    coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
                        spatial_cs_type: SpatialCsType::Cartesian,
                        dimension: Dimension::Three,
                        identifier: None,
                        spatial_axis: vec![
                            SpatialAxis {
                                axis_name_abbreviation: "(X)".into(),
                                axis_direction: AxisDirection::GeocentricX,
                                axis_order: None,
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Y)".into(),
                                axis_direction: AxisDirection::GeocentricY,
                                axis_order: None,
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Z)".into(),
                                axis_direction: AxisDirection::GeocentricZ,
                                axis_order: None,
                                spatial_unit: None,
                                identifier: None,
                            },
                        ],
                        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                            unit_name: "metre".into(),
                            conversion_factor: 1.0,
                        }))),
                    }),
                    scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
                        usage: None,
                        identifier: None,
                        remark: None,
                    },
                }),
            )),
        },
        operation_method: OperationMethod {
            operation_method_name: "Coordinate Frame".into(),
            identifier: None,
        },
        operation_parameter_wrapper: Some(vec![
            OperationParameterWrapper::OperationParameter(OperationParameter {
                parameter_name: "X-axis translation".into(),
                parameter_value: 565.2369,
                parameter_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                    unit_name: "metre".into(),
                    conversion_factor: 1.0,
                }))),
                identifier: None,
            }),
            OperationParameterWrapper::OperationParameter(OperationParameter {
                parameter_name: "Y-axis translation".into(),
                parameter_value: 50.0087,
                parameter_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                    unit_name: "metre".into(),
                    conversion_factor: 1.0,
                }))),
                identifier: None,
            }),
        ]),
        interpolation_crs: None,
        operation_accuracy: None,
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: None,
            identifier: Some(vec![Id {
                authority_name: "EPSG".into(),
                authority_unique_identifier: NumText::Int(15739),
                version: None,
                authority_citation: None,
                id_uri: None,
            }]),
            remark: None,
        },
    };

    let ast = parse_wkt(EXAMPLE4);
    assert_eq!(ast.len(), 1);

    let acc = CoordinateOperation::from_nodes(&ast).unwrap();

    assert_eq!(correct, acc.result);
}

fn test_example_5() {
    let correct = CoordinateOperation {
        operation_name: "DHHN92 height to EVRF2007 height".into(),
        operation_version: None,
        source_crs: SourceCrs {
            coordinate_system: CoordinateReferenceSystem::SingleCrs(SingleCrs::GeodeticCrs(
                GeodeticCrs::StaticGeodeticCrs(StaticGeodeticCrs {
                    crs_name: "Tokyo".into(),
                    frame: GeodeticData::GeodeticReferenceFrame(GeodeticReferenceFrame {
                        datum_name: "Tokyo 1918".into(),
                        ellipsoid: Ellipsoid {
                            ellipsoid_name: "Bessel 1841".into(),
                            semi_major_axis: 6377397.155,
                            inverse_flattening: 299.1528128,
                            length_unit: Some(LengthUnit {
                                unit_name: "metre".into(),
                                conversion_factor: 1.0,
                            }),
                        },
                        anchor: None,
                        identifier: None,
                        prime_meridian: None,
                    }),
                    coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
                        spatial_cs_type: SpatialCsType::Cartesian,
                        dimension: Dimension::Three,
                        identifier: None,
                        spatial_axis: vec![
                            SpatialAxis {
                                axis_name_abbreviation: "(X)".into(),
                                axis_direction: AxisDirection::GeocentricX,
                                axis_order: Some(Order(1)),
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Y)".into(),
                                axis_direction: AxisDirection::GeocentricY,
                                axis_order: Some(Order(2)),
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Z)".into(),
                                axis_direction: AxisDirection::GeocentricZ,
                                axis_order: Some(Order(3)),
                                spatial_unit: None,
                                identifier: None,
                            },
                        ],
                        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                            unit_name: "metre".into(),
                            conversion_factor: 1.0,
                        }))),
                    }),
                    scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
                        identifier: None,
                        remark: None,
                        usage: None,
                    },
                }),
            )),
        },
        target_crs: TargetCrs {
            coordinate_system: CoordinateReferenceSystem::SingleCrs(SingleCrs::GeodeticCrs(
                GeodeticCrs::StaticGeodeticCrs(StaticGeodeticCrs {
                    crs_name: "JGD2000".into(),
                    frame: GeodeticData::GeodeticReferenceFrame(GeodeticReferenceFrame {
                        datum_name: "Japanese Geodetic Datum 2000".into(),
                        ellipsoid: Ellipsoid {
                            ellipsoid_name: "GRS 1980".into(),
                            semi_major_axis: 6378137.0,
                            inverse_flattening: 298.257222101,
                            length_unit: Some(LengthUnit {
                                unit_name: "metre".into(),
                                conversion_factor: 1.0,
                            }),
                        },
                        anchor: None,
                        identifier: None,
                        prime_meridian: None,
                    }),
                    coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
                        spatial_cs_type: SpatialCsType::Cartesian,
                        dimension: Dimension::Three,
                        identifier: None,
                        spatial_axis: vec![
                            SpatialAxis {
                                axis_name_abbreviation: "(X)".into(),
                                axis_direction: AxisDirection::GeocentricX,
                                axis_order: None,
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Y)".into(),
                                axis_direction: AxisDirection::GeocentricY,
                                axis_order: None,
                                spatial_unit: None,
                                identifier: None,
                            },
                            SpatialAxis {
                                axis_name_abbreviation: "(Z)".into(),
                                axis_direction: AxisDirection::GeocentricZ,
                                axis_order: None,
                                spatial_unit: None,
                                identifier: None,
                            },
                        ],
                        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                            unit_name: "metre".into(),
                            conversion_factor: 1.0,
                        }))),
                    }),
                    scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
                        usage: None,
                        identifier: None,
                        remark: None,
                    },
                }),
            )),
        },
        operation_method: OperationMethod {
            operation_method_name: "Vertical Offset and Slope".into(),
            identifier: Some(Id {
                authority_name: "EPSG".into(),
                authority_unique_identifier: NumText::Int(1046),
                version: None,
                authority_citation: None,
                id_uri: None,
            }),
        },
        operation_parameter_wrapper: Some(vec![
            OperationParameterWrapper::OperationParameter(OperationParameter {
                parameter_name: "Inclination in latitude".into(),
                parameter_value: -0.010,
                parameter_unit: Some(Unit::SpatialUnit(SpatialUnit::AngleUnit(AngleUnit {
                    unit_name: "arc-second".into(),
                    conversion_factor: 4.84E-06,
                    identifier: None,
                }))),
                identifier: None,
            }),
            OperationParameterWrapper::OperationParameter(OperationParameter {
                parameter_name: "Inclination in longitude".into(),
                parameter_value: 0.002,
                parameter_unit: Some(Unit::SpatialUnit(SpatialUnit::AngleUnit(AngleUnit {
                    unit_name: "arc-second".into(),
                    conversion_factor: 4.84E-06,
                    identifier: None,
                }))),
                identifier: None,
            }),
        ]),
        interpolation_crs: Some(InterpolationCrs {
            coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
                spatial_cs_type: SpatialCsType::Cartesian,
                dimension: Dimension::Three,
                identifier: None,
                spatial_axis: vec![
                    SpatialAxis {
                        axis_name_abbreviation: "(X)".into(),
                        axis_direction: AxisDirection::GeocentricX,
                        axis_order: Some(Order(1)),
                        spatial_unit: None,
                        identifier: None,
                    },
                    SpatialAxis {
                        axis_name_abbreviation: "(Y)".into(),
                        axis_direction: AxisDirection::GeocentricY,
                        axis_order: Some(Order(2)),
                        spatial_unit: None,
                        identifier: None,
                    },
                    SpatialAxis {
                        axis_name_abbreviation: "(Z)".into(),
                        axis_direction: AxisDirection::GeocentricZ,
                        axis_order: Some(Order(3)),
                        spatial_unit: None,
                        identifier: None,
                    },
                ],
                cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                    unit_name: "metre".into(),
                    conversion_factor: 1.0,
                }))),
            }),
        }),
        operation_accuracy: Some(OperationAccuracy(0.1)),
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: None,
            identifier: None,
            remark: Some(Remark("Determined at 427 points.".into())),
        },
    };

    let ast = parse_wkt(EXAMPLE5);
    assert_eq!(ast.len(), 1);

    let acc = CoordinateOperation::from_nodes(&ast).unwrap();

    assert_eq!(correct, acc.result);
}
