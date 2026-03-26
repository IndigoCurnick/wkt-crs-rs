use crate::{
	arity::match_arity,
	ast::{Parse, WktNode},
	base_types::Id,
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct DefiningTransformation {
	pub defining_transformation_name: String,
	pub identifier: Option<Id>, // TODO: Technically the spec allows for many IDs here
}

impl WktBaseType for DefiningTransformation {
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

		match_keywords(&node.keyword, vec![Keywords::DefiningTransformation])?;

		match_arity(node.args.len(), 1, 2)?;

		let defining_transformation_name = node.args[0].parse()?;

		let identifier = match node.args.get(1) {
			Some(x) => Some(x.parse()?),
			None => None,
		};

		let unit = DefiningTransformation {
			defining_transformation_name,
			identifier,
		};

		let res = WktBaseTypeResult {
			consumed: 1,
			result: unit,
		};

		Ok(res)
	}
}
