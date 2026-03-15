use horologium::Temporal;

use crate::{
	arity::match_arity,
	ast::{Parse, WktNode},
	base_types::LengthUnit,
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub enum DateOrString {
	Date(Temporal),
	String(String),
}

#[derive(Debug, PartialEq)]
pub struct VerticalExtent {
	pub minimum_height: f64,
	pub maximum_height: f64,
	pub length_unit: Option<LengthUnit>,
}

impl WktBaseType for VerticalExtent {
	fn from_nodes<'a, I>(
		wkt_nodes: I,
	) -> Result<WktBaseTypeResult<Self>, WktParseError>
	where
		I: IntoIterator<Item = &'a WktNode>,
	{
		// Take 1

		let node = match wkt_nodes.into_iter().next() {
			Some(x) => x,
			None => return Err(WktParseError::NotEnoughNodes),
		};

		match_keywords(&node.keyword, vec![Keywords::VerticalExtent])?;
		match_arity(node.args.len(), 1, 3);

		let minimum_height = node.args[0].parse()?;
		let maximum_height = node.args[1].parse()?;

		let length_unit = match node.args.get(2) {
			Some(x) => Some(x.parse()?),
			None => None,
		};

		let ve = VerticalExtent {
			minimum_height,
			maximum_height,
			length_unit,
		};

		return Ok(WktBaseTypeResult {
			result: ve,
			consumed: 1,
		});
	}
}
