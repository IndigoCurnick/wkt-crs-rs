use crate::ast::parse_wkt;

use super::Bearing;

const EXAMPLE1: &str = "BEARING[0]";

#[test]
fn test_bearing() {
    let correct = Bearing { bearing: 0.0 };

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let bearing = Bearing::try_from(&ast[0]).unwrap();

    assert_eq!(correct, bearing);
}
