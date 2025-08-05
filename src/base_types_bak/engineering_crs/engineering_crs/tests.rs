use crate::{
    ast::parse_wkt,
    coordinate_system::{
        AxisDirection, AxisOrder, CoordinateSystem, Dimension, OrdinalDateTimeAxis,
        OrdinalDateTimeCoordinateSystem, OrdinalDateTimeCsType, SpatialAxis,
        SpatialCoordinateSystem, SpatialCsType,
    },
    datum::DatumAnchor,
    engineering_crs::engineering_datum::EngineeringDatum,
    scope_extent_identifier_remark::{
        DateOrString, Extent, Id, Scope, ScopeExtentIdentifierRemark, TemporalExtent, Usage,
    },
    units::{LengthUnit, SpatialUnit, Unit},
    utils::NumText,
};

use super::engineering_crs::EngineeringCrs;

const EXAMPLE1: &str = r#"ENGCRS["A construction site CRS",
    EDATUM["P1",ANCHOR["Peg in south corner"]],
    CS[Cartesian,2],
        AXIS["site east",southWest,ORDER[1]],
        AXIS["site north",southEast,ORDER[2]],
        LENGTHUNIT["metre",1.0],
    USAGE[SCOPE["Construction"],TIMEEXTENT["date/time t1","date/time t2"]]
]
"#;

const EXAMPLE2: &str = r#"ENGINEERINGCRS["Astra Minas Grid",
    ENGINEERINGDATUM["Astra Minas"],
    CS[Cartesian,2],
        AXIS["northing (X)",north,ORDER[1]],
        AXIS["westing (Y)",west,ORDER[2]],
        LENGTHUNIT["metre",1.0],
    ID["EPSG",5800]
]
"#;

const EXAMPLE3: &str = r#"ENGCRS["A ship-centred CRS",
    EDATUM["Ship reference point",ANCHOR["Centre of buoyancy"]],
    CS[Cartesian,3],
        AXIS["(x)",forward],
        AXIS["(y)",starboard],
        AXIS["(z)",down],
        LENGTHUNIT["metre",1.0]
]
"#;

const EXAMPLE4: &str = r#"ENGCRS["An analogue image CRS",
    EDATUM["Image reference point",
        ANCHOR["Top left corner of image = 0,0"]],
    CS[Cartesian,2],
        AXIS["Column (x)",columnPositive],
        AXIS["Row (y)",rowPositive],
        LENGTHUNIT["micrometre",1E-6]
]
"#;

const EXAMPLE5: &str = r#"ENGCRS["A digital image CRS",
    EDATUM["Image reference point",
        ANCHOR["Top left corner of image = 0,0"]],
    CS[ordinal,2],
        AXIS["Column pixel (x)",columnPositive,ORDER[1]],
        AXIS["Row pixel (y)",rowPositive,ORDER[2]]
]
"#;

#[test]
fn test_eng_crs() {
    test_example_1();
    test_example_2();
    test_example_3();
    test_example_4();
    test_example_5();
}

fn test_example_1() {
    let correct = EngineeringCrs {
        crs_name: "A construction site CRS".into(),
        engineering_datum: EngineeringDatum {
            datum_name: "P1".into(),
            datum_anchor: Some(DatumAnchor("Peg in south corner".into())),
            identifier: None,
        },
        coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
            spatial_cs_type: SpatialCsType::Cartesian,
            dimension: Dimension::Two,
            identifier: None,
            spatial_axis: vec![
                SpatialAxis {
                    axis_name_abbreviation: "site east".to_string(),
                    axis_direction: AxisDirection::SouthWest,
                    axis_order: Some(AxisOrder { order: 1.0 }),
                    spatial_unit: None,
                    identifier: None,
                },
                SpatialAxis {
                    axis_name_abbreviation: "site north".to_string(),
                    axis_direction: AxisDirection::SouthEast,
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
                scope: Scope("Construction".into()),
                extent: Extent {
                    area_description: None,
                    geographic_bounding_box: None,
                    vertical_extent: None,
                    temporal_extent: Some(TemporalExtent {
                        from: DateOrString::String("date/time t1".into()),
                        to: DateOrString::String("date/time t2".into()),
                    }),
                },
            }]),
            identifier: None,
            remark: None,
        },
    };

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let eng = EngineeringCrs::try_from(&ast[0]).unwrap();

    assert_eq!(correct, eng);
}

fn test_example_2() {
    let correct = EngineeringCrs {
        crs_name: "Astra Minas Grid".into(),
        engineering_datum: EngineeringDatum {
            datum_name: "Astra Minas".into(),
            datum_anchor: None,
            identifier: None,
        },
        coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
            spatial_cs_type: SpatialCsType::Cartesian,
            dimension: Dimension::Two,
            identifier: None,
            spatial_axis: vec![
                SpatialAxis {
                    axis_name_abbreviation: "northing (X)".to_string(),
                    axis_direction: AxisDirection::North(None),
                    axis_order: Some(AxisOrder { order: 1.0 }),
                    spatial_unit: None,
                    identifier: None,
                },
                SpatialAxis {
                    axis_name_abbreviation: "westing (Y)".to_string(),
                    axis_direction: AxisDirection::West,
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
            identifier: Some(vec![Id {
                authority_name: "EPSG".into(),
                authority_unique_identifier: NumText::Num(5800.0),
                version: None,
                authority_citation: None,
                id_uri: None,
            }]),
            remark: None,
        },
    };

    let ast = parse_wkt(EXAMPLE2);

    assert_eq!(ast.len(), 1);

    let eng = EngineeringCrs::try_from(&ast[0]).unwrap();

    assert_eq!(correct, eng);
}

fn test_example_3() {
    let correct = EngineeringCrs {
        crs_name: "A ship-centred CRS".into(),
        engineering_datum: EngineeringDatum {
            datum_name: "Ship reference point".into(),
            datum_anchor: Some(DatumAnchor("Centre of buoyancy".into())),
            identifier: None,
        },
        coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
            spatial_cs_type: SpatialCsType::Cartesian,
            dimension: Dimension::Three,
            identifier: None,
            spatial_axis: vec![
                SpatialAxis {
                    axis_name_abbreviation: "(x)".to_string(),
                    axis_direction: AxisDirection::Forward,
                    axis_order: None,
                    spatial_unit: None,
                    identifier: None,
                },
                SpatialAxis {
                    axis_name_abbreviation: "(y)".to_string(),
                    axis_direction: AxisDirection::Starboard,
                    axis_order: None,
                    spatial_unit: None,
                    identifier: None,
                },
                SpatialAxis {
                    axis_name_abbreviation: "(z)".to_string(),
                    axis_direction: AxisDirection::Down,
                    axis_order: None,
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

    let ast = parse_wkt(EXAMPLE3);

    assert_eq!(ast.len(), 1);

    let eng = EngineeringCrs::try_from(&ast[0]).unwrap();

    assert_eq!(correct, eng);
}

fn test_example_4() {
    let correct = EngineeringCrs {
        crs_name: "An analogue image CRS".into(),
        engineering_datum: EngineeringDatum {
            datum_name: "Image reference point".into(),
            datum_anchor: Some(DatumAnchor("Top left corner of image = 0,0".into())),
            identifier: None,
        },
        coordinate_system: CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
            spatial_cs_type: SpatialCsType::Cartesian,
            dimension: Dimension::Two,
            identifier: None,
            spatial_axis: vec![
                SpatialAxis {
                    axis_name_abbreviation: "Column (x)".to_string(),
                    axis_direction: AxisDirection::ColumnPositive,
                    axis_order: None,
                    spatial_unit: None,
                    identifier: None,
                },
                SpatialAxis {
                    axis_name_abbreviation: "Row (y)".to_string(),
                    axis_direction: AxisDirection::RowPositive,
                    axis_order: None,
                    spatial_unit: None,
                    identifier: None,
                },
            ],
            cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
                unit_name: "micrometre".into(),
                conversion_factor: 1e-6,
            }))),
            needed_args: 4,
        }),
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: None,
            identifier: None,
            remark: None,
        },
    };

    let ast = parse_wkt(EXAMPLE4);

    assert_eq!(ast.len(), 1);

    let eng = EngineeringCrs::try_from(&ast[0]).unwrap();

    assert_eq!(correct, eng);
}

fn test_example_5() {
    let correct = EngineeringCrs {
        crs_name: "A digital image CRS".into(),
        engineering_datum: EngineeringDatum {
            datum_name: "Image reference point".into(),
            datum_anchor: Some(DatumAnchor("Top left corner of image = 0,0".into())),
            identifier: None,
        },
        coordinate_system: CoordinateSystem::OrdinalDateTimeCS(OrdinalDateTimeCoordinateSystem {
            ordinal_date_time_cs_type: OrdinalDateTimeCsType::Ordinal,
            dimension: Dimension::Two,
            identifier: None,
            ordinal_date_time_axis: vec![
                OrdinalDateTimeAxis {
                    axis_name_abbreviation: "Column pixel (x)".into(),
                    axis_direction: AxisDirection::ColumnPositive,
                    axis_order: Some(AxisOrder { order: 1.0 }),
                    identifier: None,
                },
                OrdinalDateTimeAxis {
                    axis_name_abbreviation: "Row pixel (y)".into(),
                    axis_direction: AxisDirection::RowPositive,
                    axis_order: Some(AxisOrder { order: 2.0 }),
                    identifier: None,
                },
            ],
            needed_args: 3,
        }),
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: None,
            identifier: None,
            remark: None,
        },
    };

    let ast = parse_wkt(EXAMPLE5);

    assert_eq!(ast.len(), 1);

    let eng = EngineeringCrs::try_from(&ast[0]).unwrap();

    assert_eq!(correct, eng);
}
