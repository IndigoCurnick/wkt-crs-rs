use horologium::Temporal;
use time::{Date, Month};

use crate::{ast::parse_wkt, data_types::DateOrString, types::WktBaseType};

use super::temporal_extent::TemporalExtent;

const EXAMPLE1: &str = "TIMEEXTENT[2013-01-01,2013-12-31]";
const EXAMPLE2: &str = r#"TIMEEXTENT["Jurassic","Quaternary"]"#;

#[test]
fn test_time_extent() {
    // Example 1

    let correct = TemporalExtent {
        from: DateOrString::Date(Temporal::CalendarDay(
            Date::from_calendar_date(2013, Month::January, 1).unwrap(),
        )),
        to: DateOrString::Date(Temporal::CalendarDay(
            Date::from_calendar_date(2013, Month::December, 31).unwrap(),
        )),
    };

    let ast = parse_wkt(EXAMPLE1);
    let time = TemporalExtent::from_nodes(&ast).unwrap();

    assert_eq!(time.result, correct);
    assert_eq!(time.consumed, 1);

    // Example 2

    let correct = TemporalExtent {
        from: DateOrString::String("Jurassic".to_string()),
        to: DateOrString::String("Quaternary".to_string()),
    };

    let ast = parse_wkt(EXAMPLE2);

    let time = TemporalExtent::from_nodes(&ast).unwrap();

    assert_eq!(time.result, correct);
    assert_eq!(time.consumed, 1);
}
