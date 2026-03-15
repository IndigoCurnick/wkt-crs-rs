use log::error;

use crate::{
	ast::{Parse, WktNode},
	base_types::TimeUnit,
	compound_types::SpatialUnit,
	error::WktParseError,
	keywords::Keywords,
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub enum Unit {
	SpatialUnit(SpatialUnit),
	TimeUnit(TimeUnit),
}

impl WktBaseType for Unit {
	fn from_nodes<'a, I>(
		wkt_nodes: I,
	) -> Result<crate::types::WktBaseTypeResult<Self>, WktParseError>
	where
		I: IntoIterator<Item = &'a WktNode>,
	{
		let node = match wkt_nodes.into_iter().next() {
			Some(x) => x,
			None => return Err(WktParseError::NotEnoughNodes),
		};

		let unit = match &node.keyword {
			Keywords::TimeUnit => Unit::TimeUnit(node.parse()?),
			Keywords::LengthUnit
			| Keywords::AngleUnit
			| Keywords::ScaleUnit
			| Keywords::ParametricUnit => Unit::SpatialUnit(node.parse()?),
			Keywords::Unit => {
				error!(
					"Ambiguous Keyword `UNIT`. While older versions of the specification allowed for this keyword, in this context it is ambiguous and should be depreciated in favour of `LENGTHUNIT`, `ANGLEUNIT` or `SCALEUNIT`"
				);
				return Err(WktParseError::IncorrectKeyword {
					expected: vec![
						Keywords::LengthUnit,
						Keywords::AngleUnit,
						Keywords::ScaleUnit,
						Keywords::ParametricUnit,
						Keywords::TimeUnit,
					]
					.into(),
					found: node.keyword.clone(),
				});
			}
			_ => {
				return Err(WktParseError::IncorrectKeyword {
					expected: vec![
						Keywords::LengthUnit,
						Keywords::AngleUnit,
						Keywords::ScaleUnit,
						Keywords::ParametricUnit,
						Keywords::TimeUnit,
					]
					.into(),
					found: node.keyword.clone(),
				});
			}
		};

		return Ok(WktBaseTypeResult {
			result: unit,
			consumed: 1,
		});
	}
}
