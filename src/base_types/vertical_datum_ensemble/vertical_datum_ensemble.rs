use crate::{
	arity::lower_bound_arity,
	ast::{Parse, WktArg, WktNode},
	base_types::{DatumEnsembleAccuracy, DatumEnsembleMember, Id},
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct VerticalDatumEnsemble {
	pub datum_ensemble_name: String,
	pub datum_ensemble_member: Vec<DatumEnsembleMember>,
	pub datum_ensemble_accuracy: DatumEnsembleAccuracy,
	pub identifier: Option<Id>, // TODO: Allows multiple
}

impl WktBaseType for VerticalDatumEnsemble {
	fn from_nodes<'a, I>(
		wkt_nodes: I,
	) -> Result<WktBaseTypeResult<Self>, WktParseError>
	where
		I: IntoIterator<Item = &'a WktNode>,
	{
		// First Part
		let base_node = match wkt_nodes.into_iter().next() {
			Some(x) => x,
			None => return Err(WktParseError::NotEnoughNodes),
		};

		match_keywords(&base_node.keyword, vec![Keywords::Ensemble])?;
		lower_bound_arity(base_node.args.len(), 2)?; // TODO How to handle arity in this case?

		let datum_ensemble_name = base_node.args[0].parse()?;

		let mut datum_ensemble_member = vec![];
		let mut datum_ensemble_accuracy = None;
		let mut identifier = None;

		for i in 1..base_node.args.len() {
			let this_value = &base_node.args[i];

			match this_value {
				WktArg::Node(node) => match node.keyword {
					Keywords::Member => {
						let param = node.parse()?;

						datum_ensemble_member.push(param);
					}
					Keywords::EnsembleAccuracy => {
						if identifier.is_some() {
							return Err(WktParseError::IncorrectKeywordOrder);
						}

						if datum_ensemble_accuracy.is_some() {
							return Err(WktParseError::TooManyKeyword(
								Keywords::EnsembleAccuracy,
							));
						}

						datum_ensemble_accuracy = Some(node.parse()?);
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
				},
				_ => return Err(WktParseError::ExpectedNode),
			}
		}

		// Final validation

		let datum_ensemble_accuracy = match datum_ensemble_accuracy {
			Some(x) => x,
			None => {
				return Err(WktParseError::TooFewKeyword(
					Keywords::EnsembleAccuracy,
				));
			}
		};

		let res = VerticalDatumEnsemble {
			datum_ensemble_name,
			datum_ensemble_member,
			datum_ensemble_accuracy,
			identifier,
		};

		Ok(WktBaseTypeResult {
			result: res,
			consumed: 1,
		})
	}
}
