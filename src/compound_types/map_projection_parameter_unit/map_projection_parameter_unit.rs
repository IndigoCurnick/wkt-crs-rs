use log::error;

use crate::{
    ast::WktNode,
    base_types::{AngleUnit, LengthUnit, ScaleUnit},
    error::WktParseError,
    keywords::Keywords,
    types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub enum MapProjectionParameterUnit {
    LengthUnit(LengthUnit),
    AngleUnit(AngleUnit),
    ScaleUnit(ScaleUnit),
}

impl WktBaseType for MapProjectionParameterUnit {
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
                MapProjectionParameterUnit::LengthUnit(LengthUnit::from_nodes(vec![node])?.result)
            }
            Keywords::AngleUnit => {
                MapProjectionParameterUnit::AngleUnit(AngleUnit::from_nodes(vec![node])?.result)
            }
            Keywords::ScaleUnit => {
                MapProjectionParameterUnit::ScaleUnit(ScaleUnit::from_nodes(vec![node])?.result)
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
