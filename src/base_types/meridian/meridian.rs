use crate::{
	arity::match_arity,
	ast::{Parse, WktNode},
	base_types::AngleUnit,
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct Meridian {
	pub number: f64,
	pub angle_unit: AngleUnit,
}

impl WktBaseType for Meridian {
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

		match_keywords(&node.keyword, vec![Keywords::Meridian])?;
		match_arity(node.args.len(), 2, 2)?;

		let number = node.args[0].parse()?;
		let angle_unit = node.args[1].parse()?;

		let m = Meridian { number, angle_unit };

		return Ok(WktBaseTypeResult {
			result: m,
			consumed: 1,
		});
	}
}
