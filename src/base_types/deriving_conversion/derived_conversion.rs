use crate::{
	arity::lower_bound_arity,
	ast::{Parse, WktArg, WktNode},
	base_types::{Id, OperationMethod},
	enumerations::OperationParameterWrapper,
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct DerivingConversion {
	pub deriving_conversion_name: String,
	pub operation_method: OperationMethod,
	pub operation_parameter: Option<Vec<OperationParameterWrapper>>,
	pub identifier: Option<Id>, // TODO: allow multiple
}

impl WktBaseType for DerivingConversion {
	fn from_nodes<'a, I>(
		wkt_nodes: I,
	) -> Result<WktBaseTypeResult<Self>, WktParseError>
	where
		I: IntoIterator<Item = &'a WktNode>,
	{
		let mut iterator = wkt_nodes.into_iter();

		// First Part
		let base_node = match iterator.next() {
			Some(x) => x,
			None => return Err(WktParseError::NotEnoughNodes),
		};

		match_keywords(&base_node.keyword, vec![Keywords::DerivingConversion])?;
		lower_bound_arity(base_node.args.len(), 3)?;

		let deriving_conversion_name = base_node.args[0].parse()?;
		let operation_method = base_node.args[1].parse()?;

		let mut operation_parameter = vec![];
		let mut identifier = None;

		for i in 2..base_node.args.len() {
			let this_value = &base_node.args[i];

			match this_value {
				WktArg::Node(node) => {
					match node.keyword {
						Keywords::Parameter | Keywords::ParameterFile => {
							// Parameters must come before identifier

							if identifier.is_some() {
								return Err(
									WktParseError::IncorrectKeywordOrder,
								);
							}

							let param = node.parse()?;

							operation_parameter.push(param);
						}

						Keywords::Id => {
							if identifier.is_some() {
								return Err(WktParseError::TooManyKeyword(
									Keywords::Id,
								));
							}

							identifier = Some(node.parse()?);
						}
						_ => {
							return Err(WktParseError::IncorrectKeyword {
								expected: vec![
									Keywords::Member,
									Keywords::Ellipsoid,
									Keywords::Spheroid,
									Keywords::EnsembleAccuracy,
									Keywords::Id,
								]
								.into(),
								found: node.keyword.clone(),
							});
						}
					}
				}
				_ => return Err(WktParseError::ExpectedNode),
			}
		}

		let operation_parameter = if operation_parameter.is_empty() {
			None
		} else {
			Some(operation_parameter)
		};

		let res = DerivingConversion {
			deriving_conversion_name,
			operation_method,
			operation_parameter,
			identifier,
		};

		Ok(WktBaseTypeResult {
			result: res,
			consumed: 1,
		})
	}
}
