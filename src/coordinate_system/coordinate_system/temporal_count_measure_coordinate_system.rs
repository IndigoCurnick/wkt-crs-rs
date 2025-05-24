use crate::{
    ast::{WktArg, WktNode},
    coordinate_system::{
        cs_type::TemporalCountMeasureCsType, dimension::Dimension,
        temporal_count_measure_axis::TemporalCountMeasureAxis,
    },
    error::WktParseError,
    keywords::CS,
    scope_extent_identifier_remark::Id,
};

pub struct TemporalCountMeasureCoordinateSystem {
    pub temporal_count_measure_cs_type: TemporalCountMeasureCsType,
    pub dimension: Dimension,
    pub identifier: Option<Id>, // TODO: Technically this can be multiple
    pub temporal_count_measure_axis: TemporalCountMeasureAxis,
}

impl TryFrom<&WktNode> for TemporalCountMeasureCoordinateSystem {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != CS {
            let expected = vec![CS.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if !(value.args.len() >= 3 || value.args.len() <= 4) {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["3-4".to_string()].into(),
                found: value.args.len(),
            });
        }

        let temporal_count_measure_cs_type = TemporalCountMeasureCsType::try_from(&value.args[0])?;

        let dimension = Dimension::try_from(&value.args[1])?;

        let mut i = 2;

        let identifier = match &value.args[i] {
            WktArg::Node(n) => match Id::try_from(n) {
                Ok(x) => {
                    i += 1;
                    Some(x)
                }
                Err(_) => None,
            },
            _ => return Err(WktParseError::ExpectedNode),
        };

        let temporal_count_measure_axis = match &value.args[i] {
            WktArg::Node(n) => TemporalCountMeasureAxis::try_from(n)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        Ok(TemporalCountMeasureCoordinateSystem {
            temporal_count_measure_cs_type,
            dimension,
            identifier,
            temporal_count_measure_axis,
        })
    }
}
