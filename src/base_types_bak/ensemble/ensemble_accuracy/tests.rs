use crate::ast::parse_wkt;

use super::ensemble_accuracy::EnsembleAccuracy;

const EXAMPLE1: &str = "ENSEMBLEACCURACY[2]";

#[test]
fn test_ensemble_acc() {
    let correct = EnsembleAccuracy(2.0);

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let acc = EnsembleAccuracy::try_from(&ast[0]).unwrap();

    assert_eq!(correct, acc);
}
