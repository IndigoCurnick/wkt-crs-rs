use std::fmt::Display;

use strum::EnumString;

use crate::error::WktParseError;

#[derive(Debug, PartialEq, EnumString, Clone)]
#[strum(serialize_all = "UPPERCASE")]
#[strum(ascii_case_insensitive)]
pub enum Keywords {
    AbridgedTransformation,
    Anchor,
    AngleUnit,
    Area,
    Axis,
    BaseEngCrs,
    BaseGeodCrs,
    BaseGeogCrs,
    BaseParamCrs,
    BaseProjCrs,
    BaseTimeCrs,
    BaseVertCrs,
    BBox,
    Bearing,
    BoundCrs,
    Calendar,
    Citation,
    CompoundCrs,
    ConcatenatedOperation,
    Conversion,
    CoordEpoch,
    CoordinateMetadata,
    CoordinateOperation,
    Cs,
    Datum,
    DerivedProjCrs,
    DerivingConversion,
    Dynamic,
    EDatum,
    Ellipsoid,
    EngCrs,
    EngineeringCrs,
    EngineeringDatum,
    Ensemble,
    EnsembleAccuracy,
    Epoch,
    FrameEpoch,
    GeodCrs,
    GeodeticCrs,
    GeodeticDatum,
    GeogCrs,
    GeographicCrs,
    GeoidModel,
    Id,
    InterpolationCrs,
    LengthUnit,
    Member,
    Meridian,
    Method,
    Model,
    OperationAccuracy,
    Order,
    Parameter,
    ParameterFile,
    ParametricCrs,
    ParametricDatum,
    ParametricUnit,
    PDatum,
    PointMotionOperation,
    PrimeM,
    PrimeMeridian,
    ProjCrs,
    ProjectedCrs,
    Projection,
    Remark,
    ScaleUnit,
    Scope,
    SourceCrs,
    Spheroid,
    Step,
    TargetCrs,
    TDatum,
    TRF,
    TemporalQuantity,
    TimeCrs,
    TimeDatum,
    TimeExtent,
    TimeOrigin,
    TimeUnit,
    Triaxial,
    Unit,
    Uri,
    Usage,
    VDatum,
    VelocityGrid,
    Version,
    VertCrs,
    VerticalCrs,
    VerticalDatum,
    VerticalExtent,
    VRF,
}

impl Display for Keywords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub fn match_keywords(
    this_keyword: &Keywords,
    acceptable: Vec<Keywords>,
) -> Result<(), WktParseError> {
    return if acceptable.contains(this_keyword) {
        Ok(())
    } else {
        Err(WktParseError::IncorrectKeyword {
            expected: acceptable.into(),
            found: this_keyword.clone(),
        })
    };
}

pub const URI: &str = "URI";
pub const CITATION: &str = "CITATION";
pub const ANGLEUNIT: &str = "ANGLEUNIT";
pub const UNIT: &str = "UNIT";
pub const LENGTHUNIT: &str = "LENGTHUNIT";
pub const SCALEUNIT: &str = "SCALEUNIT";
pub const METHOD: &str = "METHOD";
pub const PROJECTION: &str = "PROJECTION";
pub const PARAMETER: &str = "PARAMETER";
pub const CONVERSION: &str = "CONVERSION";
pub const ID: &str = "ID";
pub const PARAMETRICUNIT: &str = "PARAMETRICUNIT";
pub const TIMEUNIT: &str = "TIMEUNIT";
pub const TEMPORALQUANTITY: &str = "TEMPORALQUANTITY";
pub const AXIS: &str = "AXIS";
pub const MERIDIAN: &str = "MERIDIAN";
pub const BEARING: &str = "BEARING";
pub const ORDER: &str = "ORDER";
pub const CS: &str = "CS";
pub const PRIMEM: &str = "PRIMEM";
pub const PRIMEMERIDIAN: &str = "PRIMEMERIDIAN";
pub const REMARK: &str = "REMARK";
pub const TIMEEXTENT: &str = "TIMEEXTENT";
pub const SCOPE: &str = "SCOPE";
pub const AREA: &str = "AREA";
pub const BBOX: &str = "BBOX";
pub const VERTICALEXTENT: &str = "VERTICALEXTENT";
pub const ENSEMBLE: &str = "ENSEMBLE";
pub const MEMBER: &str = "MEMBER";
pub const ENSEMBLEACCURACY: &str = "ENSEMBLEACCURACY";
pub const ELLIPSOID: &str = "ELLIPSOID";
pub const SPHEROID: &str = "SPHEROID";
pub const DYNAMIC: &str = "DYNAMIC";
pub const FRAMEEPOCH: &str = "FRAMEEPOCH";
pub const MODEL: &str = "MODEL";
pub const VELOCITYGRID: &str = "VELOCITYGRID";
pub const DATUM: &str = "DATUM";
pub const TRF: &str = "TRF";
pub const GEODETICDATUM: &str = "GEODETICDATUM";
pub const PROJCRS: &str = "PROJCRS";
pub const PROJECTEDCRS: &str = "PROJECTEDCRS";
pub const ANCHOR: &str = "ANCHOR";
pub const GEODCRS: &str = "GEODCRS";
pub const GEODETICCRS: &str = "GEODETICCRS";
pub const GEOGCRS: &str = "GEOGCRS";
pub const GEOGRAPHICCRS: &str = "GEOGRAPHICCRS";
pub const USAGE: &str = "USAGE";
pub const BASEGEODCRS: &str = "BASEGEODCRS";
pub const BASEGEOGCRS: &str = "BASEGEOGCRS";
pub const GEOIDMODEL: &str = "GEOIDMODEL";
pub const VERTCRS: &str = "VERTCRS";
pub const VERTICALCRS: &str = "VERTICALCRS";
pub const VDATUM: &str = "VDATUM";
pub const VRF: &str = "VRF";
pub const VERTICALDATUM: &str = "VERTICALDATUM";
pub const ENGCRS: &str = "ENGCRS";
pub const ENGINEERINGCRS: &str = "ENGINEERINGCRS";
pub const EDATUM: &str = "EDATUM";
pub const ENGINEERINGDATUM: &str = "ENGINEERINGDATUM";
pub const PARAMETRICCRS: &str = "PARAMETRICCRS";
pub const PDATUM: &str = "PDATUM";
pub const PARAMETRICDATUM: &str = "PARAMETRICDATUM";
pub const CALENDAR: &str = "CALENDAR";
pub const TIMEORIGIN: &str = "TIMEORIGIN";
pub const TDATUM: &str = "TDATUM";
pub const TIMEDATUM: &str = "TIMEDATUM";
pub const TIMECRS: &str = "TIMECRS";
pub const DERIVINGCONVERSION: &str = "DERIVINGCONVERSION";
pub const PARAMETERFILE: &str = "PARAMETERFILE";
pub const BASEPROJCRS: &str = "BASEPROJCRS";
pub const DERIVEDPROJCRS: &str = "DERIVEDPROJCRS";
pub const BASEVERTCRS: &str = "BASEVERTCRS";
pub const BASEENGCRS: &str = "BASEENGCRS";
pub const BASEPARAMCRS: &str = "BASEPARAMCRS";
pub const BASETIMECRS: &str = "BASETIMECRS";
