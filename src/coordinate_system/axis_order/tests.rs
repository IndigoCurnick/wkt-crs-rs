use crate::{ast::parse_wkt, coordinate_system::axis_order::AxisOrder};

const EXAMPLE1: &str = "ORDER[0]";

#[test]
fn test_order() {
    let correct = AxisOrder { order: 0.0 };

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let order = AxisOrder::try_from(&ast[0]).unwrap();

    assert_eq!(correct, order);
}
