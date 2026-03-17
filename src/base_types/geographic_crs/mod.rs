mod dynamic_geographic_crs;
mod static_geographic_crs;
pub use dynamic_geographic_crs::DynamicGeographicCrs;
pub use static_geographic_crs::StaticGeographicCrs;

use crate::{
	ast::WktNode,
	error::WktParseError,
	keywords::Keywords,
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub enum GeographicCrs {
	DynamicGeographicCrs(DynamicGeographicCrs),
	StaticGeographicCrs(StaticGeographicCrs),
}

impl WktBaseType for GeographicCrs {
	fn from_nodes<'a, I>(
		wkt_nodes: I,
	) -> Result<crate::types::WktBaseTypeResult<Self>, WktParseError>
	where
		I: IntoIterator<Item = &'a WktNode>,
	{
		let iter: Vec<&'a WktNode> = wkt_nodes.into_iter().collect();

		let first_keyword = if let Some(nod) = iter.get(0) {
			nod.keyword.clone()
		} else {
			// TODO: Just some default, if there's no nodes I guess?
			Keywords::GeogCrs
		};

		if let Ok(ordinal) = DynamicGeographicCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: GeographicCrs::DynamicGeographicCrs(ordinal.result),
				consumed: ordinal.consumed,
			});
		}

		if let Ok(spatial) = StaticGeographicCrs::from_nodes(iter) {
			return Ok(WktBaseTypeResult {
				result: GeographicCrs::StaticGeographicCrs(spatial.result),
				consumed: spatial.consumed,
			});
		}

		return Err(WktParseError::CouldNotDetermineType {
			keyword: first_keyword,
		});
	}
}
