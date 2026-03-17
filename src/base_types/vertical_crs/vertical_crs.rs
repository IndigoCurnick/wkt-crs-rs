use crate::{
	ast::WktNode,
	base_types::vertical_crs::{DynamicVerticalCrs, StaticVerticalCrs},
	error::WktParseError,
	keywords::Keywords,
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub enum VerticalCrs {
	StaticVerticalCrs(StaticVerticalCrs),
	DynamicVerticalCrs(DynamicVerticalCrs),
}

impl WktBaseType for VerticalCrs {
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
			Keywords::VerticalCrs
		};

		if let Ok(res) = StaticVerticalCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				consumed: res.consumed,
				result: Self::StaticVerticalCrs(res.result),
			});
		}

		if let Ok(res) = DynamicVerticalCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				consumed: res.consumed,
				result: Self::DynamicVerticalCrs(res.result),
			});
		}

		return Err(WktParseError::CouldNotDetermineType {
			keyword: first_keyword,
		});
	}
}
