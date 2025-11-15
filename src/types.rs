use crate::{
    ast::{WktArg, WktNode},
    base_types::{
        AbridgedCoordinateTransformation, AngleUnit, AreaDescription, Axis, BaseEngineeringCrs,
        BaseGeodeticCrs, BaseParametricCrs, BaseProjectedCrs, BaseTemporalCrs, BaseVerticalCrs,
        Bearing, BoundCrs, Calendar, Citation, CompoundCrs, ConcatenatedOperation, CoordinateEpoch,
        CoordinateMetadata, CoordinateOperation, CoordinateSystem, DatumAnchor,
        DatumEnsembleAccuracy, DatumEnsembleMember, DeformationModelId, DerivedCrsConversionMethod,
        DerivedEngineeringCrs, DerivedGeodeticCrs, DerivedParametricCrs, DerivedProjectedCrs,
        DerivedTemporalCrs, DerivedVerticalCrs, DerivingConversion, DynamicCrs, Ellipsoid,
        EngineeringCrs, EngineeringDatum, Extent, FrameEpoch, GeodeticCrs, GeodeticDatumEnsemble,
        GeodeticReferenceFrame, GeographicBoundingBox, GeographicCrs, GeoidModelId, Id,
        InterpolationCrs, LengthUnit, MapProjection, MapProjectionMethod, MapProjectionParameter,
        Meridian, OperationAccuracy, OperationMethod, OperationParameter, OperationParameterFile,
        OperationVersion, Order, ParametricCrs, ParametricDatum, ParametricUnit,
        PointMotionOperation, PrimeMeridian, ProjectedCrs, Remark, ScaleUnit, Scope, SourceCrs,
        TargetCrs, TemporalDatum, TemporalExtent, TimeCrs, TimeOrigin, TimeUnit, Uri, Usage,
        VerticalCrs, VerticalDatumEnsemble, VerticalExtent, VerticalReferenceFrame,
    },
    compound_types::Unit,
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
    DerivedCrsConversionParameter(OperationParameter),
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
    CoordinateOperation(CoordinateOperation),
    OperationVersion(OperationVersion),
    SourceCrs(SourceCrs),
    TargetCrs(TargetCrs),
    OperationMethod(OperationMethod),
    OperationParameter(OperationParameter),
    OperationParameterFile(OperationParameterFile),
    InterpolationCrs(InterpolationCrs),
    OperationAccuracy(OperationAccuracy),
    PointMotionOperation(PointMotionOperation),
    ConcatenatedOperation(ConcatenatedOperation),
    BoundCrs(BoundCrs),
    AbridgedCoordinateTransformation(AbridgedCoordinateTransformation),
    Unit(Unit),
    Uri(Uri),
}

impl WktBaseType for WktCrsTypes {
    fn from_nodes<'a, I>(wkt_nodes: I) -> Result<WktBaseTypeResult<Self>, WktParseError>
    where
        I: IntoIterator<Item = &'a WktNode>,
    {
        let iter: Vec<&'a WktNode> = wkt_nodes.into_iter().collect();

        if iter.is_empty() {
            return Err(WktParseError::ExpectedNode);
        }

        let node = iter[0];

        return match node.keyword {
            crate::keywords::Keywords::AbridgedTransformation => {
                let tmp = AbridgedCoordinateTransformation::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::AbridgedCoordinateTransformation(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Anchor => todo!(),
            crate::keywords::Keywords::AngleUnit => {
                let tmp = AngleUnit::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::AngleUnit(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Area => todo!(),
            crate::keywords::Keywords::Axis => todo!(),
            crate::keywords::Keywords::BaseEngCrs => todo!(),
            crate::keywords::Keywords::BaseGeodCrs => todo!(),
            crate::keywords::Keywords::BaseGeogCrs => todo!(),
            crate::keywords::Keywords::BaseParamCrs => todo!(),
            crate::keywords::Keywords::BaseProjCrs => todo!(),
            crate::keywords::Keywords::BaseTimeCrs => todo!(),
            crate::keywords::Keywords::BaseVertCrs => todo!(),
            crate::keywords::Keywords::BBox => {
                let tmp = GeographicBoundingBox::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::GeographicBoundingBox(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Bearing => todo!(),
            crate::keywords::Keywords::BoundCrs => todo!(),
            crate::keywords::Keywords::Calendar => {
                let tmp = Calendar::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::Calendar(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Citation => {
                let tmp = Citation::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::Citation(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::CompoundCrs => todo!(),
            crate::keywords::Keywords::ConcatenatedOperation => todo!(),
            crate::keywords::Keywords::Conversion => todo!(),
            crate::keywords::Keywords::CoordEpoch => todo!(),
            crate::keywords::Keywords::CoordinateMetadata => todo!(),
            crate::keywords::Keywords::CoordinateOperation => todo!(),
            crate::keywords::Keywords::Cs => {
                let tmp = CoordinateSystem::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::CoordinateSystem(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Datum => todo!(),
            crate::keywords::Keywords::DerivedProjCrs => todo!(),
            crate::keywords::Keywords::DerivingConversion => todo!(),
            crate::keywords::Keywords::Dynamic => todo!(),
            crate::keywords::Keywords::EDatum => todo!(),
            crate::keywords::Keywords::Ellipsoid => {
                let tmp = Ellipsoid::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::Ellipsoid(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::EngCrs => todo!(),
            crate::keywords::Keywords::EngineeringCrs => todo!(),
            crate::keywords::Keywords::EngineeringDatum => todo!(),
            crate::keywords::Keywords::Ensemble => todo!(),
            crate::keywords::Keywords::EnsembleAccuracy => todo!(),
            crate::keywords::Keywords::Epoch => todo!(),
            crate::keywords::Keywords::FrameEpoch => todo!(),
            crate::keywords::Keywords::GeodCrs => todo!(),
            crate::keywords::Keywords::GeodeticCrs => todo!(),
            crate::keywords::Keywords::GeodeticDatum => todo!(),
            crate::keywords::Keywords::GeogCrs => todo!(),
            crate::keywords::Keywords::GeographicCrs => todo!(),
            crate::keywords::Keywords::GeoidModel => todo!(),
            crate::keywords::Keywords::Id => {
                let tmp = Id::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::Identifier(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::InterpolationCrs => todo!(),
            crate::keywords::Keywords::LengthUnit => {
                let tmp = LengthUnit::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::LengthUnit(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Member => todo!(),
            crate::keywords::Keywords::Meridian => {
                let tmp = Meridian::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::Meridian(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Method => todo!(),
            crate::keywords::Keywords::Model => todo!(),
            crate::keywords::Keywords::OperationAccuracy => todo!(),
            crate::keywords::Keywords::Order => {
                let tmp = Order::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::Order(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Parameter => todo!(),
            crate::keywords::Keywords::ParameterFile => todo!(),
            crate::keywords::Keywords::ParametricCrs => todo!(),
            crate::keywords::Keywords::ParametricDatum => todo!(),
            crate::keywords::Keywords::ParametricUnit => todo!(),
            crate::keywords::Keywords::PDatum => todo!(),
            crate::keywords::Keywords::PointMotionOperation => todo!(),
            crate::keywords::Keywords::PrimeM => todo!(),
            crate::keywords::Keywords::PrimeMeridian => todo!(),
            crate::keywords::Keywords::ProjCrs => todo!(),
            crate::keywords::Keywords::ProjectedCrs => todo!(),
            crate::keywords::Keywords::Projection => todo!(),
            crate::keywords::Keywords::Remark => {
                let tmp = Remark::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::Remark(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::ScaleUnit => {
                let tmp = ScaleUnit::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::ScaleUnit(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Scope => todo!(),
            crate::keywords::Keywords::SourceCrs => {
                let tmp = SourceCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::SourceCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Spheroid => todo!(),
            crate::keywords::Keywords::Step => todo!(),
            crate::keywords::Keywords::TargetCrs => {
                let tmp = TargetCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::TargetCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::TDatum => todo!(),
            crate::keywords::Keywords::TRF => todo!(),
            crate::keywords::Keywords::TemporalQuantity => todo!(),
            crate::keywords::Keywords::TimeCrs => todo!(),
            crate::keywords::Keywords::TimeDatum => todo!(),
            crate::keywords::Keywords::TimeExtent => todo!(),
            crate::keywords::Keywords::TimeOrigin => todo!(),
            crate::keywords::Keywords::TimeUnit => todo!(),
            crate::keywords::Keywords::Triaxial => todo!(),
            crate::keywords::Keywords::Unit => {
                let tmp = Unit::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::Unit(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Uri => {
                let tmp = Uri::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::Uri(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Usage => todo!(),
            crate::keywords::Keywords::VDatum => todo!(),
            crate::keywords::Keywords::VelocityGrid => todo!(),
            crate::keywords::Keywords::Version => todo!(),
            crate::keywords::Keywords::VertCrs => todo!(),
            crate::keywords::Keywords::VerticalCrs => todo!(),
            crate::keywords::Keywords::VerticalDatum => todo!(),
            crate::keywords::Keywords::VerticalExtent => todo!(),
            crate::keywords::Keywords::VRF => todo!(),
        };
    }
}
