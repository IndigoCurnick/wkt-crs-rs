use std::vec;

use horologium::Temporal;
use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

use crate::{
    ast::parse_wkt,
    coordinate_system::{
        AxisDirection, CoordinateSystem, Dimension, OrdinalDateTimeAxis,
        OrdinalDateTimeCoordinateSystem, OrdinalDateTimeCsType, TemporalCountMeasureAxis,
        TemporalCountMeasureCoordinateSystem, TemporalCountMeasureCsType,
    },
    scope_extent_identifier_remark::{DateOrString, ScopeExtentIdentifierRemark},
    temporal_crs::{
        temporal_crs::temporal_crs::TemporalCrs, temporal_datum::TemporalDatum,
        temporal_origin::TemporalOrigin,
    },
    units::TimeUnit,
};

// TODO: Uppercase on `temporalDateTime`
const EXAMPLE1: &str = r#"TIMECRS["DateTime",
    TDATUM["Gregorian Calendar"],
    CS[temporalDateTime,1],AXIS["Time (T)",future]
]
"#;

// TODO: Uppercase on `temporalCount`
const EXAMPLE2: &str = r#"TIMECRS["GPS milliseconds",
    TDATUM["GPS time origin",TIMEORIGIN[1980-01-01T00:00:00.0Z]],
    CS[temporalCount,1],AXIS["(T)",future,TIMEUNIT["millisecond (ms)",0.001]]
]
"#;

// TODO: Uppercase on `temporalCount`
const EXAMPLE3: &str = r#"TIMECRS["Calendar hours from 1979-12-29",
    TDATUM["29 December 1979",TIMEORIGIN[1979-12-29T00Z]],
    CS[temporalCount,1],AXIS["Time",future,TIMEUNIT["hour"]]
]
"#;

const EXAMPLE4: &str = r#"TIMECRS["Decimal Years CE",
    TDATUM["Common Era",TIMEORIGIN[0000]],
    CS[temporalMeasure,1],AXIS["Decimal Years (a)",future,TIMEUNIT["year"]]
]
"#;

const EXAMPLE5: &str = r#"TIMECRS["Unix time",
    TDATUM["Unix epoch",TIMEORIGIN[1970-01-01T00:00:00Z]],
    CS[temporalCount,1],AXIS["Time",future,TIMEUNIT["second"]]
]
"#;

#[test]
fn test_temporal_crs() {
    test_example_1();
    test_example_2();
    test_example_3();
    // test_example_4(); // TODO: The 0000 is really problematic!!!
    test_example_5();
}

fn test_example_1() {
    let correct = TemporalCrs {
        crs_name: "DateTime".into(),
        temporal_datum: TemporalDatum {
            datum_name: "Gregorian Calendar".into(),
            calendar: None,
            temporal_origin: None,
            identifier: None,
        },
        coordinate_system: CoordinateSystem::OrdinalDateTimeCS(OrdinalDateTimeCoordinateSystem {
            ordinal_date_time_cs_type: OrdinalDateTimeCsType::TemporalDateTime,
            dimension: Dimension::One,
            identifier: None,
            ordinal_date_time_axis: vec![OrdinalDateTimeAxis {
                axis_name_abbreviation: "Time (T)".to_string(),
                axis_direction: AxisDirection::Future,
                axis_order: None,
                identifier: None,
            }],
            needed_args: 2,
        }),
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: None,
            identifier: None,
            remark: None,
        },
    };

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let tcrs = TemporalCrs::try_from(&ast[0]).unwrap();

    assert_eq!(correct, tcrs);
}

fn test_example_2() {
    let correct = TemporalCrs {
        crs_name: "GPS milliseconds".into(),
        temporal_datum: TemporalDatum {
            datum_name: "GPS time origin".into(),
            calendar: None,
            temporal_origin: Some(TemporalOrigin(DateOrString::Date(
                Temporal::OffsetDateTime(OffsetDateTime::new_in_offset(
                    Date::from_calendar_date(1980, Month::January, 1).unwrap(),
                    Time::from_hms(0, 0, 0).unwrap(),
                    UtcOffset::from_hms(0, 0, 0).unwrap(),
                )),
            ))),
            identifier: None,
        },
        coordinate_system: CoordinateSystem::TemporalCountMeasureCS(
            TemporalCountMeasureCoordinateSystem {
                temporal_count_measure_cs_type: TemporalCountMeasureCsType::TemporalCount,
                dimension: Dimension::One,
                identifier: None,
                temporal_count_measure_axis: TemporalCountMeasureAxis {
                    axis_name_abbreviation: "(T)".into(),
                    axis_direction: AxisDirection::Future,
                    axis_order: None,
                    time_unit: Some(TimeUnit {
                        unit_name: "millisecond (ms)".to_string(),
                        conversion_factor: Some(0.001),
                        identifier: None,
                    }),
                    identifier: None,
                },
                needed_args: 2,
            },
        ),
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: None,
            identifier: None,
            remark: None,
        },
    };

    let ast = parse_wkt(EXAMPLE2);

    assert_eq!(ast.len(), 1);

    let tcrs = TemporalCrs::try_from(&ast[0]).unwrap();

    assert_eq!(correct, tcrs);
}

fn test_example_3() {
    let correct = TemporalCrs {
        crs_name: "Calendar hours from 1979-12-29".into(),
        temporal_datum: TemporalDatum {
            datum_name: "29 December 1979".into(),
            calendar: None,
            temporal_origin: Some(TemporalOrigin(DateOrString::Date(Temporal::DateTime(
                PrimitiveDateTime::new(
                    Date::from_calendar_date(1979, Month::December, 29).unwrap(),
                    Time::from_hms(0, 0, 0).unwrap(),
                ),
            )))),
            identifier: None,
        },
        coordinate_system: CoordinateSystem::TemporalCountMeasureCS(
            TemporalCountMeasureCoordinateSystem {
                temporal_count_measure_cs_type: TemporalCountMeasureCsType::TemporalCount,
                dimension: Dimension::One,
                identifier: None,
                temporal_count_measure_axis: TemporalCountMeasureAxis {
                    axis_name_abbreviation: "Time".into(),
                    axis_direction: AxisDirection::Future,
                    axis_order: None,
                    time_unit: Some(TimeUnit {
                        unit_name: "hour".to_string(),
                        conversion_factor: None,
                        identifier: None,
                    }),
                    identifier: None,
                },
                needed_args: 2,
            },
        ),
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: None,
            identifier: None,
            remark: None,
        },
    };

    let ast = parse_wkt(EXAMPLE3);

    assert_eq!(ast.len(), 1);

    let tcrs = TemporalCrs::try_from(&ast[0]).unwrap();

    assert_eq!(correct, tcrs);
}

fn test_example_4() {
    let correct = TemporalCrs {
        crs_name: "Decimal Years CE".into(),
        temporal_datum: TemporalDatum {
            datum_name: "Common Era".into(),
            calendar: None,
            temporal_origin: Some(TemporalOrigin(DateOrString::Date(Temporal::Date(
                Date::from_calendar_date(0, Month::January, 1).unwrap(),
            )))),
            identifier: None,
        },
        coordinate_system: CoordinateSystem::TemporalCountMeasureCS(
            TemporalCountMeasureCoordinateSystem {
                temporal_count_measure_cs_type: TemporalCountMeasureCsType::TemporalMeasure,
                dimension: Dimension::One,
                identifier: None,
                temporal_count_measure_axis: TemporalCountMeasureAxis {
                    axis_name_abbreviation: "Decimal years (a)".into(),
                    axis_direction: AxisDirection::Future,
                    axis_order: None,
                    time_unit: Some(TimeUnit {
                        unit_name: "year".to_string(),
                        conversion_factor: None,
                        identifier: None,
                    }),
                    identifier: None,
                },
                needed_args: 2,
            },
        ),
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: None,
            identifier: None,
            remark: None,
        },
    };

    let ast = parse_wkt(EXAMPLE4);

    assert_eq!(ast.len(), 1);

    let tcrs = TemporalCrs::try_from(&ast[0]).unwrap();

    assert_eq!(correct, tcrs);
}

fn test_example_5() {
    let correct = TemporalCrs {
        crs_name: "Unix time".into(),
        temporal_datum: TemporalDatum {
            datum_name: "Unix epoch".into(),
            calendar: None,
            temporal_origin: Some(TemporalOrigin(DateOrString::Date(
                Temporal::OffsetDateTime(OffsetDateTime::new_utc(
                    Date::from_calendar_date(1970, Month::January, 1).unwrap(),
                    Time::from_hms(0, 0, 0).unwrap(),
                )),
            ))),
            identifier: None,
        },
        coordinate_system: CoordinateSystem::TemporalCountMeasureCS(
            TemporalCountMeasureCoordinateSystem {
                temporal_count_measure_cs_type: TemporalCountMeasureCsType::TemporalCount,
                dimension: Dimension::One,
                identifier: None,
                temporal_count_measure_axis: TemporalCountMeasureAxis {
                    axis_name_abbreviation: "Time".into(),
                    axis_direction: AxisDirection::Future,
                    axis_order: None,
                    time_unit: Some(TimeUnit {
                        unit_name: "second".to_string(),
                        conversion_factor: None,
                        identifier: None,
                    }),
                    identifier: None,
                },
                needed_args: 2,
            },
        ),
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: None,
            identifier: None,
            remark: None,
        },
    };

    let ast = parse_wkt(EXAMPLE5);

    assert_eq!(ast.len(), 1);

    let tcrs = TemporalCrs::try_from(&ast[0]).unwrap();

    assert_eq!(correct, tcrs);
}
