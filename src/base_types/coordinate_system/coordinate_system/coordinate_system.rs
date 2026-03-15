use crate::{
	ast::{WktArg, WktNode},
	base_types::coordinate_system::{
		OrdinalDateTimeCoordinateSystem, SpatialCoordinateSystem,
		TemporalCountMeasureCoordinateSystem,
	},
	error::WktParseError,
	types::{WktBaseType, WktBaseTypeResult, WktInlineResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub enum CoordinateSystem {
	SpatialCS(SpatialCoordinateSystem),
	TemporalCountMeasureCS(TemporalCountMeasureCoordinateSystem),
	OrdinalDateTimeCS(OrdinalDateTimeCoordinateSystem),
}

impl WktBaseType for CoordinateSystem {
	fn from_nodes<'a, I>(
		wkt_nodes: I,
	) -> Result<crate::types::WktBaseTypeResult<Self>, WktParseError>
	where
		I: IntoIterator<Item = &'a WktNode>,
	{
		let iter: Vec<&'a WktNode> = wkt_nodes.into_iter().collect();

		if let Ok(ordinal) =
			OrdinalDateTimeCoordinateSystem::from_nodes(iter.clone())
		{
			return Ok(WktBaseTypeResult {
				result: CoordinateSystem::OrdinalDateTimeCS(ordinal.result),
				consumed: ordinal.consumed,
			});
		}

		if let Ok(spatial) = SpatialCoordinateSystem::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: CoordinateSystem::SpatialCS(spatial.result),
				consumed: spatial.consumed,
			});
		}

		if let Ok(temporal) =
			TemporalCountMeasureCoordinateSystem::from_nodes(iter)
		{
			return Ok(WktBaseTypeResult {
				result: CoordinateSystem::TemporalCountMeasureCS(
					temporal.result,
				),
				consumed: temporal.consumed,
			});
		}

		return Err(WktParseError::CouldNotDetermineType);
	}
}

impl WktInlineType for CoordinateSystem {
	fn from_args<'a, I>(
		wkt_args: I,
	) -> Result<WktInlineResult<Self>, WktParseError>
	where
		I: IntoIterator<Item = &'a crate::ast::WktArg>,
	{
		let iter: Vec<&'a WktArg> = wkt_args.into_iter().collect();

		if let Ok(ordinal) =
			OrdinalDateTimeCoordinateSystem::from_args(iter.clone())
		{
			return Ok(WktInlineResult {
				result: CoordinateSystem::OrdinalDateTimeCS(ordinal.result),
				consumed: ordinal.consumed,
			});
		}

		if let Ok(spatial) = SpatialCoordinateSystem::from_args(iter.clone()) {
			return Ok(WktInlineResult {
				result: CoordinateSystem::SpatialCS(spatial.result),
				consumed: spatial.consumed,
			});
		}

		if let Ok(temporal) =
			TemporalCountMeasureCoordinateSystem::from_args(iter)
		{
			return Ok(WktInlineResult {
				result: CoordinateSystem::TemporalCountMeasureCS(
					temporal.result,
				),
				consumed: temporal.consumed,
			});
		}
		return Err(WktParseError::CouldNotDetermineType);
	}
}
