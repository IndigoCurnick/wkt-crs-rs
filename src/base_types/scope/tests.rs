use crate::{ast::parse_wkt, types::WktBaseType};

use super::Scope;

const EXAMPLE1: &str = r#"SCOPE["Large scale topographic mapping and cadastre."]"#;

#[test]
fn test_scope() {
    let correct = Scope("Large scale topographic mapping and cadastre.".to_string());

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let scope = Scope::from_nodes(&ast).unwrap();

    assert_eq!(scope.consumed, 1);
    assert_eq!(scope.result, correct);
}
