use super::ast::{Token, WktArg, WktNode, parse_node, parse_wkt, tokenize};

const EXAMPLE1: &str = r#"LENGTHUNIT["metre",1]"#;

#[test]
fn test_tokenise() {
    let correct = vec![
        Token::Ident("LENGTHUNIT".to_string()),
        Token::LBracket,
        Token::String("metre".to_string()),
        Token::Comma,
        Token::Number(1.0),
        Token::RBracket,
    ];

    let tokens = tokenize(EXAMPLE1);

    assert_eq!(correct, tokens);
}

#[test]
fn test_parse_wkt() {
    let correct = WktNode {
        keyword: "LENGTHUNIT".to_string(),
        args: vec![WktArg::String("metre".to_string()), WktArg::Number(1.0)],
    };

    let node = parse_wkt(EXAMPLE1);

    assert_eq!(correct, node[0]);
    assert_eq!(node.len(), 1);
}
