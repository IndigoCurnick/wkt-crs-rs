use crate::{
	arity::match_arity,
	ast::{Parse, WktNode},
	base_types::{BaseGeodeticCrs, Id, MapProjection},
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct BaseProjectedCrs {
	pub base_crs_name: String,
	pub base_geodetic_crs: BaseGeodeticCrs,
	pub map_projection: MapProjection,
	pub identifier: Option<Id>, // TODO: Technically allowed multiple
}

impl WktBaseType for BaseProjectedCrs {
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

		match_keywords(&node.keyword, vec![Keywords::BaseProjCrs])?;
		match_arity(node.args.len(), 3, 4)?;

		let base_crs_name = node.args[0].parse()?;
		let base_geodetic_crs = node.args[1].parse()?;
		let map_projection = node.args[2].parse()?;

		let identifier = match node.args.get(3) {
			Some(x) => Some(x.parse()?),
			None => None,
		};

		let res = BaseProjectedCrs {
			base_crs_name,
			base_geodetic_crs,
			map_projection,
			identifier,
		};

		Ok(WktBaseTypeResult {
			result: res,
			consumed: 1,
		})
	}
}
