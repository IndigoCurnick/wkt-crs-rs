mod dynamic_geographic_crs;
mod static_geographic_crs;
pub use dynamic_geographic_crs::DynamicGeographicCrs;
pub use static_geographic_crs::StaticGeographicCrs;

#[derive(Debug, PartialEq)]
pub enum GeographicCrs {
    DynamicGeographicCrs(DynamicGeographicCrs),
    StaticGeographicCrs(StaticGeographicCrs),
}
