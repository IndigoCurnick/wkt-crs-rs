use crate::{
	arity::match_arity,
	ast::{Parse, WktNode},
	base_types::CoordinateEpoch,
	compound_types::{
		DynamicCrsCoordinateMetadata, StaticCrsCoordinateMetadata,
	},
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub enum CoordinateMetadata {
	StaticCoordinateMetadata(StaticCoordinateMetadata),
	DynamicCoordinateMetadata(DynamicCoordinateMetadata),
}

impl WktBaseType for CoordinateMetadata {
	fn from_nodes<'a, I>(
		wkt_nodes: I,
	) -> Result<WktBaseTypeResult<Self>, WktParseError>
	where
		I: IntoIterator<Item = &'a WktNode>,
	{
		let iter: Vec<&'a WktNode> = wkt_nodes.into_iter().collect();

		let first_keyword = if let Some(nod) = iter.get(0) {
			nod.keyword.clone()
		} else {
			// TODO: Just some default, if there's no nodes I guess?
			Keywords::CoordinateMetadata
		};

		if let Ok(stati) = StaticCoordinateMetadata::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: Self::StaticCoordinateMetadata(stati.result),
				consumed: stati.consumed,
			});
		}

		if let Ok(stati) = DynamicCoordinateMetadata::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: Self::DynamicCoordinateMetadata(stati.result),
				consumed: stati.consumed,
			});
		}

		return Err(WktParseError::CouldNotDetermineType {
			keyword: first_keyword,
		});
	}
}

#[derive(Debug, PartialEq)]
pub struct StaticCoordinateMetadata {
	pub static_coordinate_metadata: StaticCrsCoordinateMetadata,
}

impl WktBaseType for StaticCoordinateMetadata {
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

		match_keywords(&node.keyword, vec![Keywords::CoordinateMetadata])?;
		match_arity(node.args.len(), 1, 1)?;

		let stati = node.args[0].parse()?;

		let s = StaticCoordinateMetadata {
			static_coordinate_metadata: stati,
		};

		return Ok(WktBaseTypeResult {
			result: s,
			consumed: 1,
		});
	}
}

#[derive(Debug, PartialEq)]
pub struct DynamicCoordinateMetadata {
	pub dynamic_coordinate_metadata: DynamicCrsCoordinateMetadata,
	pub metadata_coordinate_epoch: CoordinateEpoch,
}

impl WktBaseType for DynamicCoordinateMetadata {
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

		match_keywords(&node.keyword, vec![Keywords::CoordinateMetadata])?;
		match_arity(node.args.len(), 2, 2)?;

		let dynamic = node.args[0].parse()?;
		let epoch = node.args[1].parse()?;

		let s = DynamicCoordinateMetadata {
			dynamic_coordinate_metadata: dynamic,
			metadata_coordinate_epoch: epoch,
		};

		return Ok(WktBaseTypeResult {
			result: s,
			consumed: 1,
		});
	}
}
