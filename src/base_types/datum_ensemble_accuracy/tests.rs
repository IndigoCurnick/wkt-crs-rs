use crate::{
	ast::parse_wkt, base_types::datum_ensemble_accuracy::DatumEnsembleAccuracy,
	types::WktBaseType,
};

const EXAMPLE1: &str = "ENSEMBLEACCURACY[2]";

#[test]
fn test_ensemble_acc() {
	let correct = DatumEnsembleAccuracy(2.0);

	let ast = parse_wkt(EXAMPLE1).unwrap();

	assert_eq!(ast.len(), 1);

	let acc = DatumEnsembleAccuracy::from_nodes(&ast).unwrap();

	assert_eq!(correct, acc.result);
}
