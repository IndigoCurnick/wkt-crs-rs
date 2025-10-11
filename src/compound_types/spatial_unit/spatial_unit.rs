use log::error;

use crate::{
    ast::WktNode,
    base_types::{AngleUnit, LengthUnit, ParametricUnit, ScaleUnit},
    error::WktParseError,
    keywords::Keywords,
    types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub enum SpatialUnit {
    AngleUnit(AngleUnit),
    LengthUnit(LengthUnit),
    ParametricUnit(ParametricUnit),
    ScaleUnit(ScaleUnit),
}

impl WktBaseType for SpatialUnit {
    fn from_nodes<'a, I>(
        wkt_nodes: I,
    ) -> Result<crate::types::WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        // Take 1

        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        let unit = match &node.keyword {
            Keywords::LengthUnit => {
                SpatialUnit::LengthUnit(LengthUnit::from_nodes(vec![node])?.result)
            }
            Keywords::AngleUnit => {
                SpatialUnit::AngleUnit(AngleUnit::from_nodes(vec![node])?.result)
            }
            Keywords::ScaleUnit => {
                SpatialUnit::ScaleUnit(ScaleUnit::from_nodes(vec![node])?.result)
            }
            Keywords::ParametricUnit => {
                SpatialUnit::ParametricUnit(ParametricUnit::from_nodes(vec![node])?.result)
            }
            Keywords::Unit => {
                error!(
                    "Ambiguous Keyword `UNIT`. While older versions of the specification allowed for this keyword, in this context it is ambiguous and should be depreciated in favour of `LENGTHUNIT`, `ANGLEUNIT` or `SCALEUNIT`"
                );
                return Err(WktParseError::IncorrectKeyword {
                    expected: vec![
                        Keywords::LengthUnit,
                        Keywords::AngleUnit,
                        Keywords::ScaleUnit,
                        Keywords::ParametricUnit,
                    ]
                    .into(),
                    found: node.keyword.clone(),
                });
            }
            _ => {
                return Err(WktParseError::IncorrectKeyword {
                    expected: vec![
                        Keywords::LengthUnit,
                        Keywords::AngleUnit,
                        Keywords::ScaleUnit,
                        Keywords::ParametricUnit,
                    ]
                    .into(),
                    found: node.keyword.clone(),
                });
            }
        };

        return Ok(WktBaseTypeResult {
            result: unit,
            consumed: 1,
        });
    }
}
