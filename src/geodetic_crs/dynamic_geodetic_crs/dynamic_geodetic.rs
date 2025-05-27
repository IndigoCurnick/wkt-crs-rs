use crate::{
    ast::{WktArg, WktNode},
    coordinate_system::CoordinateSystem,
    datum::GeodeticReferenceFrameDatum,
    dynamic_crs::DynamicCrs,
    error::WktParseError,
    keywords::{GEODCRS, GEODETICCRS},
    prime_meridian::PrimeMeridian,
    scope_extent_identifier_remark::ScopeExtentIdentifierRemark,
};

#[derive(Debug, PartialEq)]
pub struct DynamicGeodeticCrs {
    pub crs_name: String,
    pub dynamic_crs: DynamicCrs,
    pub geodetic_reference_frame: GeodeticReferenceFrameDatum,
    pub coordinate_system: CoordinateSystem,
    pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl TryFrom<&WktNode> for DynamicGeodeticCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == GEODCRS || value.keyword == GEODETICCRS) {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![GEODCRS.into(), GEODETICCRS.into()].into(),
                found: value.keyword.clone(),
            });
        }

        if value.args.len() < 4 {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["4+".into()].into(),
                found: value.args.len(),
            });
        }

        let crs_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let dynamic_crs = match &value.args[1] {
            WktArg::Node(node) => DynamicCrs::try_from(node)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let prime_meridian = match &value.args[3] {
            WktArg::Node(node) => match PrimeMeridian::try_from(node) {
                Ok(x) => Some(x),
                Err(_) => None,
            },
            _ => None,
        };

        let i = match prime_meridian {
            Some(_) => 4,
            None => 3,
        };

        let geodetic_reference_frame = match &value.args[2] {
            WktArg::Node(node) => GeodeticReferenceFrameDatum::try_from((node, prime_meridian))?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAARRRRRRRRRRGGGGGGGGGGGGGGHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH
        // PLEASE JUST USE HIERARCHY IT IS PART OF YOUR LANGUAGE

        let coordinate_system = CoordinateSystem::try_from(&value.args[i..value.args.len()])?;

        let args_needed = coordinate_system.needed_args();

        // TODO: Can this fail?
        let scope_extent_identifier_remark = ScopeExtentIdentifierRemark::try_from(
            &value.args[(i + args_needed)..value.args.len()],
        )?;

        return Ok(DynamicGeodeticCrs {
            crs_name,
            dynamic_crs,
            geodetic_reference_frame,
            coordinate_system,
            scope_extent_identifier_remark,
        });
    }
}
