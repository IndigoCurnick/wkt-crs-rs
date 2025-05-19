use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    id::Id,
    keywords::{AXIS, CS, ID},
};

use super::{
    ordinal_date_time_axis::OrdinalDateTimeAxis, ordinal_date_time_cs_type::OrdinalDateTimeCsType,
    spatial_coordinate_system::Dimension,
};

pub struct OrdinalDateTimeCoordinateSystem {
    pub ordinal_date_time_cs_type: OrdinalDateTimeCsType,
    pub dimension: Dimension,
    pub identifier: Option<Id>, // TODO: technically the spec allows for many...
    pub ordinal_date_time_axis: Vec<OrdinalDateTimeAxis>,
}

impl TryFrom<&WktNode> for OrdinalDateTimeCoordinateSystem {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != CS {
            let expected = vec![CS.to_string()];
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

        let ordinal_date_time_cs_type = OrdinalDateTimeCsType::try_from(&value.args[0])?;
        let dimension = Dimension::try_from(&value.args[1])?;

        let mut identifier = None;
        let mut ordinal_date_time_axis = vec![];

        for i in 2..value.args.len() {
            let this_arg = &value.args[i];

            match this_arg {
                WktArg::Node(node) => match node.keyword.as_str() {
                    ID => {
                        // ID is before spatial axis and cs unti

                        if !ordinal_date_time_axis.is_empty() {
                            return Err(WktParseError::IncorrectKeywordOrder);
                        }

                        if identifier.is_some() {
                            return Err(WktParseError::TooManyKeyword(ID.to_string()));
                        }

                        identifier = Some(Id::try_from(node)?);
                    }
                    AXIS => {
                        // Spatial axis is before cs_unit

                        let oa = OrdinalDateTimeAxis::try_from(node)?;
                        ordinal_date_time_axis.push(oa);
                    }
                    _ => {
                        return Err(WktParseError::IncorrectKeyword {
                            expected: vec![ID.to_string(), AXIS.to_string()].into(),
                            found: node.keyword.clone(),
                        });
                    }
                },
                _ => return Err(WktParseError::ExpectedNode),
            }
        }

        // Must be at least one spatial axis

        if ordinal_date_time_axis.is_empty() {
            return Err(WktParseError::TooFewKeyword(AXIS.to_string()));
        }

        Ok(OrdinalDateTimeCoordinateSystem {
            ordinal_date_time_cs_type,
            dimension,
            identifier,
            ordinal_date_time_axis,
        })
    }
}
