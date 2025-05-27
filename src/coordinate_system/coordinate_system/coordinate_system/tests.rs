use std::vec;

use crate::{
    ast::{WktArg, parse_wkt},
    coordinate_system::{
        self,
        axis_direction::AxisDirection,
        axis_order::AxisOrder,
        bearing::Bearing,
        coordinate_system::{
            ordinal_date_time_coordinate_system::OrdinalDateTimeCoordinateSystem,
            spatial_coordinate_system::SpatialCoordinateSystem,
        },
        cs_type::{OrdinalDateTimeCsType, SpatialCsType},
        dimension::Dimension,
        meridian::Meridian,
        ordinal_date_time_axis::OrdinalDateTimeAxis,
        spatial_axis::SpatialAxis,
    },
    keywords::CS,
    units::{AngleUnit, LengthUnit, SpatialUnit, Unit},
};

use super::CoordinateSystem;

const EXAMPLE1: &str = r#"CS[ellipsoidal,2],
    AXIS["latitude",north,ORDER[1]],
    AXIS["longitude",east,ORDER[2]],
    ANGLEUNIT["degree",0.0174]"#;

const EXAMPLE2: &str = r#"CS[Cartesian,3],
AXIS["(X)",geocentricX],
AXIS["(Y)",geocentricY],
AXIS["(Z)",geocentricZ],
LENGTHUNIT["metre",1.0]
"#;

const EXAMPLE3: &str = r#"CS[Cartesian,3],
AXIS["(X)",east],AXIS["(Y)",north],AXIS["(Z)",up],
LENGTHUNIT["metre",1.0]
"#;

const EXAMPLE4: &str = r#"CS[spherical,3],
AXIS["distance (r)",awayFrom,ORDER[1],LENGTHUNIT["kilometre",1000]],
AXIS["longitude (U)",counterClockwise,BEARING[0],ORDER[2],ANGLEUNIT["degree",0.017]],
AXIS["elevation (V)",up,ORDER[3],ANGLEUNIT["degree",0.017]]
"#;

const EXAMPLE5: &str = r#"CS[ellipsoidal,3],
AXIS["latitude",north,ORDER[1],ANGLEUNIT["degree",0.017]],
AXIS["longitude",east,ORDER[2],ANGLEUNIT["degree",0.017]],
AXIS["ellipsoidal height (h)",up,ORDER[3],LENGTHUNIT["metre",1.0]]
"#;

const EXAMPLE6: &str = r#"CS[ellipsoidal,2],
AXIS["(lat)",north],
AXIS["(lon)",east],
ANGLEUNIT["degree",0.017]
"#;

const EXAMPLE7: &str = r#"CS[Cartesian,2]
AXIS["(E)",east,ORDER[1],LENGTHUNIT["metre",1.0]],
AXIS["(N)",north,ORDER[2],LENGTHUNIT["metre",1.0]]
"#;

const EXAMPLE8: &str = r#"CS[Cartesian,2]
AXIS["(E)",east,ORDER[1]],
AXIS["(N)",north,ORDER[2]],
LENGTHUNIT["metre",1.0]
"#;

const EXAMPLE9: &str = r#"CS[Cartesian,2]
AXIS["northing (X)",north,ORDER[1]],
AXIS["easting (Y)",east,ORDER[2]],
LENGTHUNIT["German legal metre",1.0000135965]
"#;

const EXAMPLE10: &str = r#"CS[Cartesian,2],
AXIS["easting (X)",south,MERIDIAN[90,ANGLEUNIT["degree",0.017]],ORDER[1]],
AXIS["northing (Y)",south,MERIDIAN[180,ANGLEUNIT["degree",0.017]],ORDER[2]],
LENGTHUNIT["metre",1.0]
"#;

const EXAMPLE11: &str = r#"CS[Cartesian,3],
AXIS["(E)",east],
AXIS["(N)",north],
AXIS["ellipsoid height (h)",up],
LENGTHUNIT["metre",1.0]
"#;

const EXAMPLE12: &str = r#"CS[vertical,1],
AXIS["gravity-related height (H)",up],
LENGTHUNIT["metre",1.0]
"#;

const EXAMPLE13: &str = r#"CS[vertical,1],
AXIS["depth (D)",down,LENGTHUNIT["metre",1.0]]
"#;

const EXAMPLE14: &str = r#"CS[Cartesian,2],
AXIS["site north (x)",southEast,ORDER[1]],
AXIS["site east (y)",southWest,ORDER[2]],
LENGTHUNIT["metre",1.0]
"#; // TODO: I notice that in the spec it says `southEast` but here it is given as `southeast`, so case sensitive or not?
// For now I assume it *is* case sensitive and the example is wrong

const EXAMPLE15: &str = r#"CS[polar,2]
AXIS["distance (r)",awayFrom,ORDER[1],LENGTHUNIT["metre",1.0]],
AXIS["bearing (U)",clockwise,BEARING[234],ORDER[2],ANGLEUNIT["degree",0.017]]
"#;

const EXAMPLE16: &str = r#"CS[Cartesian,3],
AXIS["ahead (x)",forward,ORDER[1]],
AXIS["right (y)",starboard,ORDER[2]],
AXIS["down (z)",down,ORDER[3]],
LENGTHUNIT["metre",1.0]
"#;

const EXAMPLE17: &str = r#"CS[ordinal,2],
AXIS["Inline (I)",northEast,ORDER[1]],
AXIS["Crossline (J)",northWest,ORDER[2]]
"#;

#[test]
fn test_cs_unit() {
    test_example_1();
    test_example_2();
    test_example_3();
    test_example_4();
    test_example_5();
    test_example_6();
    test_example_7();
    test_example_8();
    test_example_9();
    test_example_10();
    test_example_11();
    test_example_12();
    test_example_13();
    test_example_14();
    test_example_15();
    test_example_16();
    test_example_17();
}

fn test_example_1() {
    let correct = CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
        spatial_cs_type: SpatialCsType::Ellipsoidal,
        dimension: Dimension::Two,
        identifier: None,
        spatial_axis: vec![
            SpatialAxis {
                axis_name_abbreviation: "latitude".to_string(),
                axis_direction: AxisDirection::North(None),
                axis_order: Some(AxisOrder { order: 1.0 }),
                identifier: None,
                spatial_unit: None,
            },
            SpatialAxis {
                axis_name_abbreviation: "longitude".to_string(),
                axis_direction: AxisDirection::East,
                axis_order: Some(AxisOrder { order: 2.0 }),
                identifier: None,
                spatial_unit: None,
            },
        ],
        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::AngleUnit(AngleUnit {
            unit_name: "degree".to_string(),
            conversion_factor: 0.0174,
            identifier: None,
        }))),
        needed_args: 4,
    });

    let ast = parse_wkt(EXAMPLE1);

    // TODO: Yes it's jank but right now this makes some sense. Already left todos
    // elsewhere as to implementing a TryFrom<&[WktNode]>
    let ast: Vec<WktArg> = ast.into_iter().map(|z| WktArg::Node(z)).collect();

    let cs = CoordinateSystem::try_from(ast.as_slice()).unwrap();

    assert_eq!(cs, correct);
}

fn test_example_2() {
    let correct = CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
        spatial_cs_type: SpatialCsType::Cartesian,
        dimension: Dimension::Three,
        identifier: None,
        spatial_axis: vec![
            SpatialAxis {
                axis_name_abbreviation: "(X)".to_string(),
                axis_direction: AxisDirection::GeocentricX,
                axis_order: None,
                identifier: None,
                spatial_unit: None,
            },
            SpatialAxis {
                axis_name_abbreviation: "(Y)".to_string(),
                axis_direction: AxisDirection::GeocentricY,
                axis_order: None,
                identifier: None,
                spatial_unit: None,
            },
            SpatialAxis {
                axis_name_abbreviation: "(Z)".to_string(),
                axis_direction: AxisDirection::GeocentricZ,
                axis_order: None,
                identifier: None,
                spatial_unit: None,
            },
        ],
        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
            unit_name: "metre".to_string(),
            conversion_factor: 1.0,
        }))),
        needed_args: 5,
    });

    let ast = parse_wkt(EXAMPLE2);

    let ast: Vec<WktArg> = ast.into_iter().map(|z| WktArg::Node(z)).collect();

    let cs = CoordinateSystem::try_from(ast.as_slice()).unwrap();

    assert_eq!(cs, correct);
}

fn test_example_3() {
    let correct = CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
        spatial_cs_type: SpatialCsType::Cartesian,
        dimension: Dimension::Three,
        identifier: None,
        spatial_axis: vec![
            SpatialAxis {
                axis_name_abbreviation: "(X)".to_string(),
                axis_direction: AxisDirection::East,
                axis_order: None,
                identifier: None,
                spatial_unit: None,
            },
            SpatialAxis {
                axis_name_abbreviation: "(Y)".to_string(),
                axis_direction: AxisDirection::North(None),
                axis_order: None,
                identifier: None,
                spatial_unit: None,
            },
            SpatialAxis {
                axis_name_abbreviation: "(Z)".to_string(),
                axis_direction: AxisDirection::Up,
                axis_order: None,
                identifier: None,
                spatial_unit: None,
            },
        ],
        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
            unit_name: "metre".to_string(),
            conversion_factor: 1.0,
        }))),
        needed_args: 5,
    });

    let ast = parse_wkt(EXAMPLE3);

    let ast: Vec<WktArg> = ast.into_iter().map(|z| WktArg::Node(z)).collect();

    let cs = CoordinateSystem::try_from(ast.as_slice()).unwrap();

    assert_eq!(cs, correct);
}

fn test_example_4() {
    let correct = CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
        spatial_cs_type: SpatialCsType::Spherical,
        dimension: Dimension::Three,
        identifier: None,
        spatial_axis: vec![
            SpatialAxis {
                axis_name_abbreviation: "distance (r)".to_string(),
                axis_direction: AxisDirection::AwayFrom,
                axis_order: Some(AxisOrder { order: 1.0 }),
                identifier: None,
                spatial_unit: Some(SpatialUnit::LengthUnit(LengthUnit {
                    unit_name: "kilometre".to_string(),
                    conversion_factor: 1000.0,
                })),
            },
            SpatialAxis {
                axis_name_abbreviation: "longitude (U)".to_string(),
                axis_direction: AxisDirection::CounterClockwise(Bearing { bearing: 0.0 }),
                axis_order: Some(AxisOrder { order: 2.0 }),
                identifier: None,
                spatial_unit: Some(SpatialUnit::AngleUnit(AngleUnit {
                    unit_name: "degree".into(),
                    conversion_factor: 0.017,
                    identifier: None,
                })),
            },
            SpatialAxis {
                axis_name_abbreviation: "elevation (V)".to_string(),
                axis_direction: AxisDirection::Up,
                axis_order: Some(AxisOrder { order: 3.0 }),
                identifier: None,
                spatial_unit: Some(SpatialUnit::AngleUnit(AngleUnit {
                    unit_name: "degree".to_string(),
                    conversion_factor: 0.017,
                    identifier: None,
                })),
            },
        ],
        cs_unit: None,
        needed_args: 4,
    });

    let ast = parse_wkt(EXAMPLE4);

    let ast: Vec<WktArg> = ast.into_iter().map(|z| WktArg::Node(z)).collect();

    let cs = CoordinateSystem::try_from(ast.as_slice()).unwrap();

    assert_eq!(cs, correct);
}

fn test_example_5() {
    let correct = CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
        spatial_cs_type: SpatialCsType::Ellipsoidal,
        dimension: Dimension::Three,
        identifier: None,
        spatial_axis: vec![
            SpatialAxis {
                axis_name_abbreviation: "latitude".to_string(),
                axis_direction: AxisDirection::North(None),
                axis_order: Some(AxisOrder { order: 1.0 }),
                identifier: None,
                spatial_unit: Some(SpatialUnit::AngleUnit(AngleUnit {
                    unit_name: "degree".to_string(),
                    conversion_factor: 0.017,
                    identifier: None,
                })),
            },
            SpatialAxis {
                axis_name_abbreviation: "longitude".to_string(),
                axis_direction: AxisDirection::East,
                axis_order: Some(AxisOrder { order: 2.0 }),
                identifier: None,
                spatial_unit: Some(SpatialUnit::AngleUnit(AngleUnit {
                    unit_name: "degree".to_string(),
                    conversion_factor: 0.017,
                    identifier: None,
                })),
            },
            SpatialAxis {
                axis_name_abbreviation: "ellipsoidal height (h)".to_string(),
                axis_direction: AxisDirection::Up,
                axis_order: Some(AxisOrder { order: 3.0 }),
                identifier: None,
                spatial_unit: Some(SpatialUnit::LengthUnit(LengthUnit {
                    unit_name: "metre".to_string(),
                    conversion_factor: 1.0,
                })),
            },
        ],
        cs_unit: None,
        needed_args: 4,
    });

    let ast = parse_wkt(EXAMPLE5);

    let ast: Vec<WktArg> = ast.into_iter().map(|z| WktArg::Node(z)).collect();

    let cs = CoordinateSystem::try_from(ast.as_slice()).unwrap();

    assert_eq!(cs, correct);
}

fn test_example_6() {
    let correct = CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
        spatial_cs_type: SpatialCsType::Ellipsoidal,
        dimension: Dimension::Two,
        identifier: None,
        spatial_axis: vec![
            SpatialAxis {
                axis_name_abbreviation: "(lat)".to_string(),
                axis_direction: AxisDirection::North(None),
                axis_order: None,
                identifier: None,
                spatial_unit: None,
            },
            SpatialAxis {
                axis_name_abbreviation: "(lon)".to_string(),
                axis_direction: AxisDirection::East,
                axis_order: None,
                identifier: None,
                spatial_unit: None,
            },
        ],
        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::AngleUnit(AngleUnit {
            unit_name: "degree".to_string(),
            conversion_factor: 0.017,
            identifier: None,
        }))),
        needed_args: 4,
    });

    let ast = parse_wkt(EXAMPLE6);

    let ast: Vec<WktArg> = ast.into_iter().map(|z| WktArg::Node(z)).collect();

    let cs = CoordinateSystem::try_from(ast.as_slice()).unwrap();

    assert_eq!(cs, correct);
}

fn test_example_7() {
    let correct = CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
        spatial_cs_type: SpatialCsType::Cartesian,
        dimension: Dimension::Two,
        identifier: None,
        spatial_axis: vec![
            SpatialAxis {
                axis_name_abbreviation: "(E)".to_string(),
                axis_direction: AxisDirection::East,
                axis_order: Some(AxisOrder { order: 1.0 }),
                identifier: None,
                spatial_unit: Some(SpatialUnit::LengthUnit(LengthUnit {
                    unit_name: "metre".to_string(),
                    conversion_factor: 1.0,
                })),
            },
            SpatialAxis {
                axis_name_abbreviation: "(N)".to_string(),
                axis_direction: AxisDirection::North(None),
                axis_order: Some(AxisOrder { order: 2.0 }),
                identifier: None,
                spatial_unit: Some(SpatialUnit::LengthUnit(LengthUnit {
                    unit_name: "metre".to_string(),
                    conversion_factor: 1.0,
                })),
            },
        ],
        cs_unit: None,
        needed_args: 3,
    });

    let ast = parse_wkt(EXAMPLE7);

    let ast: Vec<WktArg> = ast.into_iter().map(|z| WktArg::Node(z)).collect();

    let cs = CoordinateSystem::try_from(ast.as_slice()).unwrap();

    assert_eq!(cs, correct);
}

fn test_example_8() {
    let correct = CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
        spatial_cs_type: SpatialCsType::Cartesian,
        dimension: Dimension::Two,
        identifier: None,
        spatial_axis: vec![
            SpatialAxis {
                axis_name_abbreviation: "(E)".to_string(),
                axis_direction: AxisDirection::East,
                axis_order: Some(AxisOrder { order: 1.0 }),
                identifier: None,
                spatial_unit: None,
            },
            SpatialAxis {
                axis_name_abbreviation: "(N)".to_string(),
                axis_direction: AxisDirection::North(None),
                axis_order: Some(AxisOrder { order: 2.0 }),
                identifier: None,
                spatial_unit: None,
            },
        ],
        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
            unit_name: "metre".to_string(),
            conversion_factor: 1.0,
        }))),
        needed_args: 4,
    });

    let ast = parse_wkt(EXAMPLE8);

    let ast: Vec<WktArg> = ast.into_iter().map(|z| WktArg::Node(z)).collect();

    let cs = CoordinateSystem::try_from(ast.as_slice()).unwrap();

    assert_eq!(cs, correct);
}

fn test_example_9() {
    let correct = CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
        spatial_cs_type: SpatialCsType::Cartesian,
        dimension: Dimension::Two,
        identifier: None,
        spatial_axis: vec![
            SpatialAxis {
                axis_name_abbreviation: "northing (X)".to_string(),
                axis_direction: AxisDirection::North(None),
                axis_order: Some(AxisOrder { order: 1.0 }),
                identifier: None,
                spatial_unit: None,
            },
            SpatialAxis {
                axis_name_abbreviation: "easting (Y)".to_string(),
                axis_direction: AxisDirection::East,
                axis_order: Some(AxisOrder { order: 2.0 }),
                identifier: None,
                spatial_unit: None,
            },
        ],
        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
            unit_name: "German legal metre".to_string(),
            conversion_factor: 1.0000135965,
        }))),
        needed_args: 4,
    });

    let ast = parse_wkt(EXAMPLE9);

    let ast: Vec<WktArg> = ast.into_iter().map(|z| WktArg::Node(z)).collect();

    let cs = CoordinateSystem::try_from(ast.as_slice()).unwrap();

    assert_eq!(cs, correct);
}

fn test_example_10() {
    let correct = CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
        spatial_cs_type: SpatialCsType::Cartesian,
        dimension: Dimension::Two,
        identifier: None,
        spatial_axis: vec![
            SpatialAxis {
                axis_name_abbreviation: "easting (X)".to_string(),
                axis_direction: AxisDirection::South(Some(Meridian {
                    number: 90.0,
                    angle_unit: AngleUnit {
                        unit_name: "degree".to_string(),
                        conversion_factor: 0.017,
                        identifier: None,
                    },
                })),
                axis_order: Some(AxisOrder { order: 1.0 }),
                identifier: None,
                spatial_unit: None,
            },
            SpatialAxis {
                axis_name_abbreviation: "northing (Y)".to_string(),
                axis_direction: AxisDirection::South(Some(Meridian {
                    number: 180.0,
                    angle_unit: AngleUnit {
                        unit_name: "degree".to_string(),
                        conversion_factor: 0.017,
                        identifier: None,
                    },
                })),
                axis_order: Some(AxisOrder { order: 2.0 }),
                identifier: None,
                spatial_unit: None,
            },
        ],
        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
            unit_name: "metre".to_string(),
            conversion_factor: 1.0,
        }))),
        needed_args: 4,
    });

    let ast = parse_wkt(EXAMPLE10);

    let ast: Vec<WktArg> = ast.into_iter().map(|z| WktArg::Node(z)).collect();

    let cs = CoordinateSystem::try_from(ast.as_slice()).unwrap();

    assert_eq!(cs, correct);
}

fn test_example_11() {
    let correct = CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
        spatial_cs_type: SpatialCsType::Cartesian,
        dimension: Dimension::Three,
        identifier: None,
        spatial_axis: vec![
            SpatialAxis {
                axis_name_abbreviation: "(E)".to_string(),
                axis_direction: AxisDirection::East,
                axis_order: None,
                identifier: None,
                spatial_unit: None,
            },
            SpatialAxis {
                axis_name_abbreviation: "(N)".to_string(),
                axis_direction: AxisDirection::North(None),
                axis_order: None,
                identifier: None,
                spatial_unit: None,
            },
            SpatialAxis {
                axis_name_abbreviation: "ellipsoid height (h)".to_string(),
                axis_direction: AxisDirection::Up,
                axis_order: None,
                identifier: None,
                spatial_unit: None,
            },
        ],
        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
            unit_name: "metre".to_string(),
            conversion_factor: 1.0,
        }))),
        needed_args: 5,
    });

    let ast = parse_wkt(EXAMPLE11);

    let ast: Vec<WktArg> = ast.into_iter().map(|z| WktArg::Node(z)).collect();

    let cs = CoordinateSystem::try_from(ast.as_slice()).unwrap();

    assert_eq!(cs, correct);
}

fn test_example_12() {
    let correct = CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
        spatial_cs_type: SpatialCsType::Vertical,
        dimension: Dimension::One,
        identifier: None,
        spatial_axis: vec![SpatialAxis {
            axis_name_abbreviation: "gravity-related height (H)".to_string(),
            axis_direction: AxisDirection::Up,
            axis_order: None,
            identifier: None,
            spatial_unit: None,
        }],
        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
            unit_name: "metre".to_string(),
            conversion_factor: 1.0,
        }))),
        needed_args: 3,
    });

    let ast = parse_wkt(EXAMPLE12);

    let ast: Vec<WktArg> = ast.into_iter().map(|z| WktArg::Node(z)).collect();

    let cs = CoordinateSystem::try_from(ast.as_slice()).unwrap();

    assert_eq!(cs, correct);
}

fn test_example_13() {
    let correct = CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
        spatial_cs_type: SpatialCsType::Vertical,
        dimension: Dimension::One,
        identifier: None,
        spatial_axis: vec![SpatialAxis {
            axis_name_abbreviation: "depth (D)".to_string(),
            axis_direction: AxisDirection::Down,
            axis_order: None,
            identifier: None,
            spatial_unit: Some(SpatialUnit::LengthUnit(LengthUnit {
                unit_name: "metre".to_string(),
                conversion_factor: 1.0,
            })),
        }],
        cs_unit: None,
        needed_args: 2,
    });

    let ast = parse_wkt(EXAMPLE13);

    let ast: Vec<WktArg> = ast.into_iter().map(|z| WktArg::Node(z)).collect();

    let cs = CoordinateSystem::try_from(ast.as_slice()).unwrap();

    assert_eq!(cs, correct);
}

fn test_example_14() {
    let correct = CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
        spatial_cs_type: SpatialCsType::Cartesian,
        dimension: Dimension::Two,
        identifier: None,
        spatial_axis: vec![
            SpatialAxis {
                axis_name_abbreviation: "site north (x)".to_string(),
                axis_direction: AxisDirection::SouthEast,
                axis_order: Some(AxisOrder { order: 1.0 }),
                identifier: None,
                spatial_unit: None,
            },
            SpatialAxis {
                axis_name_abbreviation: "site east (y)".to_string(),
                axis_direction: AxisDirection::SouthWest,
                axis_order: Some(AxisOrder { order: 2.0 }),
                identifier: None,
                spatial_unit: None,
            },
        ],
        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
            unit_name: "metre".to_string(),
            conversion_factor: 1.0,
        }))),
        needed_args: 4,
    });

    let ast = parse_wkt(EXAMPLE14);

    let ast: Vec<WktArg> = ast.into_iter().map(|z| WktArg::Node(z)).collect();

    let cs = CoordinateSystem::try_from(ast.as_slice()).unwrap();

    assert_eq!(cs, correct);
}

fn test_example_15() {
    let correct = CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
        spatial_cs_type: SpatialCsType::Polar,
        dimension: Dimension::Two,
        identifier: None,
        spatial_axis: vec![
            SpatialAxis {
                axis_name_abbreviation: "distance (r)".to_string(),
                axis_direction: AxisDirection::AwayFrom,
                axis_order: Some(AxisOrder { order: 1.0 }),
                identifier: None,
                spatial_unit: Some(SpatialUnit::LengthUnit(LengthUnit {
                    unit_name: "metre".to_string(),
                    conversion_factor: 1.0,
                })),
            },
            SpatialAxis {
                axis_name_abbreviation: "bearing (U)".to_string(),
                axis_direction: AxisDirection::Clockwise(Bearing { bearing: 234.0 }),
                axis_order: Some(AxisOrder { order: 2.0 }),
                identifier: None,
                spatial_unit: Some(SpatialUnit::AngleUnit(AngleUnit {
                    unit_name: "degree".to_string(),
                    conversion_factor: 0.017,
                    identifier: None,
                })),
            },
        ],
        cs_unit: None,
        needed_args: 3,
    });

    let ast = parse_wkt(EXAMPLE15);

    let ast: Vec<WktArg> = ast.into_iter().map(|z| WktArg::Node(z)).collect();

    let cs = CoordinateSystem::try_from(ast.as_slice()).unwrap();

    assert_eq!(cs, correct);
}

fn test_example_16() {
    let correct = CoordinateSystem::SpatialCS(SpatialCoordinateSystem {
        spatial_cs_type: SpatialCsType::Cartesian,
        dimension: Dimension::Three,
        identifier: None,
        spatial_axis: vec![
            SpatialAxis {
                axis_name_abbreviation: "ahead (x)".to_string(),
                axis_direction: AxisDirection::Forward,
                axis_order: Some(AxisOrder { order: 1.0 }),
                identifier: None,
                spatial_unit: None,
            },
            SpatialAxis {
                axis_name_abbreviation: "right (y)".to_string(),
                axis_direction: AxisDirection::Starboard,
                axis_order: Some(AxisOrder { order: 2.0 }),
                identifier: None,
                spatial_unit: None,
            },
            SpatialAxis {
                axis_name_abbreviation: "down (z)".to_string(),
                axis_direction: AxisDirection::Down,
                axis_order: Some(AxisOrder { order: 3.0 }),
                identifier: None,
                spatial_unit: None,
            },
        ],
        cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(LengthUnit {
            unit_name: "metre".to_string(),
            conversion_factor: 1.0,
        }))),
        needed_args: 5,
    });

    let ast = parse_wkt(EXAMPLE16);

    let ast: Vec<WktArg> = ast.into_iter().map(|z| WktArg::Node(z)).collect();

    let cs = CoordinateSystem::try_from(ast.as_slice()).unwrap();

    assert_eq!(cs, correct);
}

fn test_example_17() {
    let correct = CoordinateSystem::OrdinalDateTimeCS(OrdinalDateTimeCoordinateSystem {
        ordinal_date_time_cs_type: OrdinalDateTimeCsType::Ordinal,
        dimension: Dimension::Two,
        identifier: None,
        ordinal_date_time_axis: vec![
            OrdinalDateTimeAxis {
                axis_name_abbreviation: "Inline (I)".to_string(),
                axis_direction: AxisDirection::NorthEast,
                axis_order: Some(AxisOrder { order: 1.0 }),
                identifier: None,
            },
            OrdinalDateTimeAxis {
                axis_name_abbreviation: "Crossline (J)".to_string(),
                axis_direction: AxisDirection::NorthWest,
                axis_order: Some(AxisOrder { order: 2.0 }),
                identifier: None,
            },
        ],
        needed_args: 3,
    });

    let ast = parse_wkt(EXAMPLE17);

    let ast: Vec<WktArg> = ast.into_iter().map(|z| WktArg::Node(z)).collect();

    let cs = CoordinateSystem::try_from(ast.as_slice()).unwrap();

    assert_eq!(cs, correct);
}
