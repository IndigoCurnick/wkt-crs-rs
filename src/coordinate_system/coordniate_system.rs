use super::spatial_coordinate_system::SpatialCoordinateSystem;

pub enum CoordinateSystem {
    SpatialCS(SpatialCoordinateSystem),
    TemporalCountMeasureCS(TemporalCountMeasureCoordinateSystem),
    OrdinalDateTimeCS(OrdinalDateTimeCoordinateSystem),
}
