use crate::{
	arity::match_arity,
	ast::{Parse, WktNode},
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct FrameEpoch(pub f64);

impl WktBaseType for FrameEpoch {
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

		match_keywords(&node.keyword, vec![Keywords::FrameEpoch])?;
		match_arity(node.args.len(), 1, 1)?;

		let fr = node.args[0].parse()?;

		let accuracy = FrameEpoch(fr);

		Ok(WktBaseTypeResult {
			result: accuracy,
			consumed: 1,
		})
	}
}
