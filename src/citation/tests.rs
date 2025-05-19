// ! I was unable to find an example of a citation in the documentation
use crate::{ast::parse_wkt, citation::citation::Citation};

const EXAMPLE1: &str = r#"CITATION["some-citation"]"#;

#[test]
fn test_citation() {
    let correct = Citation("some-citation".to_string());

    let ast = parse_wkt(EXAMPLE1);

    let citation = Citation::try_from(&ast).unwrap();

    assert_eq!(citation, correct);
}
