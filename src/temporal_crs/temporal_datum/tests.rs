use horologium::Temporal;
use time::{Date, Month};

use crate::{
    ast::parse_wkt,
    scope_extent_identifier_remark::DateOrString,
    temporal_crs::{
        calendar::Calendar, temporal_datum::TemporalDatum, temporal_origin::TemporalOrigin,
    },
};

pub const EXAMPLE1: &str = r#"TIMEDATUM["Gregorian Calendar",CALENDAR["proleptic Gregorian"],
    TIMEORIGIN[0000-01-01]]
"#;

pub const EXAMPLE2: &str = r#"TDATUM["Gregorian Calendar",TIMEORIGIN["0001 January 1st"]]
"#;

pub const EXAMPLE3: &str = r#"TDATUM["Gregorian Calendar"]"#;

#[test]
fn test_temporal_datum() {
    test_example_1();
    test_example_2();
    test_example_3();
}

fn test_example_1() {
    let correct = TemporalDatum {
        datum_name: "Gregorian Calendar".to_string(),
        calendar: Some(Calendar("proleptic Gregorian".to_string())),
        temporal_origin: Some(TemporalOrigin(DateOrString::Date(Temporal::Date(
            Date::from_calendar_date(0, Month::January, 1).unwrap(),
        )))),
        identifier: None,
    };

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let td = TemporalDatum::try_from(&ast[0]).unwrap();
    assert_eq!(td, correct);
}

fn test_example_2() {
    let correct = TemporalDatum {
        datum_name: "Gregorian Calendar".to_string(),
        calendar: None,
        temporal_origin: Some(TemporalOrigin(DateOrString::String(
            "0001 January 1st".into(),
        ))),
        identifier: None,
    };

    let ast = parse_wkt(EXAMPLE2);

    assert_eq!(ast.len(), 1);

    let td = TemporalDatum::try_from(&ast[0]).unwrap();
    assert_eq!(td, correct);
}

fn test_example_3() {
    let correct = TemporalDatum {
        datum_name: "Gregorian Calendar".to_string(),
        calendar: None,
        temporal_origin: None,
        identifier: None,
    };

    let ast = parse_wkt(EXAMPLE3);

    assert_eq!(ast.len(), 1);

    let td = TemporalDatum::try_from(&ast[0]).unwrap();
    assert_eq!(td, correct);
}
