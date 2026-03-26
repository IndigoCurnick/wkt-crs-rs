use crate::{
	arity::lower_bound_arity,
	ast::{Parse, WktArg, WktNode},
	base_types::{CoordinateSystem, DefiningTransformation},
	compound_types::{GeodeticData, ScopeExtentIdentifierRemark},
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct StaticGeographicCrs {
	pub crs_name: String,
	pub frame: GeodeticData,
	pub coordinate_system: CoordinateSystem,
	pub defining_transformation_id: Option<DefiningTransformation>,
	pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl WktBaseType for StaticGeographicCrs {
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

		let frame = GeodeticData::from_args(&node.args[1..node.args.len()])?;

		let mut i = 1 + frame.consumed;

		let coordinate_system =
			CoordinateSystem::from_args(&node.args[i..node.args.len()])?;
		i += coordinate_system.consumed;

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

		let res = StaticGeographicCrs {
			crs_name,
			frame: frame.result,
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
