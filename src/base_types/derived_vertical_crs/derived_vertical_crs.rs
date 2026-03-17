use crate::{
	arity::lower_bound_arity,
	ast::{Parse, WktNode},
	base_types::{BaseVerticalCrs, DerivingConversion},
	compound_types::ScopeExtentIdentifierRemark,
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct DerivedVerticalCrs {
	pub derived_crs_name: String,
	pub base_vertical_crs: BaseVerticalCrs,
	pub deriving_conversion: DerivingConversion,
	pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl WktBaseType for DerivedVerticalCrs {
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

		match_keywords(
			&node.keyword,
			vec![Keywords::VertCrs, Keywords::VerticalCrs],
		)?;
		lower_bound_arity(node.args.len(), 3)?;

		let derived_crs_name = node.args[0].parse()?;
		let base_vertical_crs = node.args[1].parse()?;
		let deriving_conversion = node.args[2].parse()?;

		let scope_extent_identifier_remark =
			ScopeExtentIdentifierRemark::from_args(
				&node.args[3..node.args.len()],
			)?;

		let res = DerivedVerticalCrs {
			derived_crs_name,
			base_vertical_crs,
			deriving_conversion,
			scope_extent_identifier_remark: scope_extent_identifier_remark
				.result,
		};

		Ok(WktBaseTypeResult {
			result: res,
			consumed: 1,
		})
	}
}
