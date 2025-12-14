use wkt_crs_rs::{WktCrsTypes, base_types::OperationAccuracy, parse_wkt_crs};

const EXAMPLE: &str = "OPERATIONACCURACY[5]";

#[test]
fn test_operation_accuracy() {
    let correct = vec![WktCrsTypes::OperationAccuracy(OperationAccuracy(5.0))];

    let ast = parse_wkt_crs(EXAMPLE).unwrap();

    assert_eq!(correct, ast);
}
