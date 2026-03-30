use std::{error::Error, fmt::Display};

use strum::ParseError;

use crate::{ast::WktArg, enumerations::AxisDirection, keywords::Keywords};

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
	ExpectedKeyword,
	UnexpectedToken {
		token: String,
	},
	IncorrectArity {
		min: usize,
		max: Option<usize>,
		found: usize,
	},
	ExpectedString {
		arg: WktArg,
	},
	ExpectedNumber {
		arg: WktArg,
	},
	ExpectedStringOrNumber {
		arg: WktArg,
	},
	ExpectedStringOrDate {
		arg: WktArg,
	},
	ExpectedNode,
	IncorrectKeyword {
		expected: Allowed<Keywords>,
		found: Keywords,
	},
	TooManyKeyword(Keywords),
	TooFewKeyword(Keywords),
	IncorrectKeywordOrder,
	ParseError {
		err: ParseError,
		data: String,
	},
	IncorrectValue,
	CouldNotDetermineType {
		keyword: Keywords,
	},
	NotEnoughNodes,
	IncorrectAxisDirection {
		dir: AxisDirection,
	},
}

impl Display for WktParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::UnknownKeyword(k) => write!(f, "Unknown Keyword: `{}`", k),
			Self::ExpectedKeyword => write!(f, "Expected Keyword"),
			Self::UnexpectedToken { token } => {
				write!(f, "Unexpected Token: {}", token)
			}
			Self::IncorrectArity { min, max, found } => {
				if let Some(m) = max {
					write!(
						f,
						"Incorrect arity. Expected between `{}` and `{}`, got `{}`",
						min, m, found
					)
				} else {
					write!(
						f,
						"Incorrect arity. Expected at least `{}`, got `{}",
						min, found
					)
				}
			}
			Self::ExpectedString { arg } => {
				write!(f, "Expected String for arg `{:?}`", arg)
			}
			Self::ExpectedNumber { arg } => {
				write!(f, "Expected Number for arg `{:?}`", arg)
			}
			Self::ExpectedStringOrNumber { arg } => {
				write!(f, "Expected String or Number for arg `{:?}`", arg)
			}
			Self::ExpectedStringOrDate { arg } => {
				write!(f, "Expected String or DateTime for arg `{:?}`", arg)
			}
			Self::ExpectedNode => write!(f, "Expected Node"),
			Self::IncorrectKeyword { expected, found } => {
				write!(
					f,
					"Incorrect keyword for this type. Expected: {}, Got: {}",
					expected, found
				)
			}
			Self::TooManyKeyword(s) => {
				write!(f, "Keyword `{}` has appeared too many times", s)
			}
			Self::TooFewKeyword(s) => {
				write!(f, "Keyword `{}` has not appeared enough times", s)
			}
			Self::IncorrectKeywordOrder => {
				write!(f, "Keywords are not in correct order")
			}
			Self::ParseError { err, data } => write!(f, "{} for {}", err, data),
			Self::IncorrectValue => write!(
				f,
				"A value which is not supported for this field was provided"
			),
			Self::CouldNotDetermineType { keyword } => {
				write!(f, "Could not determine variation of: `{}`", keyword)
			}
			Self::NotEnoughNodes => {
				write!(f, "Not enough nodes to construct this type")
			}
			Self::IncorrectAxisDirection { dir } => {
				write!(f, "Incorrect Axis Direction: `{:?}`", dir)
			}
		}
	}
}

impl Error for WktParseError {}
