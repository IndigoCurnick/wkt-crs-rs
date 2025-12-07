use wkt_crs_rs::{WktCrsTypes, base_types::GeographicBoundingBox, parse_wkt_crs};

const EXAMPLE1: &str = "BBOX[51.43,2.54,55.77,6.40]";
const EXAMPLE2: &str = "BBOX[-55.95,160.60,-25.88,-171.20]";

#[test]
fn test_gbbox() {
    test_example_1();
    test_example_2();
}

fn test_example_1() {
    // Example 1
    let correct = vec![WktCrsTypes::GeographicBoundingBox(GeographicBoundingBox {
        lower_left_latitude: 51.43,
        lower_left_longitude: 2.54,
        upper_right_latitude: 55.77,
        upper_right_longitude: 6.40,
    })];

    let ast = parse_wkt_crs(EXAMPLE1).unwrap();

    assert_eq!(correct, ast);
}

fn test_example_2() {
    // Example 2
    let correct = vec![WktCrsTypes::GeographicBoundingBox(GeographicBoundingBox {
        lower_left_latitude: -55.95,
        lower_left_longitude: 160.60,
        upper_right_latitude: -25.88,
        upper_right_longitude: -171.20,
    })];

    let ast = parse_wkt_crs(EXAMPLE2).unwrap();

    assert_eq!(correct, ast);
}
