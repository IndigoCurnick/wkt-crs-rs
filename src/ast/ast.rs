use std::str::FromStr;

use horologium::Temporal;

use crate::{
    data_types::{DateOrString, NumText},
    error::WktParseError,
    keywords::Keywords,
    types::WktBaseType,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(Keywords),
    Data(String),
    LDelimiter,
    RDelimiter,
    WktSeparator,
}

pub fn tokenize(mut s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    while let Some(c) = s.chars().next() {
        match c {
            '[' => {
                tokens.push(Token::LDelimiter);
                s = &s[1..];
            }
            ']' => {
                tokens.push(Token::RDelimiter);
                s = &s[1..];
            }
            ',' => {
                tokens.push(Token::WktSeparator);
                s = &s[1..];
            }
            '"' => {
                let end = s[1..].find('"').unwrap() + 1;
                let content = &s[1..end];
                tokens.push(Token::Data(content.to_string()));
                s = &s[end + 1..];
            }
            c if c.is_whitespace() => {
                s = &s[1..];
            }
            _ => {
                let len = s
                    .find(|ch: char| ch == ',' || ch == '[' || ch == ']')
                    .unwrap_or(s.len());
                let ident_candidate = &s[..len];

                if let Ok(ident) = Keywords::from_str(ident_candidate) {
                    tokens.push(Token::Keyword(ident));
                } else {
                    tokens.push(Token::Data(ident_candidate.to_string()));
                }

                s = &s[len..];
            }
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
        let first = tokens.first();

        match first {
            Some(Token::Keyword(_)) => {
                nodes.push(parse_node(tokens));
            }
            Some(Token::WktSeparator) => {
                tokens.remove(0);
            }
            None => {
                break;
            }
            Some(Token::Data(d)) => panic!("Unexpected token: `{}`", d),
            Some(Token::LDelimiter) => panic!("Unexpected token: `[`"),
            Some(Token::RDelimiter) => panic!("Unexpected token: `]`"),
        }
    }

    while let Some(Token::Keyword(_)) = tokens.first() {
        nodes.push(parse_node(tokens));
    }
    nodes
}

pub fn parse_node(tokens: &mut Vec<Token>) -> WktNode {
    let keyword = match tokens.remove(0) {
        Token::Keyword(s) => s,
        _ => panic!("expected keyword"),
    };

    assert!(matches!(tokens.remove(0), Token::LDelimiter));
    let mut args = Vec::new();
    loop {
        match tokens.first() {
            Some(Token::RDelimiter) => {
                tokens.remove(0);
                break;
            }
            Some(Token::WktSeparator) => {
                tokens.remove(0);
            }
            Some(Token::Data(s)) => {
                args.push(WktArg::Data(s.clone()));
                tokens.remove(0);
            }
            Some(Token::Keyword(_)) => {
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

#[derive(Clone, Debug, PartialEq)]
pub struct WktNode {
    pub keyword: Keywords,
    pub args: Vec<WktArg>,
}

impl<'a> WktElement<'a> for WktNode {
    fn get_node(&'a self) -> Option<&'a WktNode> {
        return Some(self);
    }

    fn get_arg(&'a self) -> Option<&'a WktArg> {
        return None;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum WktArg {
    Data(String),
    Node(WktNode),
}

impl<'a> WktElement<'a> for WktArg {
    fn get_node(&'a self) -> Option<&'a WktNode> {
        return match self {
            Self::Node(n) => Some(n),
            Self::Data(_) => None,
        };
    }

    fn get_arg(&'a self) -> Option<&'a WktArg> {
        return Some(self);
    }
}

pub trait WktElement<'a> {
    fn get_node(&'a self) -> Option<&'a WktNode>;
    fn get_arg(&'a self) -> Option<&'a WktArg>;
}

pub trait Parse<T> {
    fn parse(&self) -> Result<T, WktParseError>;
}

impl Parse<String> for WktArg {
    fn parse(&self) -> Result<String, WktParseError> {
        match self {
            Self::Data(s) => Ok(s.clone()),
            Self::Node(_) => return Err(WktParseError::ExpectedString),
        }
    }
}

impl Parse<f64> for WktArg {
    fn parse(&self) -> Result<f64, WktParseError> {
        let num_str = match self {
            Self::Data(s) => s,
            Self::Node(_) => return Err(WktParseError::ExpectedNumber),
        };

        return match num_str.parse::<f64>() {
            Ok(x) => Ok(x),
            Err(_) => Err(WktParseError::ExpectedNumber),
        };
    }
}

impl Parse<u8> for WktArg {
    fn parse(&self) -> Result<u8, WktParseError> {
        let num_str = match self {
            Self::Data(s) => s,
            Self::Node(_) => return Err(WktParseError::ExpectedNumber),
        };

        return match num_str.parse::<u8>() {
            Ok(x) => Ok(x),
            Err(_) => Err(WktParseError::ExpectedNumber),
        };
    }
}

impl Parse<i32> for WktArg {
    fn parse(&self) -> Result<i32, WktParseError> {
        let num_str = match self {
            Self::Data(s) => s,
            Self::Node(_) => return Err(WktParseError::ExpectedNumber),
        };

        return match num_str.parse::<i32>() {
            Ok(x) => Ok(x),
            Err(_) => Err(WktParseError::ExpectedNumber),
        };
    }
}

impl Parse<NumText> for WktArg {
    fn parse(&self) -> Result<NumText, WktParseError> {
        return match self {
            Self::Data(s) => Ok(NumText::from(s.as_str())),
            Self::Node(_) => Err(WktParseError::ExpectedStringOrNumber),
        };
    }
}

impl Parse<DateOrString> for WktArg {
    fn parse(&self) -> Result<DateOrString, WktParseError> {
        let date_str = match self {
            Self::Data(s) => s,
            Self::Node(_) => return Err(WktParseError::ExpectedStringOrDate),
        };

        return if let Ok(date) = Temporal::try_from(date_str.as_str()) {
            Ok(DateOrString::Date(date))
        } else {
            Ok(DateOrString::String(date_str.clone()))
        };
    }
}

impl<T: WktBaseType> Parse<T> for WktArg {
    fn parse(&self) -> Result<T, WktParseError> {
        return match self {
            Self::Node(node) => Ok(T::from_nodes(vec![node])?.result),
            Self::Data(_) => Err(WktParseError::ExpectedNode),
        };
    }
}

impl<T: WktBaseType> Parse<T> for WktNode {
    fn parse(&self) -> Result<T, WktParseError> {
        Ok(T::from_nodes(vec![self])?.result)
    }
}
