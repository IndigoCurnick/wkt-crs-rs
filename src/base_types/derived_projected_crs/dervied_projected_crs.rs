use crate::{
	arity::lower_bound_arity,
	ast::{Parse, WktNode},
	base_types::{BaseProjectedCrs, CoordinateSystem, DerivingConversion},
	compound_types::ScopeExtentIdentifierRemark,
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct DerivedProjectedCrs {
	pub derived_crs_name: String,
	pub base_projected_crs: BaseProjectedCrs,
	pub deriving_conversion: DerivingConversion,
	pub coordinate_system: CoordinateSystem,
	pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl WktBaseType for DerivedProjectedCrs {
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

		match_keywords(&node.keyword, vec![Keywords::DerivedProjCrs])?;
		lower_bound_arity(node.args.len(), 4)?;

		let derived_crs_name = node.args[0].parse()?;
		let base_projected_crs = node.args[1].parse()?;
		let deriving_conversion = node.args[2].parse()?;

		let coordinate_system =
			CoordinateSystem::from_args(&node.args[3..node.args.len()])?;

		let scope_extent_identifier_remark =
			ScopeExtentIdentifierRemark::from_args(
				&node.args[3 + coordinate_system.consumed..node.args.len()],
			)?;

		let res = DerivedProjectedCrs {
			derived_crs_name,
			base_projected_crs,
			deriving_conversion,
			coordinate_system: coordinate_system.result,
			scope_extent_identifier_remark: scope_extent_identifier_remark
				.result,
		};

		Ok(WktBaseTypeResult {
			result: res,
			consumed: 1,
		})
	}
}
