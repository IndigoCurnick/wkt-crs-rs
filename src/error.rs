use std::{error::Error, fmt::Display};

use strum::ParseError;

#[derive(Debug)]
pub struct Allowed<T: ToString>(Vec<T>);

impl<T: ToString> Display for Allowed<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = "[".to_string();
        for a in &self.0 {
            s.push_str(&a.to_string());
            s.push(',');
        }

        s.pop();
        s.push(']');
        write!(f, "{}", s)
    }
}

impl<T: ToString> From<Vec<T>> for Allowed<T> {
    fn from(value: Vec<T>) -> Self {
        return Self(value);
    }
}

#[derive(Debug)]
pub enum WktParseError {
    UnknownKeyword(String),
    IncorrectArity {
        expected: Allowed<String>,
        found: usize,
    },
    ExpectedString,
    ExpectedNumber,
    ExpectedStringOrNumber,
    ExpectedNode,
    IncorrectKeyword {
        expected: Allowed<String>,
        found: String,
    },
    TooManyKeyword(String),
    TooFewKeyword(String),
    IncorrectKeywordOrder,
    ParseError(ParseError),
    IncorrectValue,
}

impl Display for WktParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownKeyword(k) => write!(f, "Unknown Keyword: `{}`", k),
            Self::IncorrectArity { expected, found } => write!(
                f,
                "Incorrect array length. Expected: {}. Got: {}",
                expected, found
            ),
            Self::ExpectedString => write!(f, "Expected String"),
            Self::ExpectedNumber => write!(f, "Expected Number"),
            Self::ExpectedStringOrNumber => write!(f, "Expected String or Number"),
            Self::ExpectedNode => write!(f, "Expected Node"),
            Self::IncorrectKeyword { expected, found } => {
                write!(
                    f,
                    "Incorrect keyword for this type. Expected: {}, Got: {}",
                    expected, found
                )
            }
            Self::TooManyKeyword(s) => write!(f, "Keyword `{}` has appeared too many times", s),
            Self::IncorrectKeywordOrder => write!(f, "Keywords are not in correct order"),
            Self::ParseError(p) => write!(f, "{}", p),
            Self::IncorrectValue => write!(
                f,
                "A value which is not supported for this field was provided"
            ),
        }
    }
}

impl Error for WktParseError {}
