use crate::{
	ast::{WktArg, WktNode},
	base_types::CompoundCrs,
	compound_types::SingleCrs,
	error::WktParseError,
	keywords::Keywords,
	types::{WktBaseType, WktBaseTypeResult, WktInlineResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub enum CoordinateReferenceSystem {
	SingleCrs(SingleCrs),
	CompoundCrs(CompoundCrs),
}

impl WktInlineType for CoordinateReferenceSystem {
	fn from_args<'a, I>(
		wkt_args: I,
	) -> Result<WktInlineResult<Self>, WktParseError>
	where
		I: IntoIterator<Item = &'a crate::ast::WktArg>,
	{
		let mut it = wkt_args.into_iter();

		let mut nodes = vec![];

		while let Some(arg) = it.next() {
			let node = match arg {
				WktArg::Node(n) => n,
				_ => break,
			};

			nodes.push(node);
		}

		let res = CoordinateReferenceSystem::from_nodes(nodes)?;

		return Ok(WktInlineResult {
			consumed: res.consumed,
			result: res.result,
		});
	}
}

impl WktBaseType for CoordinateReferenceSystem {
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
			Keywords::GeodCrs
		};

		if let Ok(stati) = SingleCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: Self::SingleCrs(stati.result),
				consumed: stati.consumed,
			});
		}

		if let Ok(stati) = CompoundCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: Self::CompoundCrs(stati.result),
				consumed: stati.consumed,
			});
		}

		return Err(WktParseError::CouldNotDetermineType {
			keyword: first_keyword,
		});
	}
}
