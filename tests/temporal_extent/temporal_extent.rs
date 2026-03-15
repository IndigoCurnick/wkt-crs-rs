use horologium::Temporal;
use time::{Date, Month};
use wkt_crs_rs::{
	WktCrsTypes, base_types::TemporalExtent, data_types::DateOrString,
	parse_wkt_crs,
};

const EXAMPLE1: &str = "TIMEEXTENT[2013-01-01,2013-12-31]";
const EXAMPLE2: &str = r#"TIMEEXTENT["Jurassic","Quaternary"]"#;

#[test]
fn test_time_extent() {
	test_example_1();
	test_example_2();
}

fn test_example_1() {
	let correct = TemporalExtent {
		from: DateOrString::Date(Temporal::CalendarDay(
			Date::from_calendar_date(2013, Month::January, 1).unwrap(),
		)),
		to: DateOrString::Date(Temporal::CalendarDay(
			Date::from_calendar_date(2013, Month::December, 31).unwrap(),
		)),
	};

	let correct = vec![WktCrsTypes::TemporalExtent(correct)];

	let ast = parse_wkt_crs(EXAMPLE1).unwrap();

	assert_eq!(ast, correct);
}

fn test_example_2() {
	let correct = TemporalExtent {
		from: DateOrString::String("Jurassic".to_string()),
		to: DateOrString::String("Quaternary".to_string()),
	};

	let correct = vec![WktCrsTypes::TemporalExtent(correct)];

	let ast = parse_wkt_crs(EXAMPLE2).unwrap();

	assert_eq!(ast, correct);
}
