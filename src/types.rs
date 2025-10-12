use crate::{
    ast::{WktArg, WktNode},
    base_types::{
        AngleUnit, AreaDescription, Axis, BaseEngineeringCrs, BaseGeodeticCrs, BaseParametricCrs,
        BaseProjectedCrs, BaseTemporalCrs, BaseVerticalCrs, Bearing, Calendar, Citation,
        CompoundCrs, CoordinateEpoch, CoordinateMetadata, CoordinateSystem, DatumAnchor,
        DatumEnsembleAccuracy, DatumEnsembleMember, DeformationModelId, DerivedCrsConversionMethod,
        DerivedCrsConversionParameter, DerivedCrsConversionParameterFile, DerivedEngineeringCrs,
        DerivedGeodeticCrs, DerivedParametricCrs, DerivedProjectedCrs, DerivedTemporalCrs,
        DerivedVerticalCrs, DerivingConversion, DynamicCrs, Ellipsoid, EngineeringCrs,
        EngineeringDatum, Extent, FrameEpoch, GeodeticCrs, GeodeticDatumEnsemble,
        GeodeticReferenceFrame, GeographicBoundingBox, GeographicCrs, GeoidModelId, Id, LengthUnit,
        MapProjection, MapProjectionMethod, MapProjectionParameter, Meridian, OperationMethod,
        Order, ParametricCrs, ParametricDatum, ParametricUnit, PrimeMeridian, ProjectedCrs, Remark,
        ScaleUnit, Scope, TemporalDatum, TemporalExtent, TimeCrs, TimeOrigin, TimeUnit, Usage,
        VerticalCrs, VerticalDatumEnsemble, VerticalExtent, VerticalReferenceFrame,
    },
    error::WktParseError,
};

pub struct WktBaseTypeResult<T: WktBaseType + Sized> {
    pub result: T,
    pub consumed: usize,
}

pub trait WktBaseType
where
    Self: Sized,
{
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>;
}

pub struct WktInlineResult<T: WktInlineType + Sized> {
    pub result: T,
    pub consumed: usize,
}

pub trait WktInlineType
where
    Self: Sized,
{
    fn from_args<'a, I>(wkt_args: I) -> Result<WktInlineResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktArg>;
}

pub enum WktCrsTypes {
    Scope(Scope),
    Extent(Extent),
    Usage(Usage),
    AreaDescription(AreaDescription),
    GeographicBoundingBox(GeographicBoundingBox),
    VerticalExtent(VerticalExtent),
    TemporalExtent(TemporalExtent),
    Identifier(Id),
    Citation(Citation),
    Remark(Remark),
    LengthUnit(LengthUnit),
    ParametricUnit(ParametricUnit),
    ScaleUnit(ScaleUnit),
    TimeUnit(TimeUnit),
    AngleUnit(AngleUnit),
    Axis(Axis),
    CoordinateSystem(CoordinateSystem),
    Meridian(Meridian),
    Bearing(Bearing),
    Order(Order),
    GeodeticDatumEnsemble(GeodeticDatumEnsemble),
    VerticalDatumEnsemble(VerticalDatumEnsemble),
    DatumEnsembleMember(DatumEnsembleMember),
    DatumEnsembleAccuracy(DatumEnsembleAccuracy),
    DynamicCrs(DynamicCrs),
    FrameEpoch(FrameEpoch),
    DeformationModelId(DeformationModelId),
    GeodeticCrs(GeodeticCrs),
    GeographicCrs(GeographicCrs),
    Ellipsoid(Ellipsoid),
    PrimeMeridian(PrimeMeridian),
    GeodeticReferenceFrame(GeodeticReferenceFrame),
    ProjectedCrs(ProjectedCrs),
    BaseGeodeticCrs(BaseGeodeticCrs),
    MapProjection(MapProjection),
    MapProjectionMethod(MapProjectionMethod),
    MapProjectionParameter(MapProjectionParameter),
    VerticalCrs(VerticalCrs),
    GeoidModel(GeoidModelId),
    VerticalReferenceFrame(VerticalReferenceFrame),
    DatumAnchor(DatumAnchor),
    EngineeringCrs(EngineeringCrs),
    EngineeringDatum(EngineeringDatum),
    ParametricCrs(ParametricCrs),
    ParametricDatum(ParametricDatum),
    TimeCrs(TimeCrs),
    TemporalDatum(TemporalDatum),
    TimeOrigin(TimeOrigin),
    Calendar(Calendar),
    DerivingConversion(DerivingConversion),
    DerivedCrsConversionMethod(DerivedCrsConversionMethod),
    DerivedCrsConversionParameter(DerivedCrsConversionParameter),
    DerivedCrsConversionParameterFile(DerivedCrsConversionParameterFile),
    DerivedGeodeticCrs(DerivedGeodeticCrs),
    DerivedProjectedCrs(DerivedProjectedCrs),
    BaseProjectedCrs(BaseProjectedCrs),
    DerivedVerticalCrs(DerivedVerticalCrs),
    BaseVerticalCrs(BaseVerticalCrs),
    DerivedEngineeringCrs(DerivedEngineeringCrs),
    BaseEngineeringCrs(BaseEngineeringCrs),
    DerivedParametricCrs(DerivedParametricCrs),
    BaseParametricCrs(BaseParametricCrs),
    DerivedTemporalCrs(DerivedTemporalCrs),
    BaseTemporalCrs(BaseTemporalCrs),
    CompoundCrs(CompoundCrs),
    CoordinateEpoch(CoordinateEpoch),
    CoordinateMetadata(CoordinateMetadata),
    // CoordinateOperation(CoordinateOperation),
    // OperationVersion(OperationVersion),
    // SourceCrs(SourceCrs),
    // TargetCrs(TargetCrs),
    OperationMethod(OperationMethod),
    // OperationParameter(OperationParameter),
    // OperationParameterFile(OperationParameterFile),
    // InterpolationCrs(InterpolationCrs),
    // OperationAccuracy(OperationAccuracy),
    // PointMotionOperation(PointMotionOperation),
    // ConcatenatedOperation(ConcatenatedOperation),
    // BoundCrs(BoundCrs),
    // AbridgedCoordinateTransformation(AbridgedCoordinateTransformation),
}
