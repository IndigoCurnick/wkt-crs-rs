use horologium::Temporal;
use time::{Date, Month};
use wkt_crs_rs::{
    WktCrsTypes,
    base_types::{Calendar, TemporalDatum, TimeOrigin},
    data_types::DateOrString,
    parse_wkt_crs,
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
        temporal_origin: Some(TimeOrigin(DateOrString::Date(Temporal::CalendarDay(
            Date::from_calendar_date(0, Month::January, 1).unwrap(),
        )))),
        identifier: None,
    };

    let correct = vec![WktCrsTypes::TemporalDatum(correct)];

    let ast = parse_wkt_crs(EXAMPLE1).unwrap();

    assert_eq!(ast, correct);
}

fn test_example_2() {
    let correct = TemporalDatum {
        datum_name: "Gregorian Calendar".to_string(),
        calendar: None,
        temporal_origin: Some(TimeOrigin(DateOrString::String("0001 January 1st".into()))),
        identifier: None,
    };

    let correct = vec![WktCrsTypes::TemporalDatum(correct)];

    let ast = parse_wkt_crs(EXAMPLE2).unwrap();

    assert_eq!(ast, correct);
}

fn test_example_3() {
    let correct = TemporalDatum {
        datum_name: "Gregorian Calendar".to_string(),
        calendar: None,
        temporal_origin: None,
        identifier: None,
    };

    let correct = vec![WktCrsTypes::TemporalDatum(correct)];

    let ast = parse_wkt_crs(EXAMPLE3).unwrap();

    assert_eq!(ast, correct);
}
