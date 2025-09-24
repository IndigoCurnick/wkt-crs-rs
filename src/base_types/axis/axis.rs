use crate::{
    ast::{Parse, WktNode},
    base_types::axis::{
        ordinal_date_time_axis::OrdinalDateTimeAxis, spatial_axis::SpatialAxis,
        temporal_count_measure_axis::TemporalCountMeasureAxis,
    },
    error::WktParseError,
    types::{WktBaseType, WktBaseTypeResult},
};

pub enum Axis {
    OrdinalDateTimeAxis(OrdinalDateTimeAxis),
    SpatialAxis(SpatialAxis),
    TemporalCountMeasureAxis(TemporalCountMeasureAxis),
}

impl WktBaseType for Axis {
    fn from_nodes<'a, I>(
        wkt_nodes: I,
    ) -> Result<crate::types::WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let node = match wkt_nodes.into_iter().next() {
            Some(x) => x,
            None => return Err(WktParseError::NotEnoughNodes),
        };

        if let Ok(ordinal) = node.parse() {
            return Ok(WktBaseTypeResult {
                result: Self::OrdinalDateTimeAxis(ordinal),
                consumed: 1,
            });
        }

        if let Ok(temporal) = node.parse() {
            return Ok(WktBaseTypeResult {
                result: Self::OrdinalDateTimeAxis(temporal),
                consumed: 1,
            });
        }

        if let Ok(spatial) = node.parse() {
            return Ok(WktBaseTypeResult {
                result: Self::OrdinalDateTimeAxis(spatial),
                consumed: 1,
            });
        }

        return Err(WktParseError::CouldNotDetermineType);
    }
}
