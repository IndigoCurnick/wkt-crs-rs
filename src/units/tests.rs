use crate::{ast::parse_wkt, units::ScaleUnit};

use super::{LengthUnit, angle_unit::AngleUnit};

const LU_EXAMPLE1: &str = r#"LENGTHUNIT["metre",1]"#;

const AU_EXAMPLE1: &str = r#"ANGLEUNIT["degree",0.0174]"#;

const SU_EXAMPLE1: &str = r#"SCALEUNIT["parts per million",1E-06]"#;

#[test]
fn test_parse_length_unit() {
    let correct = LengthUnit {
        unit_name: "metre".to_string(),
        conversion_factor: 1.0,
    };

    let ast = parse_wkt(LU_EXAMPLE1);

    let lu = LengthUnit::try_from(&ast).unwrap();

    assert_eq!(correct, lu);
}

#[test]
fn test_angle_unit() {
    let correct = AngleUnit {
        conversion_factor: 0.0174,
        identifier: None,
        unit_name: "degree".to_string(),
    };

    let ast = parse_wkt(AU_EXAMPLE1);

    let au = AngleUnit::try_from(&ast).unwrap();

    assert_eq!(correct, au);
}

#[test]
fn test_scale_unit() {
    let correct = ScaleUnit {
        conversion_factor: 1e-6,
        identifier: None,
        unit_name: "parts per million".to_string(),
    };

    let ast = parse_wkt(SU_EXAMPLE1);

    let su = ScaleUnit::try_from(&ast).unwrap();

    assert_eq!(correct, su);
}
