use crate::{
	arity::match_arity,
	ast::{Parse, WktNode},
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct GeographicBoundingBox {
	pub lower_left_latitude: f64,
	pub lower_left_longitude: f64,
	pub upper_right_latitude: f64,
	pub upper_right_longitude: f64,
}

// ?: Should there should be some restrictions on the allowed lats/lons
impl WktBaseType for GeographicBoundingBox {
	fn from_nodes<'a, I>(
		wkt_nodes: I,
	) -> Result<crate::types::WktBaseTypeResult<Self>, WktParseError>
	where
		I: IntoIterator<Item = &'a WktNode>,
	{
		// Take 1
		let node = match wkt_nodes.into_iter().next() {
			Some(x) => x,
			None => return Err(WktParseError::NotEnoughNodes),
		};

		match_keywords(&node.keyword, vec![Keywords::BBox])?;
		match_arity(node.args.len(), 4, 4)?;

		let lower_left_latitude = node.args[0].parse()?;
		let lower_left_longitude = node.args[1].parse()?;
		let upper_right_latitude = node.args[2].parse()?;
		let upper_right_longitude = node.args[3].parse()?;

		let bbox = GeographicBoundingBox {
			lower_left_latitude,
			lower_left_longitude,
			upper_right_latitude,
			upper_right_longitude,
		};

		let res = WktBaseTypeResult {
			consumed: 1,
			result: bbox,
		};

		Ok(res)
	}
}
