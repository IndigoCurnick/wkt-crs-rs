use std::str::FromStr;

use strum::{AsRefStr, EnumString};

use crate::{ast::WktArg, error::WktParseError};

#[derive(Debug, PartialEq, EnumString, AsRefStr)]
#[strum(ascii_case_insensitive)]
pub enum OrdinalDateTimeCsType {
	#[strum(serialize = "ordinal")]
	Ordinal,
	#[strum(serialize = "temporalDateTime")]
	TemporalDateTime,
}

impl TryFrom<&WktArg> for OrdinalDateTimeCsType {
	type Error = WktParseError;

	fn try_from(value: &WktArg) -> Result<Self, Self::Error> {
		return match value {
			WktArg::Data(n) => match OrdinalDateTimeCsType::from_str(n) {
				Ok(x) => Ok(x),
				Err(y) => Err(WktParseError::ParseError(y)),
			},
			_ => Err(WktParseError::ExpectedString { arg: value.clone() }),
		};
	}
}
