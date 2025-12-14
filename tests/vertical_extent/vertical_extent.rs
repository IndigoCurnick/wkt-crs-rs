use wkt_crs_rs::{
    WktCrsTypes,
    base_types::{LengthUnit, VerticalExtent},
    parse_wkt_crs,
};

const EXAMPLE1: &str = r#"VERTICALEXTENT[-1000,0,LENGTHUNIT["metre",1.0]]"#;
const EXAMPLE2: &str = "VERTICALEXTENT[-1000,0]";

#[test]
fn test_vertical_extent() {
    test_example_1();
    test_example_2();
}

fn test_example_1() {
    let correct = VerticalExtent {
        minimum_height: -1000.0,
        maximum_height: 0.0,
        length_unit: Some(LengthUnit {
            unit_name: "metre".to_string(),
            conversion_factor: 1.0,
        }),
    };

    let correct = vec![WktCrsTypes::VerticalExtent(correct)];

    let ast = parse_wkt_crs(EXAMPLE1).unwrap();

    assert_eq!(ast, correct);
}

fn test_example_2() {
    let correct = VerticalExtent {
        minimum_height: -1000.0,
        maximum_height: 0.0,
        length_unit: None,
    };

    let correct = vec![WktCrsTypes::VerticalExtent(correct)];

    let ast = parse_wkt_crs(EXAMPLE2).unwrap();

    assert_eq!(ast, correct);
}
