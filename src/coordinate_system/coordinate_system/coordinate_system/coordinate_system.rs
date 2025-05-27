use crate::{
    ast::{WktArg, WktNode},
    coordinate_system::{
        coordinate_system::{
            ordinal_date_time_coordinate_system::OrdinalDateTimeCoordinateSystem,
            spatial_coordinate_system::SpatialCoordinateSystem,
            temporal_count_measure_coordinate_system::TemporalCountMeasureCoordinateSystem,
        },
        cs_type::CsType,
        dimension::Dimension,
    },
    error::WktParseError,
};

use super::coordinate_system_inner::CsInner;

#[derive(Debug, PartialEq)]
pub enum CoordinateSystem {
    SpatialCS(SpatialCoordinateSystem),
    TemporalCountMeasureCS(TemporalCountMeasureCoordinateSystem),
    OrdinalDateTimeCS(OrdinalDateTimeCoordinateSystem),
}

impl CoordinateSystem {
    pub fn needed_args(&self) -> usize {
        match self {
            CoordinateSystem::SpatialCS(spatial_coordinate_system) => {
                spatial_coordinate_system.needed_args()
            }
            CoordinateSystem::TemporalCountMeasureCS(temporal_count_measure_coordinate_system) => {
                temporal_count_measure_coordinate_system.needed_args()
            }
            CoordinateSystem::OrdinalDateTimeCS(ordinal_date_time_coordinate_system) => {
                ordinal_date_time_coordinate_system.needed_args()
            }
        }
    }
}

// TODO: It would also make sense to impl a TryFrom for &[WktNode]!
impl TryFrom<&[WktArg]> for CoordinateSystem {
    type Error = WktParseError;

    fn try_from(value: &[WktArg]) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            return Err(WktParseError::ExpectedNode);
        }

        let inner = CsInner::try_from(&value[0])?;

        // We need as many axis as in dimension, plus the original CS
        // Worth noting that we might also have units on the end, so it
        // _might_ be bigger but that is optional. This represents the
        // minimum length
        let dim = <&Dimension as Into<usize>>::into(&inner.dimension); // * I'm pretty surprised I needed to do this and Rust couldn't infer the type inline

        if value.len() < dim + 1 {
            return Err(WktParseError::IncorrectArity {
                expected: vec![format!("{}+", dim + 1)].into(),
                found: value.len(),
            });
        }

        let cs = match inner.cs_type {
            CsType::SpatialCs(spatial_cs_type) => {
                CoordinateSystem::SpatialCS(SpatialCoordinateSystem::try_from(value)?)
            }
            CsType::TemporalCountMeasureCs(temporal_count_measure_cs_type) => {
                CoordinateSystem::TemporalCountMeasureCS(
                    TemporalCountMeasureCoordinateSystem::try_from(value)?,
                )
            }
            CsType::OrdinalDateTimeCs(ordinal_date_time_cs_type) => {
                CoordinateSystem::OrdinalDateTimeCS(OrdinalDateTimeCoordinateSystem::try_from(
                    value,
                )?)
            }
        };

        return Ok(cs);
    }
}
