use crate::{
	arity::lower_bound_arity,
	ast::WktNode,
	base_types::CoordinateSystem,
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct InterpolationCrs {
	pub coordinate_system: CoordinateSystem,
}

impl WktBaseType for InterpolationCrs {
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
		match_keywords(&node.keyword, vec![Keywords::InterpolationCrs])?;
		lower_bound_arity(node.args.len(), 2)?;

		let coordinate_system = CoordinateSystem::from_args(&node.args)?;

		let res = InterpolationCrs {
			coordinate_system: coordinate_system.result,
		};

		Ok(WktBaseTypeResult {
			result: res,
			consumed: 1,
		})
	}
}
