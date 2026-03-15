use crate::{
	ast::{Parse, WktArg},
	base_types::{
		AreaDescription, GeographicBoundingBox, TemporalExtent, VerticalExtent,
	},
	error::WktParseError,
	keywords::Keywords,
	types::{WktInlineResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub struct Extent {
	pub area_description: Option<AreaDescription>,
	pub geographic_bounding_box: Option<GeographicBoundingBox>,
	pub vertical_extent: Option<VerticalExtent>,
	pub temporal_extent: Option<TemporalExtent>,
}

impl WktInlineType for Extent {
	fn from_args<'a, I>(
		wkt_nodes: I,
	) -> Result<WktInlineResult<Self>, WktParseError>
	where
		I: IntoIterator<Item = &'a WktArg>,
	{
		let mut area = None;
		let mut geo = None;
		let mut vert = None;
		let mut temp = None;

		let mut i = 0;

		let mut it = wkt_nodes.into_iter();
		while let Some(arg) = it.next() {
			match arg {
				WktArg::Node(node) => match node.keyword {
					Keywords::Area => {
						if area.is_some() {
							return Err(WktParseError::TooManyKeyword(
								Keywords::Area,
							));
						}
						i += 1;
						area = Some(node.parse()?);
					}
					Keywords::BBox => {
						if geo.is_some() {
							return Err(WktParseError::TooManyKeyword(
								Keywords::BBox,
							));
						}
						i += 1;
						geo = Some(node.parse()?);
					}
					Keywords::VerticalExtent => {
						if vert.is_some() {
							return Err(WktParseError::TooManyKeyword(
								Keywords::VerticalExtent,
							));
						}
						i += 1;
						vert = Some(node.parse()?);
					}
					Keywords::TimeExtent => {
						if temp.is_some() {
							return Err(WktParseError::TooManyKeyword(
								Keywords::TimeExtent,
							));
						}
						i += 1;
						temp = Some(node.parse()?);
					}
					_ => {
						return Err(WktParseError::IncorrectKeyword {
							expected: vec![
								Keywords::Area,
								Keywords::BBox,
								Keywords::VerticalExtent,
								Keywords::TimeExtent,
							]
							.into(),
							found: node.keyword.clone(),
						});
					}
				},
				WktArg::Data(_) => return Err(WktParseError::ExpectedNode),
			}
		}

		if i == 0 {
			return Err(WktParseError::NotEnoughNodes);
		}

		let extent = Extent {
			area_description: area,
			geographic_bounding_box: geo,
			vertical_extent: vert,
			temporal_extent: temp,
		};

		let res = WktInlineResult {
			consumed: i,
			result: extent,
		};

		Ok(res)
	}
}
