use crate::{
    ast::WktArg,
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
    pub needed_args: usize, // Need to make these not `pub` as they are not part of the actual spec
                            // But instead something for me, but it would mess up the tests right now. Maybe a builder?
}

// TODO: We might not even need to store the needed args but we could reconstruct it
// based on the params in the struct?
impl SpatialCoordinateSystem {
    pub fn needed_args(&self) -> usize {
        return self.needed_args;
    }
}

// TODO: this implementation is simpler but also means we parse the `CSInner`
// twice. It's also easier to test the individual units this way. I'm not really
// sure if we ought to keep this or implement the technically more complex but
// also marginally more efficient system. For now this should do
impl TryFrom<&[WktArg]> for SpatialCoordinateSystem {
    type Error = WktParseError;

    fn try_from(value: &[WktArg]) -> Result<Self, Self::Error> {
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
            let this_arg = &value[i];

            let this_node = match this_arg {
                WktArg::Node(node) => node,
                _ => return Err(WktParseError::ExpectedNode),
            };

            let axis = SpatialAxis::try_from(this_node)?;
            axes.push(axis);
        }

        // After the axis, we may then optionally find a unit
        // It must be a spatial unit
        let unit = match value.get(dim + 1) {
            None => None,
            Some(x) => {
                match x {
                    WktArg::Node(node) => {
                        if is_unit_keyword(&node.keyword) {
                            match Unit::try_from(node) {
                                Ok(y) => Some(y),
                                Err(z) => return Err(z),
                            }
                        } else {
                            // It's not an error because it could just be some unrelated node!!
                            None
                        }
                    }
                    _ => None,
                }
            }
        };

        let needed_args = match unit {
            Some(_) => 1 + dim + 1,
            None => 1 + dim,
        };

        return Ok(SpatialCoordinateSystem {
            spatial_cs_type: ty,
            dimension: inner.dimension,
            identifier: inner.identifier,
            spatial_axis: axes,
            cs_unit: unit,
            needed_args,
        });
    }
}
