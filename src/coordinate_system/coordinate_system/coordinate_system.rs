use crate::{ast::WktNode, error::WktParseError};

use super::{
    ordinal_date_time_coordinate_system::OrdinalDateTimeCoordinateSystem,
    spatial_coordinate_system::SpatialCoordinateSystem,
    temporal_count_measure_coordinate_system::TemporalCountMeasureCoordinateSystem,
};

pub enum CoordinateSystem {
    SpatialCS(SpatialCoordinateSystem),
    TemporalCountMeasureCS(TemporalCountMeasureCoordinateSystem),
    OrdinalDateTimeCS(OrdinalDateTimeCoordinateSystem),
}

impl TryFrom<&WktNode> for CoordinateSystem {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        match SpatialCoordinateSystem::try_from(value) {
            Ok(x) => return Ok(Self::SpatialCS(x)),
            Err(_) => {}
        }

        match TemporalCountMeasureCoordinateSystem::try_from(value) {
            Ok(x) => return Ok(Self::TemporalCountMeasureCS(x)),
            Err(_) => {}
        }

        match OrdinalDateTimeCoordinateSystem::try_from(value) {
            Ok(x) => return Ok(Self::OrdinalDateTimeCS(x)),
            Err(_) => {}
        }

        return Err(WktParseError::CouldNotDetermineType);
    }
}
