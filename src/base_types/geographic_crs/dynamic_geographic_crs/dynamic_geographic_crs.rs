use crate::{
	arity::lower_bound_arity,
	ast::{Parse, WktNode},
	base_types::{CoordinateSystem, DynamicCrs, GeodeticReferenceFrame},
	compound_types::ScopeExtentIdentifierRemark,
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct DynamicGeographicCrs {
	pub crs_name: String,
	pub dynamic_crs: DynamicCrs,
	pub geodetic_reference_frame: GeodeticReferenceFrame,
	pub coordinate_system: CoordinateSystem,
	pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl WktBaseType for DynamicGeographicCrs {
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
			vec![Keywords::GeogCrs, Keywords::GeographicCrs],
		)?;
		lower_bound_arity(node.args.len(), 4)?;

		let crs_name = node.args[0].parse()?;
		let dynamic_crs = node.args[1].parse()?;
		let geodetic_reference_frame = node.args[2].parse()?;

		let coordinate_system =
			CoordinateSystem::from_args(&node.args[3..node.args.len()])?;

		let scope_extent_identifier_remark =
			ScopeExtentIdentifierRemark::from_args(
				&node.args[3 + coordinate_system.consumed..node.args.len()],
			)?;

		let res = DynamicGeographicCrs {
			crs_name,
			dynamic_crs,
			geodetic_reference_frame,
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
