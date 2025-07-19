use crate::{
    ast::parse_wkt,
    coordinate_system::{
        AxisDirection, AxisOrder, CoordinateSystem, Dimension, SpatialAxis,
        SpatialCoordinateSystem, SpatialCsType,
    },
    datum::GeodeticReferenceFrameDatum,
    ellipsoid::Ellipsoid,
    geodetic_crs::GeodeticData,
    map_projection::{
        MapProjection, MapProjectionMethod, MapProjectionParameter, MapProjectionParameterUnit,
    },
    prime_meridian::PrimeMeridian,
    scope_extent_identifier_remark::{
        AreaDescription, Extent, Id, Remark, Scope, ScopeExtentIdentifierRemark, Usage,
    },
    units::{AngleUnit, LengthUnit, ScaleUnit, SpatialUnit, Unit},
    utils::NumText,
};

use super::{
    base_geodetic_crs::{BaseGeodeticGeographicCrs, BaseStaticGeographicCrs},
    proj_crs::ProjectedCrs,
};

const EXAMPLE1: &str = r#"PROJCRS["ETRS89 Lambert Azimuthal Equal Area CRS",
    BASEGEOGCRS["ETRS89",
        DATUM["ETRS89",
            ELLIPSOID["GRS 80",6378137,298.25,LENGTHUNIT["metre",1.0]]
        ],ID["EuroGeographics","ETRS89-LatLon"]
    ],
    CONVERSION["LAEA",
        METHOD["Lambert Azimuthal Equal Area",ID["EPSG",9820]],
        PARAMETER["Latitude of origin",52.0
            ANGLEUNIT["degree",0.017]]
        PARAMETER["Longitude of origin",10.0,
            ANGLEUNIT["degree",0.017]],
        PARAMETER["False easting",4321000.0,LENGTHUNIT["metre",1.0]],
        PARAMETER["False northing",3210000.0,LENGTHUNIT["metre",1.0]]
    ],
    CS[Cartesian,2],
        AXIS["(Y)",north,ORDER[1]],
        AXIS["(X)",east,ORDER[2]],
        LENGTHUNIT["metre",1.0],
    USAGE[SCOPE["Description of purpose"],AREA["An area description"]],
    ID["EuroGeographics","ETRS-LAEA"]
]
"#;

const EXAMPLE2: &str = r#"PROJCRS["NAD27 / Texas South Central",
    BASEGEOGCRS["NAD27",
        DATUM["North American Datum 1927",
            ELLIPSOID["Clarke 1866",20925832.164,294.97,
                LENGTHUNIT["US survey foot",0.304]]
        ]
    ],
    CONVERSION["Texas South Central SPCS27",
        METHOD["Lambert Conic Conformal (2SP)",ID["EPSG",9802]],
        PARAMETER["Latitude of false origin",27.83,
            ANGLEUNIT["degree",0.017],ID["EPSG",8821]],
        PARAMETER["Longitude of false origin",-99.0,
            ANGLEUNIT["degree",0.017],ID["EPSG",8822]],
        PARAMETER["Latitude of 1st standard parallel",28.38,
            ANGLEUNIT["degree",0.017],ID["EPSG",8823]],
        PARAMETER["Latitude of 2nd standard parallel",30.28,
            ANGLEUNIT["degree",0.017],ID["EPSG",8824]],
        PARAMETER["Easting at false origin",2000000.0,
            LENGTHUNIT["US survey foot",0.304],ID["EPSG",8826]],
        PARAMETER["Northing at false origin",0.0,
            LENGTHUNIT["US survey foot",0.304],ID["EPSG",8827]]
    ],
    CS[Cartesian,2],
        AXIS["(X)",east],
        AXIS["(Y)",north],
        LENGTHUNIT["US survey foot",0.304],
    REMARK["Fundamental point: Meade's ranch"]
]
"#;

const EXAMPLE3: &str = r#"PROJCRS["NAD83 UTM 10",
    BASEGEOGCRS["NAD83(86)",
        DATUM["North American Datum 1983",
            ELLIPSOID["GRS 1980",6378137,298.257]
        ],
        PRIMEM["Greenwich",0]
    ],
    CONVERSION["UTM zone 10N",
        METHOD["Transverse Mercator"],
        PARAMETER["Latitude of natural origin",0.0],
        PARAMETER["Longitude of natural origin",-123.0],
        PARAMETER["Scale factor",0.9996],
        PARAMETER["False easting",500000.0],
        PARAMETER["False northing",0.0],
        ID["EPSG",16010]
    ],
    CS[Cartesian,2],
        AXIS["(E)",east,ORDER[1]],
        AXIS["(N)",north,ORDER[2]],
        LENGTHUNIT["metre",1.0],
    REMARK["Some remark"]
]
"#;

const EXAMPLE4: &str = r#"PROJCRS["WGS 84 (G1762) / UTM zone 31N 3D",
    BASEGEOGCRS["WGS 84",
        DATUM["World Geodetic System of 1984 (G1762)",
            ELLIPSOID["WGS 84",6378137,298.257,LENGTHUNIT["metre",1.0]]
        ]
    ],
    CONVERSION["UTM zone 31N 3D",
        METHOD["Transverse Mercator (3D)"],
        PARAMETER["Latitude of origin",0.0,ANGLEUNIT["degree",0.017]],
        PARAMETER["Longitude of origin",3.0,ANGLEUNIT["degree",0.017]],
        PARAMETER["Scale factor",0.9996,SCALEUNIT["unity",1.0]],
        PARAMETER["False easting",500000.0,LENGTHUNIT["metre",1.0]],
        PARAMETER["False northing",0.0,LENGTHUNIT["metre",1.0]]
    ],
    CS[Cartesian,3],
        AXIS["(E)",east,ORDER[1]],
        AXIS["(N)",north,ORDER[2]],
        AXIS["ellipsoidal height (h)",up,ORDER[3]],
        LENGTHUNIT["metre",1.0]
]
"#;

#[test]
fn test_proj_crs() {
    test_example_1();
    test_example_2();
    test_example_3();
    test_example_4();
}

fn test_example_1() {
    let correct = ProjectedCrs {
        crs_name: "ETRS89 Lambert Azimuthal Equal Area CRS".into(),
        base_geodetic_crs: BaseGeodeticGeographicCrs::BaseStaticGeographicCrs(
            BaseStaticGeographicCrs {
                base_crs_name: "ETRS89".into(),
                geodetic_data: GeodeticData::GeodeticReferenceFrame(GeodeticReferenceFrameDatum {
                    datum_name: "ETRS89".into(),
                    ellipsoid: Ellipsoid {
                        ellipsoid_name: "GRS 80".into(),
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
                }),
                ellipsoidal_cs_unit: None,
                identifier: Some(Id {
                    authority_name: "EuroGeographics".into(),
                    authority_unique_identifier: NumText::Text("ETRS89-LatLon".into()),
                    version: None,
                    authority_citation: None,
                    id_uri: None,
                }),
            },
        ),
        map_projection: MapProjection {
            map_projection_name: "LAEA".into(),
            map_projection_method: MapProjectionMethod {
                map_projection_method_name: "Lambert Azimuthal Equal Area".into(),
                identifier: Some(Id {
                    authority_name: "EPSG".into(),
                    authority_unique_identifier: NumText::Num(9820.0),
                    version: None,
                    authority_citation: None,
                    id_uri: None,
                }),
            },
            map_projection_parameters: Some(vec![
                MapProjectionParameter {
                    parameter_name: "Latitude of origin".into(),
                    parameter_value: 52.0,
                    map_projection_parameter_unit: Some(MapProjectionParameterUnit::AngleUnit(
                        AngleUnit {
                            unit_name: "degree".into(),
                            conversion_factor: 0.017,
                            identifier: None,
                        },
                    )),
                    identifier: None,
                },
                MapProjectionParameter {
                    parameter_name: "Longitude of origin".into(),
                    parameter_value: 10.0,
                    map_projection_parameter_unit: Some(MapProjectionParameterUnit::AngleUnit(
                        AngleUnit {
                            unit_name: "degree".into(),
                            conversion_factor: 0.017,
                            identifier: None,
                        },
                    )),
                    identifier: None,
                },
                MapProjectionParameter {
                    parameter_name: "False easting".into(),
                    parameter_value: 4321000.0,
                    map_projection_parameter_unit: Some(MapProjectionParameterUnit::LengthUnit(
                        LengthUnit {
                            unit_name: "metre".into(),
                            conversion_factor: 1.0,
                        },
                    )),
                    identifier: None,
                },
                MapProjectionParameter {
                    parameter_name: "False northing".into(),
                    parameter_value: 3210000.0,
                    map_projection_parameter_unit: Some(MapProjectionParameterUnit::LengthUnit(
                        LengthUnit {
                            unit_name: "metre".into(),
                            conversion_factor: 1.0,
                        },
                    )),
                    identifier: None,
                },
            ]),
            identifier: None,
        },
        coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
            spatial_cs_type: SpatialCsType::Cartesian,
            dimension: Dimension::Two,
            identifier: None,
            spatial_axis: vec![
                SpatialAxis {
                    axis_name_abbreviation: "(Y)".into(),
                    axis_direction: AxisDirection::North(None),
                    axis_order: Some(AxisOrder { order: 1.0 }),
                    spatial_unit: None,
                    identifier: None,
                },
                SpatialAxis {
                    axis_name_abbreviation: "(X)".into(),
                    axis_direction: AxisDirection::East,
                    axis_order: Some(AxisOrder { order: 2.0 }),
                    spatial_unit: None,
                    identifier: None,
                },
            ],
            cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                unit_name: "metre".into(),
                conversion_factor: 1.0,
            }))),
            needed_args: 4,
        }),
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: Some(vec![Usage {
                scope: Scope("Description of purpose".into()),
                extent: Extent {
                    area_description: Some(AreaDescription("An area description".into())),
                    geographic_bounding_box: None,
                    vertical_extent: None,
                    temporal_extent: None,
                },
            }]),
            identifier: Some(vec![Id {
                authority_name: "EuroGeographics".into(),
                authority_unique_identifier: NumText::Text("ETRS-LAEA".into()),
                version: None,
                authority_citation: None,
                id_uri: None,
            }]),
            remark: None,
        },
    };

    let ast = parse_wkt(EXAMPLE1);
    assert_eq!(ast.len(), 1);
    let map_proj = ProjectedCrs::try_from(&ast[0]).unwrap();

    assert_eq!(correct, map_proj);
}

fn test_example_2() {
    let correct = ProjectedCrs {
        crs_name: "NAD27 / Texas South Central".into(),
        base_geodetic_crs: BaseGeodeticGeographicCrs::BaseStaticGeographicCrs(
            BaseStaticGeographicCrs {
                base_crs_name: "NAD27".into(),
                geodetic_data: GeodeticData::GeodeticReferenceFrame(GeodeticReferenceFrameDatum {
                    datum_name: "North American Datum 1927".into(),
                    ellipsoid: Ellipsoid {
                        ellipsoid_name: "Clarke 1866".into(),
                        semi_major_axis: 20925832.164,
                        inverse_flattening: 294.97,
                        length_unit: Some(LengthUnit {
                            unit_name: "US survey foot".into(),
                            conversion_factor: 0.304,
                        }),
                    },
                    anchor: None,
                    identifier: None,
                    prime_meridian: None,
                }),
                ellipsoidal_cs_unit: None,
                identifier: None,
            },
        ),
        map_projection: MapProjection {
            map_projection_name: "Texas South Central SPCS27".into(),
            map_projection_method: MapProjectionMethod {
                map_projection_method_name: "Lambert Conic Conformal (2SP)".into(),
                identifier: Some(Id {
                    authority_name: "EPSG".into(),
                    authority_unique_identifier: NumText::Num(9802.0),
                    version: None,
                    authority_citation: None,
                    id_uri: None,
                }),
            },
            map_projection_parameters: Some(vec![
                MapProjectionParameter {
                    parameter_name: "Latitude of false origin".into(),
                    parameter_value: 27.83,
                    map_projection_parameter_unit: Some(MapProjectionParameterUnit::AngleUnit(
                        AngleUnit {
                            unit_name: "degree".into(),
                            conversion_factor: 0.017,
                            identifier: None,
                        },
                    )),
                    identifier: Some(Id {
                        authority_name: "EPSG".into(),
                        authority_unique_identifier: NumText::Num(8821.0),
                        version: None,
                        authority_citation: None,
                        id_uri: None,
                    }),
                },
                MapProjectionParameter {
                    parameter_name: "Longitude of false origin".into(),
                    parameter_value: -99.0,
                    map_projection_parameter_unit: Some(MapProjectionParameterUnit::AngleUnit(
                        AngleUnit {
                            unit_name: "degree".into(),
                            conversion_factor: 0.017,
                            identifier: None,
                        },
                    )),
                    identifier: Some(Id {
                        authority_name: "EPSG".into(),
                        authority_unique_identifier: NumText::Num(8822.0),
                        version: None,
                        authority_citation: None,
                        id_uri: None,
                    }),
                },
                MapProjectionParameter {
                    parameter_name: "Latitude of 1st standard parallel".into(),
                    parameter_value: 28.38,
                    map_projection_parameter_unit: Some(MapProjectionParameterUnit::AngleUnit(
                        AngleUnit {
                            unit_name: "degree".into(),
                            conversion_factor: 0.017,
                            identifier: None,
                        },
                    )),
                    identifier: Some(Id {
                        authority_name: "EPSG".into(),
                        authority_unique_identifier: NumText::Num(8823.0),
                        version: None,
                        authority_citation: None,
                        id_uri: None,
                    }),
                },
                MapProjectionParameter {
                    parameter_name: "Latitude of 2nd standard parallel".into(),
                    parameter_value: 30.28,
                    map_projection_parameter_unit: Some(MapProjectionParameterUnit::AngleUnit(
                        AngleUnit {
                            unit_name: "degree".into(),
                            conversion_factor: 0.017,
                            identifier: None,
                        },
                    )),
                    identifier: Some(Id {
                        authority_name: "EPSG".into(),
                        authority_unique_identifier: NumText::Num(8824.0),
                        version: None,
                        authority_citation: None,
                        id_uri: None,
                    }),
                },
                MapProjectionParameter {
                    parameter_name: "Easting at false origin".into(),
                    parameter_value: 2000000.0,
                    map_projection_parameter_unit: Some(MapProjectionParameterUnit::LengthUnit(
                        LengthUnit {
                            unit_name: "US survey foot".into(),
                            conversion_factor: 0.304,
                        },
                    )),
                    identifier: Some(Id {
                        authority_name: "EPSG".into(),
                        authority_unique_identifier: NumText::Num(8826.0),
                        version: None,
                        authority_citation: None,
                        id_uri: None,
                    }),
                },
                MapProjectionParameter {
                    parameter_name: "Northing at false origin".into(),
                    parameter_value: 0.0,
                    map_projection_parameter_unit: Some(MapProjectionParameterUnit::LengthUnit(
                        LengthUnit {
                            unit_name: "US survey foot".into(),
                            conversion_factor: 0.304,
                        },
                    )),
                    identifier: Some(Id {
                        authority_name: "EPSG".into(),
                        authority_unique_identifier: NumText::Num(8827.0),
                        version: None,
                        authority_citation: None,
                        id_uri: None,
                    }),
                },
            ]),
            identifier: None,
        },
        coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
            spatial_cs_type: SpatialCsType::Cartesian,
            dimension: Dimension::Two,
            identifier: None,
            spatial_axis: vec![
                SpatialAxis {
                    axis_name_abbreviation: "(X)".into(),
                    axis_direction: AxisDirection::East,
                    axis_order: None,
                    spatial_unit: None,
                    identifier: None,
                },
                SpatialAxis {
                    axis_name_abbreviation: "(Y)".into(),
                    axis_direction: AxisDirection::North(None),
                    axis_order: None,
                    spatial_unit: None,
                    identifier: None,
                },
            ],
            cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                unit_name: "US survey foot".into(),
                conversion_factor: 0.304,
            }))),
            needed_args: 4,
        }),
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: None,
            identifier: None,
            remark: Some(Remark {
                remark: "Fundamental point: Meade's ranch".into(),
            }),
        },
    };

    let ast = parse_wkt(EXAMPLE2);
    assert_eq!(ast.len(), 1);
    let map_proj = ProjectedCrs::try_from(&ast[0]).unwrap();

    assert_eq!(correct, map_proj);
}

fn test_example_3() {
    let correct = ProjectedCrs {
        crs_name: "NAD83 UTM 10".into(),
        base_geodetic_crs: BaseGeodeticGeographicCrs::BaseStaticGeographicCrs(
            BaseStaticGeographicCrs {
                base_crs_name: "NAD83(86)".into(),
                geodetic_data: GeodeticData::GeodeticReferenceFrame(GeodeticReferenceFrameDatum {
                    datum_name: "North American Datum 1983".into(),
                    ellipsoid: Ellipsoid {
                        ellipsoid_name: "GRS 1980".into(),
                        semi_major_axis: 6378137.0,
                        inverse_flattening: 298.257,
                        length_unit: None,
                    },
                    anchor: None,
                    identifier: None,
                    prime_meridian: Some(PrimeMeridian {
                        prime_meridian_name: "Greenwich".into(),
                        irm_longitude: 0.0,
                        angle_unit: None,
                        identifier: None,
                    }),
                }),
                ellipsoidal_cs_unit: None,
                identifier: None,
            },
        ),
        map_projection: MapProjection {
            map_projection_name: "UTM zone 10N".into(),
            map_projection_method: MapProjectionMethod {
                map_projection_method_name: "Transverse Mercator".into(),
                identifier: None,
            },
            map_projection_parameters: Some(vec![
                MapProjectionParameter {
                    parameter_name: "Latitude of natural origin".into(),
                    parameter_value: 0.0,
                    map_projection_parameter_unit: None,
                    identifier: None,
                },
                MapProjectionParameter {
                    parameter_name: "Longitude of natural origin".into(),
                    parameter_value: -123.0,
                    map_projection_parameter_unit: None,
                    identifier: None,
                },
                MapProjectionParameter {
                    parameter_name: "Scale factor".into(),
                    parameter_value: 0.9996,
                    map_projection_parameter_unit: None,
                    identifier: None,
                },
                MapProjectionParameter {
                    parameter_name: "False easting".into(),
                    parameter_value: 500000.0,
                    map_projection_parameter_unit: None,
                    identifier: None,
                },
                MapProjectionParameter {
                    parameter_name: "False northing".into(),
                    parameter_value: 0.0,
                    map_projection_parameter_unit: None,
                    identifier: None,
                },
            ]),
            identifier: Some(Id {
                authority_name: "EPSG".into(),
                authority_unique_identifier: NumText::Num(16010.0),
                version: None,
                authority_citation: None,
                id_uri: None,
            }),
        },
        coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
            spatial_cs_type: SpatialCsType::Cartesian,
            dimension: Dimension::Two,
            identifier: None,
            spatial_axis: vec![
                SpatialAxis {
                    axis_name_abbreviation: "(E)".into(),
                    axis_direction: AxisDirection::East,
                    axis_order: Some(AxisOrder { order: 1.0 }),
                    spatial_unit: None,
                    identifier: None,
                },
                SpatialAxis {
                    axis_name_abbreviation: "(N)".into(),
                    axis_direction: AxisDirection::North(None),
                    axis_order: Some(AxisOrder { order: 2.0 }),
                    spatial_unit: None,
                    identifier: None,
                },
            ],
            cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                unit_name: "metre".into(),
                conversion_factor: 1.0,
            }))),
            needed_args: 4,
        }),
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: None,
            identifier: None,
            remark: Some(Remark {
                remark: "Some remark".into(),
            }),
        },
    };

    let ast = parse_wkt(EXAMPLE3);
    assert_eq!(ast.len(), 1);
    let map_proj = ProjectedCrs::try_from(&ast[0]).unwrap();

    assert_eq!(correct, map_proj);
}

fn test_example_4() {
    let correct = ProjectedCrs {
        crs_name: "WGS 84 (G1762) / UTM zone 31N 3D".into(),
        base_geodetic_crs: BaseGeodeticGeographicCrs::BaseStaticGeographicCrs(
            BaseStaticGeographicCrs {
                base_crs_name: "WGS 84".into(),
                geodetic_data: GeodeticData::GeodeticReferenceFrame(GeodeticReferenceFrameDatum {
                    datum_name: "World Geodetic System of 1984 (G1762)".into(),
                    ellipsoid: Ellipsoid {
                        ellipsoid_name: "WGS 84".into(),
                        semi_major_axis: 6378137.0,
                        inverse_flattening: 298.257,
                        length_unit: Some(LengthUnit {
                            unit_name: "metre".into(),
                            conversion_factor: 1.0,
                        }),
                    },
                    anchor: None,
                    identifier: None,
                    prime_meridian: None,
                }),
                ellipsoidal_cs_unit: None,
                identifier: None,
            },
        ),
        map_projection: MapProjection {
            map_projection_name: "UTM zone 31N 3D".into(),
            map_projection_method: MapProjectionMethod {
                map_projection_method_name: "Transverse Mercator (3D)".into(),
                identifier: None,
            },
            map_projection_parameters: Some(vec![
                MapProjectionParameter {
                    parameter_name: "Latitude of origin".into(),
                    parameter_value: 0.0,
                    map_projection_parameter_unit: Some(MapProjectionParameterUnit::AngleUnit(
                        AngleUnit {
                            unit_name: "degree".into(),
                            conversion_factor: 0.017,
                            identifier: None,
                        },
                    )),
                    identifier: None,
                },
                MapProjectionParameter {
                    parameter_name: "Longitude of origin".into(),
                    parameter_value: 3.0,
                    map_projection_parameter_unit: Some(MapProjectionParameterUnit::AngleUnit(
                        AngleUnit {
                            unit_name: "degree".into(),
                            conversion_factor: 0.017,
                            identifier: None,
                        },
                    )),
                    identifier: None,
                },
                MapProjectionParameter {
                    parameter_name: "Scale factor".into(),
                    parameter_value: 0.9996,
                    map_projection_parameter_unit: Some(MapProjectionParameterUnit::ScaleUnit(
                        ScaleUnit {
                            unit_name: "unity".into(),
                            conversion_factor: 1.0,
                            identifier: None,
                        },
                    )),
                    identifier: None,
                },
                MapProjectionParameter {
                    parameter_name: "False easting".into(),
                    parameter_value: 500000.0,
                    map_projection_parameter_unit: Some(MapProjectionParameterUnit::LengthUnit(
                        LengthUnit {
                            unit_name: "metre".into(),
                            conversion_factor: 1.0,
                        },
                    )),
                    identifier: None,
                },
                MapProjectionParameter {
                    parameter_name: "False northing".into(),
                    parameter_value: 0.0,
                    map_projection_parameter_unit: Some(MapProjectionParameterUnit::LengthUnit(
                        LengthUnit {
                            unit_name: "metre".into(),
                            conversion_factor: 1.0,
                        },
                    )),
                    identifier: None,
                },
            ]),
            identifier: None,
        },
        coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
            spatial_cs_type: SpatialCsType::Cartesian,
            dimension: Dimension::Three,
            identifier: None,
            spatial_axis: vec![
                SpatialAxis {
                    axis_name_abbreviation: "(E)".into(),
                    axis_direction: AxisDirection::East,
                    axis_order: Some(AxisOrder { order: 1.0 }),
                    spatial_unit: None,
                    identifier: None,
                },
                SpatialAxis {
                    axis_name_abbreviation: "(N)".into(),
                    axis_direction: AxisDirection::North(None),
                    axis_order: Some(AxisOrder { order: 2.0 }),
                    spatial_unit: None,
                    identifier: None,
                },
                SpatialAxis {
                    axis_name_abbreviation: "ellipsoidal height (h)".into(),
                    axis_direction: AxisDirection::Up,
                    axis_order: Some(AxisOrder { order: 3.0 }),
                    spatial_unit: None,
                    identifier: None,
                },
            ],
            cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                unit_name: "metre".into(),
                conversion_factor: 1.0,
            }))),
            needed_args: 5,
        }),
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: None,
            identifier: None,
            remark: None,
        },
    };

    let ast = parse_wkt(EXAMPLE4);
    assert_eq!(ast.len(), 1);
    let map_proj = ProjectedCrs::try_from(&ast[0]).unwrap();

    assert_eq!(correct, map_proj);
}
