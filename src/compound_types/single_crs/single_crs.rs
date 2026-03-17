use crate::{
	ast::{WktArg, WktNode},
	base_types::{
		DerivedEngineeringCrs, DerivedGeodeticCrs, DerivedParametricCrs,
		DerivedProjectedCrs, DerivedTemporalCrs, DerivedVerticalCrs,
		EngineeringCrs, GeodeticCrs, ParametricCrs, ProjectedCrs, TimeCrs,
		VerticalCrs,
	},
	error::WktParseError,
	keywords::Keywords,
	types::{WktBaseType, WktBaseTypeResult, WktInlineResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub enum SingleCrs {
	GeodeticCrs(GeodeticCrs),
	DerivedGeodeticCrs(DerivedGeodeticCrs),
	ProjectedCrs(ProjectedCrs),
	DerivedProjectedCrs(DerivedProjectedCrs),
	VerticalCrs(VerticalCrs),
	DerivedVerticalCrs(DerivedVerticalCrs),
	EngineeringCrs(EngineeringCrs),
	DerivedEngineeringCrs(DerivedEngineeringCrs),
	ParametricCrs(ParametricCrs),
	DerivedParametricCrs(DerivedParametricCrs),
	TimeCrs(TimeCrs),
	DerivedTemporalCrs(DerivedTemporalCrs),
}

impl WktInlineType for SingleCrs {
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

		let res = SingleCrs::from_nodes(nodes)?;

		return Ok(WktInlineResult {
			consumed: res.consumed,
			result: res.result,
		});
	}
}

impl WktBaseType for SingleCrs {
	fn from_nodes<'a, I>(
		wkt_nodes: I,
	) -> Result<WktBaseTypeResult<Self>, WktParseError>
	where
		I: IntoIterator<Item = &'a WktNode>,
	{
		let iter: Vec<&'a WktNode> = wkt_nodes.into_iter().collect();

		let first_keyword = if let Some(nod) = iter.get(0) {
			nod.keyword.clone()
		} else {
			// TODO: Just some default, if there's no nodes I guess?
			Keywords::GeodeticCrs
		};

		if let Ok(stati) = GeodeticCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: Self::GeodeticCrs(stati.result),
				consumed: stati.consumed,
			});
		}

		if let Ok(stati) = DerivedGeodeticCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: Self::DerivedGeodeticCrs(stati.result),
				consumed: stati.consumed,
			});
		}

		if let Ok(stati) = ProjectedCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: Self::ProjectedCrs(stati.result),
				consumed: stati.consumed,
			});
		}

		if let Ok(stati) = DerivedProjectedCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: Self::DerivedProjectedCrs(stati.result),
				consumed: stati.consumed,
			});
		}

		if let Ok(stati) = VerticalCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: Self::VerticalCrs(stati.result),
				consumed: stati.consumed,
			});
		}

		if let Ok(stati) = DerivedVerticalCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: Self::DerivedVerticalCrs(stati.result),
				consumed: stati.consumed,
			});
		}

		if let Ok(stati) = EngineeringCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: Self::EngineeringCrs(stati.result),
				consumed: stati.consumed,
			});
		}

		if let Ok(stati) = DerivedEngineeringCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: Self::DerivedEngineeringCrs(stati.result),
				consumed: stati.consumed,
			});
		}

		if let Ok(stati) = ParametricCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: Self::ParametricCrs(stati.result),
				consumed: stati.consumed,
			});
		}

		if let Ok(stati) = DerivedParametricCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: Self::DerivedParametricCrs(stati.result),
				consumed: stati.consumed,
			});
		}

		if let Ok(stati) = TimeCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: Self::TimeCrs(stati.result),
				consumed: stati.consumed,
			});
		}

		if let Ok(stati) = DerivedTemporalCrs::from_nodes(iter.clone()) {
			return Ok(WktBaseTypeResult {
				result: Self::DerivedTemporalCrs(stati.result),
				consumed: stati.consumed,
			});
		}

		return Err(WktParseError::CouldNotDetermineType {
			keyword: first_keyword,
		});
	}
}
