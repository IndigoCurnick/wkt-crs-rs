use crate::{
	ast::WktNode,
	error::WktParseError,
	keywords::Keywords,
	types::{WktBaseType, WktBaseTypeResult},
};

pub use derived_dynamic_geod_crs::DerivedDynamicGeodCrs;
pub use derived_geographic_crs::{
	DerivedDynamicGeogCrs, DerivedGeographicCrs, DerivedStaticGeogCrs,
};
pub use derived_static_geod_crs::DerivedStaticGeodCrs;

mod derived_dynamic_geod_crs;
mod derived_geographic_crs;
mod derived_static_geod_crs;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub enum DerivedGeodeticCrs {
	DerivedStaticGeodCrs(DerivedStaticGeodCrs),
	DerivedDynamicGeodCrs(DerivedDynamicGeodCrs),
	DerivedGeographicCrs(DerivedGeographicCrs),
}

impl WktBaseType for DerivedGeodeticCrs {
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
			Keywords::GeodeticCrs
		};

		if let Ok(stati) = DerivedStaticGeodCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: Self::DerivedStaticGeodCrs(stati.result),
				consumed: stati.consumed,
			});
		}

		if let Ok(stati) = DerivedDynamicGeodCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: Self::DerivedDynamicGeodCrs(stati.result),
				consumed: stati.consumed,
			});
		}

		if let Ok(stati) = DerivedGeographicCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: Self::DerivedGeographicCrs(stati.result),
				consumed: stati.consumed,
			});
		}

		return Err(WktParseError::CouldNotDetermineType {
			keyword: first_keyword,
		});
	}
}
