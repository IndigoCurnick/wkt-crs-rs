use crate::{
	arity::match_arity,
	ast::{Parse, WktArg, WktNode},
	base_types::{AnchorEpoch, DatumAnchor, Ellipsoid, Id, PrimeMeridian},
	error::WktParseError,
	keywords::{Keywords, match_keywords},
	types::{WktBaseType, WktBaseTypeResult, WktInlineResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct GeodeticReferenceFrame {
	pub datum_name: String,
	pub ellipsoid: Ellipsoid,
	pub anchor: Option<DatumAnchor>,
	pub anchor_epoch: Option<AnchorEpoch>,
	pub identifier: Option<Id>, // TODO: technically multiple allowed
	pub prime_meridian: Option<PrimeMeridian>,
}

impl WktInlineType for GeodeticReferenceFrame {
	fn from_args<'a, I>(
		wkt_args: I,
	) -> Result<crate::types::WktInlineResult<Self>, WktParseError>
	where
		I: IntoIterator<Item = &'a crate::ast::WktArg>,
	{
		// Note that only nodes are necessary, so we can basically iterate the
		// args, consuming all nodes
		// when we hit not a node we can stop and just throw it into the base type
		let mut it = wkt_args.into_iter();

		let mut nodes = vec![];

		while let Some(arg) = it.next() {
			let node = match arg {
				WktArg::Node(n) => n,
				_ => break,
			};

			nodes.push(node);
		}

		let res = GeodeticReferenceFrame::from_nodes(nodes)?;

		return Ok(WktInlineResult {
			consumed: res.consumed,
			result: res.result,
		});
	}
}

impl WktBaseType for GeodeticReferenceFrame {
	fn from_nodes<'a, I>(
		wkt_nodes: I,
	) -> Result<WktBaseTypeResult<Self>, WktParseError>
	where
		I: IntoIterator<Item = &'a WktNode>,
	{
		let mut it = wkt_nodes.into_iter();

		let node = match it.next() {
			Some(x) => x,
			None => return Err(WktParseError::NotEnoughNodes),
		};

		match_keywords(
			&node.keyword,
			vec![Keywords::Datum, Keywords::TRF, Keywords::GeodeticDatum],
		)?;
		match_arity(node.args.len(), 2, 6)?;

		let datum_name = node.args[0].parse()?;
		let ellipsoid = node.args[1].parse()?;

		let mut i = 2;

		let mut anchor = None;
		let mut anchor_epoch = None;
		let mut identifier = None;

		while i < node.args.len() {
			let arg = match node.args.get(i) {
				Some(x) => x,
				None => break,
			};

			match arg {
				WktArg::Data(_) => return Err(WktParseError::ExpectedNode),
				WktArg::Node(wkt_node) => match wkt_node.keyword {
					Keywords::Id => {
						identifier = Some(wkt_node.parse()?);
						i += 1;
					}
					Keywords::Anchor => {
						anchor = Some(wkt_node.parse()?);
						i += 1;
					}
					Keywords::AnchorEpoch => {
						anchor_epoch = Some(wkt_node.parse()?);
						i += 1;
					}
					_ => {
						return Err(WktParseError::IncorrectKeyword {
							expected: vec![Keywords::Anchor, Keywords::Id]
								.into(),
							found: wkt_node.keyword.clone(),
						});
					}
				},
			}
		}

		// Second node

		let next = it.next();

		let prime_meridian = match next {
			Some(second) => match second.parse() {
				Ok(x) => Some(x),
				Err(_) => None,
			},
			None => None,
		};

		let consumed = if prime_meridian.is_some() { 2 } else { 1 };

		let datum = GeodeticReferenceFrame {
			datum_name,
			ellipsoid,
			anchor,
			anchor_epoch,
			identifier,
			prime_meridian,
		};

		let res = WktBaseTypeResult {
			consumed: consumed,
			result: datum,
		};

		Ok(res)
	}
}
