use crate::keywords::Keywords;

use super::ast::{Token, WktArg, WktNode, parse_wkt, tokenize};

const EXAMPLE1: &str = r#"LENGTHUNIT["metre",1]"#;

#[test]
fn test_tokenise() {
	let correct = vec![
		Token::Keyword(Keywords::LengthUnit),
		Token::LDelimiter,
		Token::Data("metre".to_string()),
		Token::WktSeparator,
		Token::Data("1".to_string()),
		Token::RDelimiter,
	];

	let tokens = tokenize(EXAMPLE1);

	assert_eq!(correct, tokens);
}

#[test]
fn test_parse_wkt() {
	let correct = WktNode {
		keyword: Keywords::LengthUnit,
		args: vec![
			WktArg::Data("metre".to_string()),
			WktArg::Data("1".to_string()),
		],
	};

	let node = parse_wkt(EXAMPLE1).unwrap();

	assert_eq!(correct, node[0]);
	assert_eq!(node.len(), 1);
}
