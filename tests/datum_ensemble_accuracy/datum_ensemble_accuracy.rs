use wkt_crs_rs::{WktCrsTypes, base_types::DatumEnsembleAccuracy, parse_wkt_crs};

const EXAMPLE1: &str = "ENSEMBLEACCURACY[2]";

#[test]
fn test_ensemble_acc() {
    let correct = vec![WktCrsTypes::DatumEnsembleAccuracy(DatumEnsembleAccuracy(
        2.0,
    ))];

    let ast = parse_wkt_crs(EXAMPLE1).unwrap();

    assert_eq!(correct, ast);
}
