use log::warn;

use crate::{
	arity::match_arity,
	ast::{Parse, WktNode},
	base_types::Id,
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct MapProjectionMethod {
	pub map_projection_method_name: String,
	pub identifier: Option<Id>, // TODO: Technically the spec allows for multiple
}

impl WktBaseType for MapProjectionMethod {
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

		match_keywords(
			&node.keyword,
			vec![Keywords::Method, Keywords::Projection],
		)?;
		match_arity(node.args.len(), 1, 2)?;

		if node.keyword == Keywords::Projection {
			warn!(
				"Keyword PROJECTION depreciated. Consider using METHOD instead"
			);
		}

		let map_projection_method_name = node.args[0].parse()?;
		let identifier = match node.args.get(1) {
			Some(x) => Some(x.parse()?),
			None => None,
		};

		let method = MapProjectionMethod {
			map_projection_method_name,
			identifier,
		};

		Ok(WktBaseTypeResult {
			result: method,
			consumed: 1,
		})
	}
}
