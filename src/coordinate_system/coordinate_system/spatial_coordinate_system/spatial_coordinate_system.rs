use crate::{
    ast::{WktArg, WktNode},
    coordinate_system::{
        coordinate_system::coordinate_system::CsInner,
        cs_type::{CsType, SpatialCsType},
        dimension::Dimension,
        spatial_axis::SpatialAxis,
    },
    error::WktParseError,
    scope_extent_identifier_remark::Id,
    units::{Unit, is_unit_keyword},
};

#[derive(Debug, PartialEq)]
pub struct SpatialCoordinateSystem {
    pub spatial_cs_type: SpatialCsType,
    pub dimension: Dimension,
    pub identifier: Option<Id>, // TODO: Technically the spec allows for any number of these
    pub spatial_axis: Vec<SpatialAxis>,
    pub cs_unit: Option<Unit>,
}

// TODO: this implementation is simpler but also means we parse the `CSInner`
// twice. It's also easier to test the individual units this way. I'm not really
// sure if we ought to keep this or implement the technically more complex but
// also marginally more efficient system. For now this should do
impl TryFrom<&[WktNode]> for SpatialCoordinateSystem {
    type Error = WktParseError;

    fn try_from(value: &[WktNode]) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            return Err(WktParseError::ExpectedNode);
        }

        let inner = CsInner::try_from(&value[0])?;

        // Obviously we need to be of the spatial coordinate system type here!

        let ty = match inner.cs_type {
            CsType::SpatialCs(t) => t,
            _ => return Err(WktParseError::IncorrectValue),
        };

        let dim = <&Dimension as Into<usize>>::into(&inner.dimension); // * I'm pretty surprised I needed to do this and Rust couldn't infer the type inline

        if value.len() < dim + 1 {
            return Err(WktParseError::IncorrectArity {
                expected: vec![format!("{}+", dim + 1)].into(),
                found: value.len(),
            });
        }

        // From 1 to the number of claimed axis, we should find an axis
        let mut axes = vec![];
        for i in 1..dim + 1 {
            let this_node = &value[i];

            let axis = SpatialAxis::try_from(this_node)?;
            axes.push(axis);
        }

        // After the axis, we may then optionally find a unit
        // It must be a spatial unit
        let unit = match value.get(dim + 1) {
            None => None,
            Some(x) => {
                if is_unit_keyword(&x.keyword) {
                    match Unit::try_from(x) {
                        Ok(y) => Some(y),
                        Err(z) => return Err(z),
                    }
                } else {
                    // It's not an error because it could just be some unrelated node!!
                    None
                }
            }
        };

        return Ok(SpatialCoordinateSystem {
            spatial_cs_type: ty,
            dimension: inner.dimension,
            identifier: inner.identifier,
            spatial_axis: axes,
            cs_unit: unit,
        });
    }
}
