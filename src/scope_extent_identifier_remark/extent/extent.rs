use crate::{ast::WktNode, error::WktParseError};

use super::{
    area_description::AreaDescription, geographic_bounding_box::GeographicBoundingBox,
    temporal_extent::TemporalExtent, vertical_extent::VerticalExtent,
};

// TODO: Do we really need this?
pub enum Extent {
    Area(AreaDescription),
    Geographic(GeographicBoundingBox),
    Vertical(VerticalExtent),
    Temporal(TemporalExtent),
    AreaGeographic {
        area: AreaDescription,
        geographic: GeographicBoundingBox,
    },
    AreaVertical {
        area: AreaDescription,
        vertical: VerticalExtent,
    },
    AreaTemporal {
        area: AreaDescription,
        temporal: TemporalExtent,
    },
    GeographicVertical {
        geographic: GeographicBoundingBox,
        vertical: VerticalExtent,
    },
    GeographicTemporal {
        geographic: GeographicBoundingBox,
        temporal: TemporalExtent,
    },
    VerticalTemporal {
        vertical: VerticalExtent,
        temporal: TemporalExtent,
    },
    AreaGeographicVertical {
        area: AreaDescription,
        geographic: GeographicBoundingBox,
        vertical: VerticalExtent,
    },
    AreaGeographicTemporal {
        area: AreaDescription,
        geographic: GeographicBoundingBox,
        temporal: TemporalExtent,
    },
    AreaVerticalTemporal {
        area: AreaDescription,
        vertical: VerticalExtent,
        temporal: TemporalExtent,
    },
    GeographicVerticalTemporal {
        geographic: GeographicBoundingBox,
        vertical: VerticalExtent,
        temporal: TemporalExtent,
    },
    AreaGeographicVerticalTemporal {
        area: AreaDescription,
        geographic: GeographicBoundingBox,
        vertical: VerticalExtent,
        temporal: TemporalExtent,
    },
}
