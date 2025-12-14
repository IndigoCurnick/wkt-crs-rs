use wkt_crs_rs::{
    WktCrsTypes,
    base_types::{DatumAnchor, ParametricDatum},
    parse_wkt_crs,
};

const EXAMPLE: &str = r#"PDATUM["Mean Sea Level",ANCHOR["1013.25 hPa at 15C"]]"#;

#[test]
fn test_parametric_datum() {
    let correct = ParametricDatum {
        datum_name: "Mean Sea Level".to_string(),
        datum_anchor: Some(DatumAnchor("1013.25 hPa at 15C".to_string())),
        identifier: None,
    };

    let correct = vec![WktCrsTypes::ParametricDatum(correct)];

    let ast = parse_wkt_crs(EXAMPLE).unwrap();

    assert_eq!(correct, ast);
}
