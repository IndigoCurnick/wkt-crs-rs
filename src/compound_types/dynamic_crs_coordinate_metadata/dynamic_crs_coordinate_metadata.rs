use crate::{
    ast::{WktArg, WktNode},
    base_types::{
        DerivedGeodeticCrs, DerivedProjectedCrs, DerivedVerticalCrs, DynamicGeodeticCrs,
        DynamicGeographicCrs, DynamicVerticalCrs, ProjectedCrs,
    },
    error::WktParseError,
    types::{WktBaseType, WktBaseTypeResult, WktInlineResult, WktInlineType},
};

#[derive(Debug, PartialEq)]
pub enum DynamicCrsCoordinateMetadata {
    DynamicGeodeticCrs(DynamicGeodeticCrs),
    DynamicGeographicCrs(DynamicGeographicCrs),
    ProjectedCrs(ProjectedCrs),
    DynamicVerticalCrs(DynamicVerticalCrs),
    DerivedGeodeticCrs(DerivedGeodeticCrs),
    DerivedProjectedCrs(DerivedProjectedCrs),
    DerivedVerticalCrs(DerivedVerticalCrs),
}

impl WktInlineType for DynamicCrsCoordinateMetadata {
    fn from_args<'a, I>(wkt_args: I) -> Result<WktInlineResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a crate::ast::WktArg>,
    {
        let mut it = wkt_args.into_iter();

        let mut nodes = vec![];

        while let Some(arg) = it.next() {
            let node = match arg {
                WktArg::Node(n) => n,
                _ => break,
            };

            nodes.push(node);
        }

        let res = DynamicCrsCoordinateMetadata::from_nodes(nodes)?;

        return Ok(WktInlineResult {
            consumed: res.consumed,
            result: res.result,
        });
    }
}

impl WktBaseType for DynamicCrsCoordinateMetadata {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let iter: Vec<&'a WktNode> = wkt_nodes.into_iter().collect();

        if let Ok(stati) = DynamicGeodeticCrs::from_nodes(iter.clone()) {
            return Ok(WktBaseTypeResult {
                result: Self::DynamicGeodeticCrs(stati.result),
                consumed: stati.consumed,
            });
        }

        if let Ok(stati) = DynamicGeographicCrs::from_nodes(iter.clone()) {
            return Ok(WktBaseTypeResult {
                result: Self::DynamicGeographicCrs(stati.result),
                consumed: stati.consumed,
            });
        }

        if let Ok(stati) = ProjectedCrs::from_nodes(iter.clone()) {
            return Ok(WktBaseTypeResult {
                result: Self::ProjectedCrs(stati.result),
                consumed: stati.consumed,
            });
        }

        if let Ok(stati) = DynamicVerticalCrs::from_nodes(iter.clone()) {
            return Ok(WktBaseTypeResult {
                result: Self::DynamicVerticalCrs(stati.result),
                consumed: stati.consumed,
            });
        }

        if let Ok(stati) = DerivedGeodeticCrs::from_nodes(iter.clone()) {
            return Ok(WktBaseTypeResult {
                result: Self::DerivedGeodeticCrs(stati.result),
                consumed: stati.consumed,
            });
        }

        if let Ok(stati) = DerivedProjectedCrs::from_nodes(iter.clone()) {
            return Ok(WktBaseTypeResult {
                result: Self::DerivedProjectedCrs(stati.result),
                consumed: stati.consumed,
            });
        }

        if let Ok(stati) = DerivedVerticalCrs::from_nodes(iter.clone()) {
            return Ok(WktBaseTypeResult {
                result: Self::DerivedVerticalCrs(stati.result),
                consumed: stati.consumed,
            });
        }

        return Err(WktParseError::CouldNotDetermineType);
    }
}
