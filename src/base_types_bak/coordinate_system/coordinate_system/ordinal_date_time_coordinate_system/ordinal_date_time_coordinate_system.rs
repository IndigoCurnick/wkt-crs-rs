use crate::{
    ast::WktArg,
    coordinate_system::{
        coordinate_system::coordinate_system::CsInner,
        cs_type::{CsType, OrdinalDateTimeCsType},
        dimension::Dimension,
        ordinal_date_time_axis::OrdinalDateTimeAxis,
    },
    error::WktParseError,
    scope_extent_identifier_remark::Id,
};

#[derive(Debug, PartialEq)]
pub struct OrdinalDateTimeCoordinateSystem {
    pub ordinal_date_time_cs_type: OrdinalDateTimeCsType,
    pub dimension: Dimension,
    pub identifier: Option<Id>, // TODO: technically the spec allows for many...
    pub ordinal_date_time_axis: Vec<OrdinalDateTimeAxis>,
    pub needed_args: usize, // Need to make these not `pub` as they are not part of the actual spec
                            // But instead something for me, but it would mess up the tests right now. Maybe a builder?
}

impl OrdinalDateTimeCoordinateSystem {
    pub fn needed_args(&self) -> usize {
        return self.needed_args;
    }
}

// TODO: this implementation is simpler but also means we parse the `CSInner`
// twice. It's also easier to test the individual units this way. I'm not really
// sure if we ought to keep this or implement the technically more complex but
// also marginally more efficient system. For now this should do
impl TryFrom<&[WktArg]> for OrdinalDateTimeCoordinateSystem {
    type Error = WktParseError;

    fn try_from(value: &[WktArg]) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            return Err(WktParseError::ExpectedNode);
        }

        let inner = CsInner::try_from(&value[0])?;

        // Obviously we need to be of the spatial coordinate system type here!

        let ty = match inner.cs_type {
            CsType::OrdinalDateTimeCs(t) => t,
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

            let axis = OrdinalDateTimeAxis::try_from(this_node)?;
            axes.push(axis);
        }

        let dims: usize = (&inner.dimension).into();

        return Ok(OrdinalDateTimeCoordinateSystem {
            ordinal_date_time_cs_type: ty,
            dimension: inner.dimension,
            identifier: inner.identifier,
            ordinal_date_time_axis: axes,
            needed_args: dims + 1,
        });
    }
}
