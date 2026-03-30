use crate::{
	arity::lower_bound_arity,
	ast::{Parse, WktNode},
	base_types::{
		CoordinateSystem, DynamicCrs, GeoidModelId, VerticalReferenceFrame,
	},
	compound_types::ScopeExtentIdentifierRemark,
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct DynamicVerticalCrs {
	pub crs_name: String,
	pub dynamic_crs: DynamicCrs,
	pub vertical_reference_frame: VerticalReferenceFrame,
	pub coordinate_system: CoordinateSystem,
	pub geoid_model_id: Option<GeoidModelId>,
	pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl WktBaseType for DynamicVerticalCrs {
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
		lower_bound_arity(node.args.len(), 4)?;

		let crs_name = node.args[0].parse()?;
		let dynamic_crs = node.args[1].parse()?;
		let vertical_reference_frame = node.args[2].parse()?;

		let coordinate_system =
			CoordinateSystem::from_args(&node.args[3..node.args.len()])?;

		let mut u = 0;
		let geoid_model_id = match node.args.get(3 + coordinate_system.consumed)
		{
			Some(x) => {
				if let Ok(y) = x.parse() {
					u += 1;
					Some(y)
				} else {
					None
				}
			}
			None => None,
		};

		let scope_extent_identifier_remark =
			ScopeExtentIdentifierRemark::from_args(
				&node.args[u + 3 + coordinate_system.consumed..node.args.len()],
			)?;

		let res = DynamicVerticalCrs {
			crs_name,
			dynamic_crs,
			vertical_reference_frame,
			coordinate_system: coordinate_system.result,
			geoid_model_id,
			scope_extent_identifier_remark: scope_extent_identifier_remark
				.result,
		};

		Ok(WktBaseTypeResult {
			result: res,
			consumed: 1,
		})
	}
}
