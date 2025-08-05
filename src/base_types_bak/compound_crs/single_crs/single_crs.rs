// 15.1 - Page 83

use crate::vertical_crs::VerticalCrs;

pub enum SingleCrs {
    GeodeticCrs(GeodeticCrs),
    DerivedGeodeticCrs(DerivedProjectedCrs),
    ProjectedCrs(ProjectedCrs),
    DerivedProjectedCrs(DerivedProjectedCrs),
    VerticalCrs(VerticalCrs),
    DerivedVerticalCrs(DerivedVerticalCrs),
    EngineeringCrs(EngineeringCrs),
    DerivedEngineeringCrs(DerivedEngineeringCrs),
    ParametricCrs(ParametricCrs),
    DerivedParametricCrs(DerivedParametricCrs),
    TemporalCrs(TemporalCrs),
    DerivedTemporalCrs(DerivedTemporalCrs),
}
