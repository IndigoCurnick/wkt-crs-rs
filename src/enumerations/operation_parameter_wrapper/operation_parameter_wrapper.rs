use crate::{
	base_types::{OperationParameter, OperationParameterFile},
	error::WktParseError,
	keywords::Keywords,
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub enum OperationParameterWrapper {
	OperationParameter(OperationParameter),
	OperationParameterFile(OperationParameterFile),
}

impl WktBaseType for OperationParameterWrapper {
	fn from_nodes<'a, I>(
		wkt_nodes: I,
	) -> Result<
		crate::types::WktBaseTypeResult<Self>,
		crate::error::WktParseError,
	>
	where
		I: IntoIterator<Item = &'a crate::ast::WktNode>,
	{
		let node = match wkt_nodes.into_iter().next() {
			Some(x) => x,
			None => return Err(WktParseError::NotEnoughNodes),
		};

		return match node.keyword {
			Keywords::Parameter => Ok(WktBaseTypeResult {
				result: Self::OperationParameter(
					OperationParameter::from_nodes(vec![node])?.result,
				),
				consumed: 1,
			}),
			Keywords::ParameterFile => Ok(WktBaseTypeResult {
				result: Self::OperationParameterFile(
					OperationParameterFile::from_nodes(vec![node])?.result,
				),
				consumed: 1,
			}),
			_ => Err(WktParseError::IncorrectKeyword {
				expected: vec![Keywords::Parameter, Keywords::ParameterFile]
					.into(),
				found: node.keyword.clone(),
			}),
		};
	}
}
