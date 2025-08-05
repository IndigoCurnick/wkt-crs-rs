use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    keywords::BASEPROJCRS,
    map_projection::MapProjection,
    proj_crs::BaseGeodeticGeographicCrs,
    scope_extent_identifier_remark::Id,
};

#[derive(Debug, PartialEq)]
pub struct BaseProjectedCrs {
    pub base_crs_name: String,
    pub base_geodetic_geographic_crs: BaseGeodeticGeographicCrs,
    pub map_projection: MapProjection,
    pub identifier: Option<Id>, // TODO: Technically allowed multiple
}

impl TryFrom<&WktNode> for BaseProjectedCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != BASEPROJCRS {
            let expected = vec![BASEPROJCRS.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if !(value.args.len() >= 3) {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["3+".to_string()].into(),
                found: value.args.len(),
            });
        }

        let base_crs_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let base_geodetic_geographic_crs = match &value.args[1] {
            WktArg::Node(x) => BaseGeodeticGeographicCrs::try_from(x)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let map_projection = match &value.args[2] {
            WktArg::Node(x) => MapProjection::try_from(x)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let identifier = match value.args.get(3) {
            Some(x) => match x {
                WktArg::Node(y) => Some(Id::try_from(y)?),
                _ => return Err(WktParseError::ExpectedNode),
            },
            None => None,
        };

        return Ok(Self {
            base_crs_name,
            base_geodetic_geographic_crs,
            map_projection,
            identifier,
        });
    }
}
