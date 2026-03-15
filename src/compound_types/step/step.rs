use crate::{
	arity::match_arity,
	ast::{WktArg, WktNode},
	base_types::{
		CoordinateOperation, DerivingConversion, MapProjection,
		PointMotionOperation,
	},
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult, WktInlineResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub enum Step {
	CoordinateOperation(CoordinateOperation),
	PointMotionOperation(PointMotionOperation),
	MapProjection(MapProjection),
	DerivingConversion(DerivingConversion),
}

impl WktInlineType for Step {
	fn from_args<'a, I>(
		wkt_args: I,
	) -> Result<WktInlineResult<Self>, WktParseError>
	where
		I: IntoIterator<Item = &'a crate::ast::WktArg>,
	{
		let mut it = wkt_args.into_iter();

		let mut nodes = vec![];

		while let Some(arg) = it.next() {
			let node = match arg {
				WktArg::Node(n) => n,
				_ => break,
			};

			nodes.push(node);
		}

		let res = Step::from_nodes(nodes)?;

		return Ok(WktInlineResult {
			consumed: res.consumed,
			result: res.result,
		});
	}
}

impl WktBaseType for Step {
	fn from_nodes<'a, I>(
		wkt_nodes: I,
	) -> Result<WktBaseTypeResult<Self>, WktParseError>
	where
		I: IntoIterator<Item = &'a WktNode>,
	{
		let in_node = match wkt_nodes.into_iter().next() {
			Some(x) => x,
			None => return Err(WktParseError::NotEnoughNodes),
		};

		match_keywords(&in_node.keyword, vec![Keywords::Step])?;
		match_arity(in_node.args.len(), 1, 1)?;

		let arg = &in_node.args[0];

		let node = match arg {
			WktArg::Node(n) => n,
			_ => return Err(WktParseError::ExpectedNode),
		};

		return match node.keyword {
			Keywords::CoordinateOperation => {
				let s = CoordinateOperation::from_nodes(vec![node])?;
				Ok(WktBaseTypeResult {
					result: Step::CoordinateOperation(s.result),
					consumed: s.consumed,
				})
			}
			Keywords::PointMotionOperation => {
				let s = PointMotionOperation::from_nodes(vec![node])?;
				Ok(WktBaseTypeResult {
					result: Step::PointMotionOperation(s.result),
					consumed: s.consumed,
				})
			}
			Keywords::Conversion => {
				let s = MapProjection::from_nodes(vec![node])?;
				Ok(WktBaseTypeResult {
					result: Step::MapProjection(s.result),
					consumed: s.consumed,
				})
			}
			Keywords::DerivingConversion => {
				let s = DerivingConversion::from_nodes(vec![node])?;
				Ok(WktBaseTypeResult {
					result: Step::DerivingConversion(s.result),
					consumed: s.consumed,
				})
			}
			_ => Err(WktParseError::IncorrectKeyword {
				expected: vec![
					Keywords::CoordinateOperation,
					Keywords::PointMotionOperation,
					Keywords::Conversion,
					Keywords::DerivingConversion,
				]
				.into(),
				found: node.keyword.clone(),
			}),
		};
	}
}
