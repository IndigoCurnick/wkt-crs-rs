mod coordinate_metadata;

pub use coordinate_metadata::{
	CoordinateMetadata, DynamicCoordinateMetadata, StaticCoordinateMetadata,
};

#[cfg(test)]
mod tests;
