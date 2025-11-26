use wkt_crs_rs::{WktCrsTypes, base_types::AngleUnit, parse_wkt_crs};

const AU_EXAMPLE1: &str = r#"ANGLEUNIT["degree",0.0174]"#;

#[test]
fn test_angle_unit() {
    let correct = vec![WktCrsTypes::AngleUnit(AngleUnit {
        conversion_factor: 0.0174,
        identifier: None,
        unit_name: "degree".to_string(),
    })];

    let wkt = parse_wkt_crs(AU_EXAMPLE1).unwrap();

    assert_eq!(correct, wkt);
}
