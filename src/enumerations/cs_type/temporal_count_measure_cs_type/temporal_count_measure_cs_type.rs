use std::str::FromStr;

use strum::{AsRefStr, EnumString};

use crate::{ast::WktArg, error::WktParseError};

#[derive(Debug, PartialEq, EnumString, AsRefStr)]
pub enum TemporalCountMeasureCsType {
	#[strum(serialize = "temporalCount")]
	TemporalCount,
	#[strum(serialize = "temporalMeasure")]
	TemporalMeasure,
}

impl TryFrom<&WktArg> for TemporalCountMeasureCsType {
	type Error = WktParseError;

	fn try_from(value: &WktArg) -> Result<Self, Self::Error> {
		return match value {
			WktArg::Data(n) => match TemporalCountMeasureCsType::from_str(n) {
				Ok(x) => Ok(x),
				Err(y) => Err(WktParseError::ParseError(y)),
			},
			_ => Err(WktParseError::ExpectedString { arg: value.clone() }),
		};
	}
}
