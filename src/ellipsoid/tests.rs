use crate::{ast::parse_wkt, units::LengthUnit};

use super::ellipsoid::Ellipsoid;

const EXAMPLE1: &str = r#"ELLIPSOID["GRS 1980",6378132,298.257222101,LENGTHUNIT["metre",1]]"#;
const EXAMPLE2: &str = r#"SPHEROID["GRS 1980",6378132,298.257222101]"#;

#[test]
fn test_ellipsoid() {
    // EXAMPLE 1

    let correct = Ellipsoid {
        ellipsoid_name: "GRS 1980".to_string(),
        semi_major_axis: 6378132.0,
        inverse_flattening: 298.257222101,
        length_unit: Some(LengthUnit {
            conversion_factor: 1.0,
            unit_name: "metre".to_string(),
        }),
    };

    let ast = parse_wkt(EXAMPLE1);

    let el = Ellipsoid::try_from(&ast).unwrap();

    assert_eq!(el, correct);

    // EXAMPLE 2
    let correct = Ellipsoid {
        ellipsoid_name: "GRS 1980".to_string(),
        semi_major_axis: 6378132.0,
        inverse_flattening: 298.257222101,
        length_unit: None,
    };

    let ast = parse_wkt(EXAMPLE2);

    let el = Ellipsoid::try_from(&ast).unwrap();

    assert_eq!(el, correct);
}
