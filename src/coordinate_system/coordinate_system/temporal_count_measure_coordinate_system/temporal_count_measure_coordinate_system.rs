use crate::{
    ast::{WktArg, WktNode},
    coordinate_system::{
        coordinate_system::coordinate_system::CsInner,
        cs_type::{CsType, TemporalCountMeasureCsType},
        dimension::Dimension,
        temporal_count_measure_axis::TemporalCountMeasureAxis,
    },
    error::WktParseError,
    keywords::CS,
    scope_extent_identifier_remark::Id,
};

#[derive(Debug, PartialEq)]
pub struct TemporalCountMeasureCoordinateSystem {
    pub temporal_count_measure_cs_type: TemporalCountMeasureCsType,
    pub dimension: Dimension,
    pub identifier: Option<Id>, // TODO: Technically this can be multiple
    pub temporal_count_measure_axis: TemporalCountMeasureAxis,
}

// TODO: this implementation is simpler but also means we parse the `CSInner`
// twice. It's also easier to test the individual units this way. I'm not really
// sure if we ought to keep this or implement the technically more complex but
// also marginally more efficient system. For now this should do
impl TryFrom<&[WktNode]> for TemporalCountMeasureCoordinateSystem {
    type Error = WktParseError;

    fn try_from(value: &[WktNode]) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            return Err(WktParseError::ExpectedNode);
        }

        let inner = CsInner::try_from(&value[0])?;

        // Obviously we need to be of the spatial coordinate system type here!

        let ty = match inner.cs_type {
            CsType::TemporalCountMeasureCs(t) => t,
            _ => return Err(WktParseError::IncorrectValue),
        };

        let dim = <&Dimension as Into<usize>>::into(&inner.dimension); // * I'm pretty surprised I needed to do this and Rust couldn't infer the type inline

        if value.len() < dim + 1 {
            return Err(WktParseError::IncorrectArity {
                expected: vec![format!("{}+", dim + 1)].into(),
                found: value.len(),
            });
        }

        // TemporalCountMeasureCS is only allowed one axis
        if dim != 1 {
            return Err(WktParseError::IncorrectArity {
                expected: vec![format!("{}+", dim + 1)].into(),
                found: value.len(),
            });
        }

        // From 1 to the number of claimed axis, we should find an axis
        let axis = TemporalCountMeasureAxis::try_from(&value[1])?;

        return Ok(TemporalCountMeasureCoordinateSystem {
            temporal_count_measure_cs_type: ty,
            dimension: inner.dimension,
            identifier: inner.identifier,
            temporal_count_measure_axis: axis,
        });
    }
}
