use crate::{
	arity::match_arity,
	ast::{Parse, WktNode},
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct Calendar(pub String);

impl WktBaseType for Calendar {
	fn from_nodes<'a, I>(
		wkt_nodes: I,
	) -> Result<WktBaseTypeResult<Self>, WktParseError>
	where
		I: IntoIterator<Item = &'a WktNode>,
	{
		let node = match wkt_nodes.into_iter().next() {
			Some(x) => x,
			None => return Err(WktParseError::NotEnoughNodes),
		};

		match_keywords(&node.keyword, vec![Keywords::Calendar])?;
		match_arity(node.args.len(), 1, 1)?;

		let cal = node.args[0].parse()?;

		let calendar = Calendar(cal);

		return Ok(WktBaseTypeResult {
			result: calendar,
			consumed: 1,
		});
	}
}
