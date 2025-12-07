use wkt_crs_rs::{WktCrsTypes, base_types::Bearing, parse_wkt_crs};

const EXAMPLE1: &str = "BEARING[0]";

#[test]
fn test_bearing() {
    let correct = vec![WktCrsTypes::Bearing(Bearing(0.0))];

    let ast = parse_wkt_crs(EXAMPLE1).unwrap();

    assert_eq!(correct, ast);
}
