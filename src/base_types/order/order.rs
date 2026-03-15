use crate::{
	arity::match_arity,
	ast::{Parse, WktNode},
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct Order(pub u8);

impl WktBaseType for Order {
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

		match_keywords(&node.keyword, vec![Keywords::Order])?;
		match_arity(node.args.len(), 1, 1)?;

		let order = node.args[0].parse()?;

		let res = WktBaseTypeResult {
			consumed: 1,
			result: Order(order),
		};

		Ok(res)
	}
}
