use crate::{
    ast::WktNode,
    ensemble::{geodetic_ensemble::GeodeticDatumEnsemble, vertical_ensemble::VerticalEnsemble},
    error::WktParseError,
    keywords::PRIMEMERIDIAN,
    prime_meridian::PrimeMeridian,
};

#[derive(Debug, PartialEq)]
pub enum DataEnsemble {
    GeodeticEnsemble(GeodeticDatumEnsemble),
    VerticalEnsemble(VerticalEnsemble),
}

impl TryFrom<(&WktNode, Option<&WktNode>)> for DataEnsemble {
    type Error = WktParseError;

    fn try_from(value: (&WktNode, Option<&WktNode>)) -> Result<Self, Self::Error> {
        let (node, maybe_pm) = value;

        match VerticalEnsemble::try_from(node) {
            Ok(x) => return Ok(DataEnsemble::VerticalEnsemble(x)),
            Err(_) => {}
        };

        // Geodetic MUST have a prime meridian

        let pm = match maybe_pm {
            Some(x) => PrimeMeridian::try_from(x)?,
            None => return Err(WktParseError::TooFewKeyword(PRIMEMERIDIAN.to_string())),
        };

        return match GeodeticDatumEnsemble::try_from((node, pm)) {
            Ok(x) => Ok(DataEnsemble::GeodeticEnsemble(x)),
            Err(y) => Err(y),
        };
    }
}
