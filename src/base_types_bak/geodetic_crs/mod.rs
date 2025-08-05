use crate::{datum::GeodeticReferenceFrameDatum, ensemble::GeodeticDatumEnsemble};

mod dynamic_geodetic_crs;
mod geodetic_crs;
mod geographic_crs;
mod static_geodetic_crs;

#[derive(Debug, PartialEq)]
pub enum GeodeticData {
    GeodeticReferenceFrame(GeodeticReferenceFrameDatum),
    GeodeticDatumEnsemble(GeodeticDatumEnsemble),
}
