use crate::{ast::parse_wkt, units::LengthUnit};

use super::vertical_extent::VerticalExtent;

const EXAMPLE1: &str = r#"VERTICALEXTENT[-1000,0,LENGTHUNIT["metre",1.0]]"#;
const EXAMPLE2: &str = "VERTICALEXTENT[-1000,0]";

#[test]
fn test_vertical_extent() {
    // Example 1

    let correct = VerticalExtent {
        minimum_height: -1000.0,
        maximum_height: 0.0,
        length_unit: Some(LengthUnit {
            unit_name: "metre".to_string(),
            conversion_factor: 1.0,
        }),
    };

    let ast = parse_wkt(EXAMPLE1);

    let vert = VerticalExtent::try_from(&ast).unwrap();

    assert_eq!(vert, correct);

    // Example 1

    let correct = VerticalExtent {
        minimum_height: -1000.0,
        maximum_height: 0.0,
        length_unit: None,
    };

    let ast = parse_wkt(EXAMPLE2);

    let vert = VerticalExtent::try_from(&ast).unwrap();

    assert_eq!(vert, correct);
}
