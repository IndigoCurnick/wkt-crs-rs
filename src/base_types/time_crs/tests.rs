use std::vec;

use horologium::{Temporal, types::Year};
use time::{Date, Month, OffsetDateTime, Time, UtcOffset};

use crate::{
    ast::parse_wkt,
    base_types::{
        CoordinateSystem, OrdinalDateTimeAxis, OrdinalDateTimeCoordinateSystem,
        TemporalCountMeasureAxis, TemporalCountMeasureCoordinateSystem, TemporalDatum, TimeOrigin,
        TimeUnit, time_crs::time_crs::TimeCrs,
    },
    compound_types::ScopeExtentIdentifierRemark,
    data_types::DateOrString,
    enumerations::{AxisDirection, Dimension, OrdinalDateTimeCsType, TemporalCountMeasureCsType},
    types::WktBaseType,
};

const EXAMPLE1: &str = r#"TIMECRS["DateTime",
    TDATUM["Gregorian Calendar"],
    CS[TemporalDateTime,1],AXIS["Time (T)",future]
]
"#;

const EXAMPLE2: &str = r#"TIMECRS["GPS milliseconds",
    TDATUM["GPS time origin",TIMEORIGIN[1980-01-01T00:00:00.0Z]],
    CS[temporalCount,1],AXIS["(T)",future,TIMEUNIT["millisecond (ms)",0.001]]
]
"#;

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
    test_example_4();
    test_example_5();
}

fn test_example_1() {
    let correct = TimeCrs {
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
        }),
        scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
            usage: None,
            identifier: None,
            remark: None,
        },
    };

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let tcrs = TimeCrs::from_nodes(&ast).unwrap();

    assert_eq!(correct, tcrs.result);
}

fn test_example_2() {
    let correct = TimeCrs {
        crs_name: "GPS milliseconds".into(),
        temporal_datum: TemporalDatum {
            datum_name: "GPS time origin".into(),
            calendar: None,
            temporal_origin: Some(TimeOrigin(DateOrString::Date(Temporal::OffsetDateTime(
                OffsetDateTime::new_in_offset(
                    Date::from_calendar_date(1980, Month::January, 1).unwrap(),
                    Time::from_hms_milli(0, 0, 0, 0).unwrap(),
                    UtcOffset::from_hms(0, 0, 0).unwrap(),
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

    let tcrs = TimeCrs::from_nodes(&ast).unwrap();

    assert_eq!(correct, tcrs.result);
}

fn test_example_3() {
    let correct = TimeCrs {
        crs_name: "Calendar hours from 1979-12-29".into(),
        temporal_datum: TemporalDatum {
            datum_name: "29 December 1979".into(),
            calendar: None,
            temporal_origin: Some(TimeOrigin(DateOrString::Date(Temporal::OffsetDateTime(
                OffsetDateTime::new_in_offset(
                    Date::from_calendar_date(1979, Month::December, 29).unwrap(),
                    Time::from_hms(0, 0, 0).unwrap(),
                    UtcOffset::from_hms(0, 0, 0).unwrap(),
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

    let tcrs = TimeCrs::from_nodes(&ast).unwrap();

    assert_eq!(correct, tcrs.result);
}

fn test_example_4() {
    let correct = TimeCrs {
        crs_name: "Decimal Years CE".into(),
        temporal_datum: TemporalDatum {
            datum_name: "Common Era".into(),
            calendar: None,
            temporal_origin: Some(TimeOrigin(DateOrString::Date(Temporal::Year(Year(0))))),
            identifier: None,
        },
        coordinate_system: CoordinateSystem::TemporalCountMeasureCS(
            TemporalCountMeasureCoordinateSystem {
                temporal_count_measure_cs_type: TemporalCountMeasureCsType::TemporalMeasure,
                dimension: Dimension::One,
                identifier: None,
                temporal_count_measure_axis: TemporalCountMeasureAxis {
                    axis_name_abbreviation: "Decimal Years (a)".into(),
                    axis_direction: AxisDirection::Future,
                    axis_order: None,
                    time_unit: Some(TimeUnit {
                        unit_name: "year".to_string(),
                        conversion_factor: None,
                        identifier: None,
                    }),
                    identifier: None,
                },
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

    let tcrs = TimeCrs::from_nodes(&ast).unwrap();

    assert_eq!(correct, tcrs.result);
}

fn test_example_5() {
    let correct = TimeCrs {
        crs_name: "Unix time".into(),
        temporal_datum: TemporalDatum {
            datum_name: "Unix epoch".into(),
            calendar: None,
            temporal_origin: Some(TimeOrigin(DateOrString::Date(Temporal::OffsetDateTime(
                OffsetDateTime::new_utc(
                    Date::from_calendar_date(1970, Month::January, 1).unwrap(),
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
                        unit_name: "second".to_string(),
                        conversion_factor: None,
                        identifier: None,
                    }),
                    identifier: None,
                },
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

    let tcrs = TimeCrs::from_nodes(&ast).unwrap();

    assert_eq!(correct, tcrs.result);
}
