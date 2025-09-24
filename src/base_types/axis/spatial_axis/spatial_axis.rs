use crate::{
    arity::match_arity,
    ast::{Parse, WktNode},
    base_types::{Id, Order},
    compound_types::SpatialUnit,
    enumerations::AxisDirection,
    error::WktParseError,
    keywords::{Keywords, match_keywords},
    types::{WktBaseType, WktBaseTypeResult},
};

#[derive(Debug, PartialEq)]
pub struct SpatialAxis {
    pub axis_name_abbreviation: String,
    pub axis_direction: AxisDirection,
    pub axis_order: Option<Order>,
    pub spatial_unit: Option<SpatialUnit>,
    pub identifier: Option<Id>, // TODO: technically the spec allows for multiple
}

impl WktBaseType for SpatialAxis {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        // Take 1

        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        match_keywords(&node.keyword, vec![Keywords::Axis])?;
        match_arity(node.args.len(), 1, 6)?; // TODO: Double check this

        let axis_name_abbreviation = node.args[0].parse()?;

        let maybe = node.args.get(2);

        let axis_direction = AxisDirection::try_from((&node.args[1], maybe))?;

        let mut i = 2;

        if axis_direction.used_second() {
            i += 1;
        }

        let axis_order = match node.args.get(i) {
            Some(x) => match x.parse() {
                Ok(y) => {
                    i += 1;
                    Some(y)
                }
                Err(_) => None,
            },
            None => None,
        };

        let spatial_unit = match node.args.get(i) {
            Some(x) => match x.parse() {
                Ok(y) => {
                    i += 1;
                    Some(y)
                }
                Err(_) => None,
            },
            None => None,
        };

        let identifier = match node.args.get(i) {
            Some(x) => match x.parse() {
                Ok(y) => Some(y),
                Err(_) => None,
            },
            None => None,
        };

        let axis = SpatialAxis {
            axis_direction,
            axis_name_abbreviation,
            axis_order,
            spatial_unit,
            identifier,
        };

        let res = WktBaseTypeResult {
            consumed: 1,
            result: axis,
        };

        Ok(res)
    }
}
