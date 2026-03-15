use crate::{
	arity::match_arity,
	ast::{Parse, WktNode},
	base_types::Id,
	compound_types::MapProjectionParameterUnit,
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct MapProjectionParameter {
	pub parameter_name: String,
	pub parameter_value: f64,
	pub map_projection_parameter_unit: Option<MapProjectionParameterUnit>,
	pub identifier: Option<Id>, // TODO: Technically the spec allows for multiple
}

impl WktBaseType for MapProjectionParameter {
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

		match_keywords(&node.keyword, vec![Keywords::Parameter])?;
		match_arity(node.args.len(), 2, 4)?;

		let parameter_name = node.args[0].parse()?;
		let parameter_value = node.args[1].parse()?;

		let map_projection_parameter_unit = match node.args.get(2) {
			Some(x) => Some(x.parse()?),
			None => None,
		};

		let identifier = match node.args.get(3) {
			Some(x) => Some(x.parse()?),
			None => None,
		};

		let m = Self {
			parameter_name,
			parameter_value,
			map_projection_parameter_unit,
			identifier,
		};

		return Ok(WktBaseTypeResult {
			result: m,
			consumed: 1,
		});
	}
}
