use crate::{
	arity::match_arity,
	ast::{Parse, WktNode},
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct AnchorEpoch(pub f64);

impl WktBaseType for AnchorEpoch {
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

		match_keywords(&node.keyword, vec![Keywords::AnchorEpoch])?;
		match_arity(node.args.len(), 1, 1)?;

		let anchor = node.args[0].parse()?;

		let res = WktBaseTypeResult {
			consumed: 1,
			result: AnchorEpoch(anchor),
		};

		Ok(res)
	}
}
