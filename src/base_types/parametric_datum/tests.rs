use crate::{
    WktBaseType,
    ast::parse_wkt,
    base_types::{DatumAnchor, ParametricDatum},
};

const EXAMPLE: &str = r#"PDATUM["Mean Sea Level",ANCHOR["1013.25 hPa at 15C"]]"#;

#[test]
fn test_parametric_datum() {
    let correct = ParametricDatum {
        datum_name: "Mean Sea Level".to_string(),
        datum_anchor: Some(DatumAnchor("1013.25 hPa at 15C".to_string())),
        identifier: None,
    };

    let ast = parse_wkt(EXAMPLE);

    assert_eq!(ast.len(), 1);

    let para = ParametricDatum::from_nodes(&ast).unwrap();

    assert_eq!(correct, para.result);
}
