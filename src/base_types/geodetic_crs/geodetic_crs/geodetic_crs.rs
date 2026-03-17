use crate::{
	ast::WktNode,
	base_types::{
		GeographicCrs,
		geodetic_crs::{
			dynamic_geodetic_crs::DynamicGeodeticCrs,
			static_geodetic_crs::StaticGeodeticCrs,
		},
	},
	error::WktParseError,
	keywords::Keywords,
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub enum GeodeticCrs {
	StaticGeodeticCrs(StaticGeodeticCrs),
	DynamicGeodeticCrs(DynamicGeodeticCrs),
	GeographicCrs(GeographicCrs),
}

impl WktBaseType for GeodeticCrs {
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
			Keywords::GeodeticCrs
		};

		if let Ok(ordinal) = StaticGeodeticCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: GeodeticCrs::StaticGeodeticCrs(ordinal.result),
				consumed: ordinal.consumed,
			});
		}

		if let Ok(spatial) = DynamicGeodeticCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: GeodeticCrs::DynamicGeodeticCrs(spatial.result),
				consumed: spatial.consumed,
			});
		}

		if let Ok(temporal) = GeographicCrs::from_nodes(iter) {
			return Ok(WktBaseTypeResult {
				result: GeodeticCrs::GeographicCrs(temporal.result),
				consumed: temporal.consumed,
			});
		}

		return Err(WktParseError::CouldNotDetermineType {
			keyword: first_keyword,
		});
	}
}
