use crate::{
	arity::lower_bound_arity,
	ast::{Parse, WktArg, WktNode},
	base_types::{
		CoordinateSystem, DefiningTransformation, DynamicCrs,
		GeodeticReferenceFrame,
	},
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
	pub defining_transformation_id: Option<DefiningTransformation>,
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

		let mut i = 3 + coordinate_system.consumed;

		let defining_transformation_id = match node.args.get(i) {
			Some(x) => match x {
				WktArg::Data(_) => None,
				WktArg::Node(y) => match y.keyword {
					Keywords::DefiningTransformation => {
						i += 1;
						Some(y.parse()?)
					}
					_ => None,
				},
			},
			None => None,
		};

		let scope_extent_identifier_remark =
			ScopeExtentIdentifierRemark::from_args(
				&node.args[i..node.args.len()],
			)?;

		let res = DynamicGeographicCrs {
			crs_name,
			dynamic_crs,
			geodetic_reference_frame,
			coordinate_system: coordinate_system.result,
			defining_transformation_id,
			scope_extent_identifier_remark: scope_extent_identifier_remark
				.result,
		};

		Ok(WktBaseTypeResult {
			result: res,
			consumed: 1,
		})
	}
}
