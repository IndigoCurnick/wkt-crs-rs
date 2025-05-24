use crate::{ast::WktArg, error::WktParseError};

use super::{
    ordinal_date_time_cs_type::OrdinalDateTimeCsType, spatial_cs_type::SpatialCsType,
    temporal_count_measure_cs_type::TemporalCountMeasureCsType,
};

#[derive(Debug, PartialEq)]
pub enum CsType {
    SpatialCs(SpatialCsType),
    TemporalCountMeasureCs(TemporalCountMeasureCsType),
    OrdinalDateTimeCs(OrdinalDateTimeCsType),
}

impl TryFrom<&WktArg> for CsType {
    type Error = WktParseError;

    fn try_from(value: &WktArg) -> Result<Self, Self::Error> {
        match SpatialCsType::try_from(value) {
            Ok(x) => return Ok(Self::SpatialCs(x)),
            Err(_) => {}
        };

        match TemporalCountMeasureCsType::try_from(value) {
            Ok(x) => return Ok(Self::TemporalCountMeasureCs(x)),
            Err(_) => {}
        };

        match OrdinalDateTimeCsType::try_from(value) {
            Ok(x) => return Ok(Self::OrdinalDateTimeCs(x)),
            Err(_) => {}
        };

        return Err(WktParseError::CouldNotDetermineType);
    }
}
