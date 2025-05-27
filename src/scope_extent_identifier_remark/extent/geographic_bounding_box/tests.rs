use crate::ast::parse_wkt;

use super::geographic_bounding_box::GeographicBoundingBox;

const EXAMPLE1: &str = "BBOX[51.43,2.54,55.77,6.40]";
const EXAMPLE2: &str = "BBOX[-55.95,160.60,-25.88,-171.20]";

#[test]
fn test_gbbox() {
    // Example 1
    let correct = GeographicBoundingBox {
        lower_left_latitude: 51.43,
        lower_left_longitude: 2.54,
        upper_right_latitude: 55.77,
        upper_right_longitude: 6.40,
    };

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let bbox = GeographicBoundingBox::try_from(&ast[0]).unwrap();

    assert_eq!(correct, bbox);

    // Example 2
    let correct = GeographicBoundingBox {
        lower_left_latitude: -55.95,
        lower_left_longitude: 160.60,
        upper_right_latitude: -25.88,
        upper_right_longitude: -171.20,
    };

    let ast = parse_wkt(EXAMPLE2);

    assert_eq!(ast.len(), 1);

    let bbox = GeographicBoundingBox::try_from(&ast[0]).unwrap();

    assert_eq!(correct, bbox);
}
