use crate::{
    WktBaseType,
    ast::parse_wkt,
    base_types::{Id, OperationMethod},
    data_types::NumText,
};

const EXAMPLE: &str = r#"METHOD["NADCON",ID["EPSG",9613]]"#;

#[test]
fn test_operation_method() {
    let correct = OperationMethod {
        operation_method_name: "NADCON".into(),
        identifier: Some(Id {
            authority_name: "EPSG".into(),
            authority_unique_identifier: NumText::Int(9613),
            version: None,
            authority_citation: None,
            id_uri: None,
        }),
    };

    let ast = parse_wkt(EXAMPLE);

    let op = OperationMethod::from_nodes(&ast).unwrap();

    assert_eq!(correct, op.result);
    assert_eq!(op.consumed, 1);
}
