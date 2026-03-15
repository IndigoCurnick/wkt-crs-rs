use crate::{
	arity::match_arity,
	ast::{Parse, WktNode},
	base_types::Id,
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct ParametricUnit {
	pub unit_name: String,
	pub conversion_factor: f64,
	pub identifier: Option<Id>, // TODO: Technically the Specification allows for multiple
}

impl WktBaseType for ParametricUnit {
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

		match_keywords(&node.keyword, vec![Keywords::ParametricUnit])?;
		match_arity(node.args.len(), 1, 3)?;

		let unit_name = node.args[0].parse()?;
		let conversion_factor = node.args[1].parse()?;

		let identifier = match node.args.get(2) {
			Some(x) => Some(x.parse()?),
			None => None,
		};

		let unit = ParametricUnit {
			conversion_factor,
			identifier,
			unit_name,
		};

		let res = WktBaseTypeResult {
			consumed: 1,
			result: unit,
		};

		Ok(res)
	}
}
