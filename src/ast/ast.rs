use core::num;

use horologium::Temporal;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    String(String),
    Number(f64),
    DateTime(Temporal),
    LBracket,
    RBracket,
    Comma,
}

pub fn tokenize(mut s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    while let Some(c) = s.chars().next() {
        match c {
            '[' => {
                tokens.push(Token::LBracket);
                s = &s[1..];
            }
            ']' => {
                tokens.push(Token::RBracket);
                s = &s[1..];
            }
            ',' => {
                tokens.push(Token::Comma);
                s = &s[1..];
            }
            '"' => {
                let end = s[1..].find('"').unwrap() + 1;
                let content = &s[1..end];
                tokens.push(Token::String(content.to_string()));
                s = &s[end + 1..];
            }
            c if c.is_ascii_digit() || c == '-' || c == '.' => {
                let len = s
                    .find(|ch: char| {
                        !ch.is_ascii_digit()
                            && ch != '.'
                            && ch != '-'
                            && ch != '+'
                            && ch != 'e'
                            && ch != 'E'
                    }) // TODO: we might need more robust scientific notation handling in future
                    .unwrap_or(s.len());
                let num_str = &s[..len];

                // ! This is some premium jank
                // TODO: Integer support?
                if let Ok(num) = num_str.parse::<f64>() {
                    tokens.push(Token::Number(num));
                    s = &s[len..];
                } else if let Ok(date) = Temporal::try_from(num_str) {
                    tokens.push(Token::DateTime(date));
                    s = &s[len..];
                } else {
                    panic!("Unknown number type - `{}`", num_str)
                };
            }
            c if c.is_ascii_alphabetic() => {
                let len = s
                    .find(|ch: char| !ch.is_ascii_alphabetic())
                    .unwrap_or(s.len());
                let ident = &s[..len];

                // All uppercase is a keyword. Lowercase strings without quotes
                // indicate essentially enums, like `ellipsoidal` to delineate
                // CS type
                if is_all_upper(ident) {
                    tokens.push(Token::Ident(ident.to_string()));
                } else {
                    tokens.push(Token::String(ident.to_string()));
                }

                s = &s[len..];
            }
            c if c.is_whitespace() => {
                s = &s[1..];
            }
            _ => panic!("unhandled char {:?}", c),
        }
    }

    tokens
}

fn is_all_upper(s: &str) -> bool {
    s.chars().any(|c| c.is_alphabetic()) && s.chars().all(|c| !c.is_lowercase())
}

fn parse_nodes(tokens: &mut Vec<Token>) -> Vec<WktNode> {
    let mut nodes = Vec::new();

    loop {
        match tokens.first() {
            Some(Token::Ident(_)) => {
                nodes.push(parse_node(tokens));
            }
            Some(Token::Comma) => {
                tokens.remove(0);
            }
            None => {
                break;
            }
            _ => panic!("Unexpected token"),
        }
    }
    while let Some(Token::Ident(_)) = tokens.first() {
        nodes.push(parse_node(tokens));
    }
    nodes
}

pub fn parse_node(tokens: &mut Vec<Token>) -> WktNode {
    let keyword = match tokens.remove(0) {
        Token::Ident(s) => s,
        _ => panic!("expected keyword"),
    };

    assert!(matches!(tokens.remove(0), Token::LBracket));

    let mut args = Vec::new();
    loop {
        match tokens.first() {
            Some(Token::RBracket) => {
                tokens.remove(0);
                break;
            }
            Some(Token::Comma) => {
                tokens.remove(0);
            }
            Some(Token::String(s)) => {
                args.push(WktArg::String(s.clone()));
                tokens.remove(0);
            }
            Some(Token::Number(n)) => {
                args.push(WktArg::Number(*n));
                tokens.remove(0);
            }
            Some(Token::DateTime(d)) => {
                args.push(WktArg::DateTime(d.clone()));
                tokens.remove(0);
            }
            Some(Token::Ident(_)) => {
                let node = parse_node(tokens);
                args.push(WktArg::Node(node));
            }
            other => panic!("unexpected token: {:?}", other),
        }
    }

    WktNode { keyword, args }
}

pub fn parse_wkt(s: &str) -> Vec<WktNode> {
    let mut tokens = tokenize(s);
    parse_nodes(&mut tokens)
}

#[derive(Debug, PartialEq)]
pub struct WktNode {
    pub keyword: String,
    pub args: Vec<WktArg>,
}

#[derive(Debug, PartialEq)]
pub enum WktArg {
    String(String),
    Number(f64),
    Node(WktNode),
    DateTime(Temporal),
}
