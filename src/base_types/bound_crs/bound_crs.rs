use crate::{
	arity::lower_bound_arity,
	ast::{Parse, WktNode},
	base_types::{AbridgedCoordinateTransformation, SourceCrs, TargetCrs},
	compound_types::ScopeExtentIdentifierRemark,
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct BoundCrs {
	pub source_crs: SourceCrs,
	pub target_crs: TargetCrs,
	pub abridged_coordinate_transformation: AbridgedCoordinateTransformation,
	pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl WktBaseType for BoundCrs {
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

		match_keywords(&node.keyword, vec![Keywords::BoundCrs])?;
		lower_bound_arity(node.args.len(), 3)?;

		let source_crs = node.args[0].parse()?;
		let target_crs = node.args[1].parse()?;
		let abridged_coordinate_transformation = node.args[2].parse()?;

		let maybe_slice = node.args.get(3..node.args.len());

		let scope_extent_identifier_remark = match maybe_slice {
			Some(x) => ScopeExtentIdentifierRemark::from_args(x)?.result,
			None => ScopeExtentIdentifierRemark {
				usage: None,
				identifier: None,
				remark: None,
			},
		};

		let res = BoundCrs {
			source_crs,
			target_crs,
			abridged_coordinate_transformation,
			scope_extent_identifier_remark,
		};

		Ok(WktBaseTypeResult {
			result: res,
			consumed: 1,
		})
	}
}
