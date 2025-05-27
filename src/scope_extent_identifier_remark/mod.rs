mod extent;
mod id;
mod remark;
mod scope;
mod scope_extent_identifier_remark;
mod usage;

pub use extent::{
    AreaDescription, DateOrString, Extent, GeographicBoundingBox, TemporalExtent, VerticalExtent,
};
pub use id::Id;
pub use remark::Remark;
pub use scope::Scope;
pub use scope_extent_identifier_remark::ScopeExtentIdentifierRemark;
pub use usage::Usage;
