use crate::{
	arity::match_arity,
	ast::{Parse, WktNode},
	base_types::Id,
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct DatumEnsembleMember {
	pub ensemble_member_name: String,
	pub identifier: Option<Id>, // TODO: Technically the spec allows multiple
}

impl WktBaseType for DatumEnsembleMember {
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

		match_keywords(&node.keyword, vec![Keywords::Member])?;
		match_arity(node.args.len(), 1, 2)?;

		let ensemble_member_name = node.args[0].parse()?;
		let identifier = match node.args.get(1) {
			Some(x) => Some(x.parse()?),
			None => None,
		};

		let method = DatumEnsembleMember {
			ensemble_member_name,
			identifier,
		};

		Ok(WktBaseTypeResult {
			result: method,
			consumed: 1,
		})
	}
}
