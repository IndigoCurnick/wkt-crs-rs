mod area_description;
mod extent;
mod geographic_bounding_box;
mod temporal_extent;
mod vertical_extent;

#[cfg(test)]
mod tests;

pub use area_description::AreaDescription;
pub use extent::Extent;
pub use geographic_bounding_box::GeographicBoundingBox;
pub use temporal_extent::{DateOrString, TemporalExtent};
pub use vertical_extent::VerticalExtent;
