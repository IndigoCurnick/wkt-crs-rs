use crate::{
    ast::{WktArg, WktNode},
    coordinate_system::CoordinateSystem,
    error::WktParseError,
    keywords::PARAMETRICCRS,
    parametric_crs::parametric_datum::ParametricDatum,
    scope_extent_identifier_remark::ScopeExtentIdentifierRemark,
};

#[derive(Debug, PartialEq)]
pub struct ParametricCrs {
    pub crs_name: String,
    pub parametric_datum: ParametricDatum,
    pub coordinate_system: CoordinateSystem,
    pub scope_extent_identifier_remark: ScopeExtentIdentifierRemark,
}

impl TryFrom<&WktNode> for ParametricCrs {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != PARAMETRICCRS {
            return Err(WktParseError::IncorrectKeyword {
                expected: vec![PARAMETRICCRS.into()].into(),
                found: value.keyword.clone(),
            });
        }

        if value.args.len() < 3 {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["4+".into()].into(),
                found: value.args.len(),
            });
        }

        let crs_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let parametric_datum = match &value.args[1] {
            WktArg::Node(node) => ParametricDatum::try_from(node)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAARRRRRRRRRRGGGGGGGGGGGGGGHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH
        // PLEASE JUST USE HIERARCHY IT IS PART OF YOUR LANGUAGE

        let coordinate_system = CoordinateSystem::try_from(&value.args[2..value.args.len()])?;

        let args_needed = coordinate_system.needed_args();

        // TODO: Can this fail?
        let scope_extent_identifier_remark = ScopeExtentIdentifierRemark::try_from(
            &value.args[(2 + args_needed)..value.args.len()],
        )?;

        return Ok(ParametricCrs {
            crs_name,
            parametric_datum,
            coordinate_system,
            scope_extent_identifier_remark,
        });
    }
}
