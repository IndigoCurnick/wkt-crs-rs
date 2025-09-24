use crate::{ast::parse_wkt, base_types::order::Order, types::WktBaseType};

const EXAMPLE1: &str = "ORDER[0]";

#[test]
fn test_order() {
    let correct = Order(0);

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let order = Order::from_nodes(&ast).unwrap();

    assert_eq!(correct, order.result);
}
