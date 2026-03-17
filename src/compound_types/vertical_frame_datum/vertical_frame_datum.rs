use crate::{
	ast::WktNode,
	base_types::{VerticalDatumEnsemble, VerticalReferenceFrame},
	error::WktParseError,
	keywords::{self, Keywords},
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub enum VerticalFrameDatum {
	VerticalReferenceFrame(VerticalReferenceFrame),
	VerticalDatumEnsemble(VerticalDatumEnsemble),
}

impl WktBaseType for VerticalFrameDatum {
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
			Keywords::VerticalDatum
		};

		if let Ok(res) = VerticalReferenceFrame::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				consumed: res.consumed,
				result: Self::VerticalReferenceFrame(res.result),
			});
		}

		if let Ok(res) = VerticalDatumEnsemble::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				consumed: res.consumed,
				result: Self::VerticalDatumEnsemble(res.result),
			});
		}

		return Err(WktParseError::CouldNotDetermineType {
			keyword: first_keyword,
		});
	}
}
