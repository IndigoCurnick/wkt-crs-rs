use horologium::Temporal;
use time::{Date, Month, OffsetDateTime, Time, UtcOffset};
use wkt_crs_rs::{
	WktCrsTypes, base_types::TimeOrigin, data_types::DateOrString,
	parse_wkt_crs,
};

const EXAMPLE: &str = r#"TIMEORIGIN[1980-01-01T00:00:00.0Z]"#;

#[test]
fn test_time_origin() {
	let correct = TimeOrigin(DateOrString::Date(Temporal::OffsetDateTime(
		OffsetDateTime::new_in_offset(
			Date::from_calendar_date(1980, Month::January, 1).unwrap(),
			Time::from_hms_milli(0, 0, 0, 0).unwrap(),
			UtcOffset::from_hms(0, 0, 0).unwrap(),
		),
	)));

	let correct = vec![WktCrsTypes::TimeOrigin(correct)];

	let ast = parse_wkt_crs(EXAMPLE).unwrap();

	assert_eq!(correct, ast);
}
