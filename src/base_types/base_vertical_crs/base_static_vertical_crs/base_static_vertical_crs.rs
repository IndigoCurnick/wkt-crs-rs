use crate::{
	arity::match_arity,
	ast::{Parse, WktNode},
	base_types::{Id, VerticalReferenceFrame},
	compound_types::VerticalFrameDatum,
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct BaseStaticVerticalCrs {
	pub base_crs_name: String,
	pub vertical_frame_datum: VerticalFrameDatum,
	pub vertical_reference_frame: VerticalReferenceFrame,
	pub identifier: Option<Id>, // TODO: multiple
}

impl WktBaseType for BaseStaticVerticalCrs {
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

		match_keywords(&node.keyword, vec![Keywords::BaseVertCrs])?;
		match_arity(node.args.len(), 3, 4);

		let base_crs_name = node.args[0].parse()?;
		let vertical_frame_datum = node.args[1].parse()?;
		let vertical_reference_frame = node.args[2].parse()?;

		let identifier = match node.args.get(3) {
			Some(x) => Some(x.parse()?),
			None => None,
		};

		let res = BaseStaticVerticalCrs {
			base_crs_name,
			vertical_frame_datum,
			vertical_reference_frame,
			identifier,
		};

		Ok(WktBaseTypeResult {
			result: res,
			consumed: 1,
		})
	}
}
