use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::{AREA, BBOX, Keywords, TIMEEXTENT, VERTICALEXTENT, match_keywords},
    types::WktBaseType,
};

use super::{
    area_description::AreaDescription, geographic_bounding_box::GeographicBoundingBox,
    temporal_extent::TemporalExtent, vertical_extent::VerticalExtent,
};

// TODO: Do we really need this?
#[derive(Debug, PartialEq)]
pub struct Extent {
    pub area_description: Option<AreaDescription>,
    pub geographic_bounding_box: Option<GeographicBoundingBox>,
    pub vertical_extent: Option<VerticalExtent>,
    pub temporal_extent: Option<TemporalExtent>,
}

impl WktBaseType for Extent {
    fn from_nodes(wkt_nodes: &[WktNode]) -> Result<crate::types::WktResult<Self>, WktParseError> {
        // Take 1

        let node = match wkt_nodes.first() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        if node.keyword != Keywords::Extent {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![Keywords::Extent].into(),
                found: node.keyword.clone(),
            });
        }

        match_keywords(&node.keyword, vec![Keywords::Extent])?;

        if node.args.len() == 0 || node.args.len() > 4 {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["1".into(), "2".into(), "3".into(), "4".into()].into(),
                found: node.args.len(),
            });
        }

        let scope = match &node.args[0] {
            WktArg::Data(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let res = WktResult {
            consumed: 1,
            result: Scope(scope),
        };

        Ok(res)
    }
}

impl TryFrom<&[WktArg]> for Extent {
    type Error = WktParseError;

    fn try_from(value: &[WktArg]) -> Result<Self, Self::Error> {
        if value.len() == 0 || value.len() > 4 {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["1".into(), "2".into(), "3".into(), "4".into()].into(),
                found: value.len(),
            });
        }

        let mut area = None;
        let mut geo = None;
        let mut vert = None;
        let mut temp = None;

        for i in 0..value.len() {
            let this_arg = &value[i];

            match this_arg {
                WktArg::Node(this_node) => match this_node.keyword.as_str() {
                    AREA => {
                        if area.is_some() {
                            return Err(WktParseError::TooManyKeyword(AREA.into()));
                        }

                        area = Some(AreaDescription::try_from(this_node)?)
                    }
                    BBOX => {
                        if geo.is_some() {
                            return Err(WktParseError::TooManyKeyword(BBOX.into()));
                        }

                        geo = Some(GeographicBoundingBox::try_from(this_node)?)
                    }
                    VERTICALEXTENT => {
                        if vert.is_some() {
                            return Err(WktParseError::TooManyKeyword(VERTICALEXTENT.into()));
                        }

                        vert = Some(VerticalExtent::try_from(this_node)?)
                    }
                    TIMEEXTENT => {
                        if temp.is_some() {
                            return Err(WktParseError::TooManyKeyword(TIMEEXTENT.into()));
                        }

                        temp = Some(TemporalExtent::try_from(this_node)?)
                    }
                    _ => {
                        return Err(WktParseError::IncorrectKeyword {
                            expected: vec![
                                AREA.into(),
                                BBOX.into(),
                                VERTICALEXTENT.into(),
                                TIMEEXTENT.into(),
                            ]
                            .into(),
                            found: this_node.keyword.clone(),
                        });
                    }
                },
                _ => return Err(WktParseError::ExpectedNode),
            }
        }

        // TODO: What if they're all None?
        return Ok(Extent {
            area_description: area,
            geographic_bounding_box: geo,
            vertical_extent: vert,
            temporal_extent: temp,
        });
    }
}
