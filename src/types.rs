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
        SpatialAxis, TargetCrs, TemporalDatum, TemporalExtent, TimeCrs, TimeOrigin, TimeUnit, Uri,
        Usage, VerticalCrs, VerticalDatumEnsemble, VerticalExtent, VerticalReferenceFrame,
    },
    compound_types::{Step, Unit},
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
    Step(Step),
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
            crate::keywords::Keywords::Anchor => {
                let tmp = DatumAnchor::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::DatumAnchor(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::AngleUnit => {
                let tmp = AngleUnit::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::AngleUnit(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Area => {
                let tmp = AreaDescription::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::AreaDescription(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Axis => {
                let tmp = Axis::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::Axis(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::BaseEngCrs => {
                let tmp = BaseEngineeringCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::BaseEngineeringCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::BaseGeodCrs => {
                let tmp = BaseGeodeticCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::BaseGeodeticCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::BaseGeogCrs => {
                let tmp = BaseGeodeticCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::BaseGeodeticCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::BaseParamCrs => {
                let tmp = BaseParametricCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::BaseParametricCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::BaseProjCrs => {
                let tmp = BaseProjectedCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::BaseProjectedCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::BaseTimeCrs => {
                let tmp = BaseTemporalCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::BaseTemporalCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::BaseVertCrs => {
                let tmp = BaseVerticalCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::BaseVerticalCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::BBox => {
                let tmp = GeographicBoundingBox::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::GeographicBoundingBox(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Bearing => {
                let tmp = Bearing::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::Bearing(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::BoundCrs => {
                let tmp = BoundCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::BoundCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
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
            crate::keywords::Keywords::CompoundCrs => {
                let tmp = CompoundCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::CompoundCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::ConcatenatedOperation => {
                let tmp = ConcatenatedOperation::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::ConcatenatedOperation(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Conversion => {
                let tmp = MapProjection::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::MapProjection(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::CoordEpoch => {
                let tmp = CoordinateEpoch::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::CoordinateEpoch(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::CoordinateMetadata => {
                let tmp = CoordinateMetadata::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::CoordinateMetadata(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::CoordinateOperation => {
                let tmp = CoordinateOperation::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::CoordinateOperation(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Cs => {
                let tmp = CoordinateSystem::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::CoordinateSystem(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Datum => {
                let tmp = GeodeticReferenceFrame::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::GeodeticReferenceFrame(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::DerivedProjCrs => {
                let tmp = DerivedProjectedCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::DerivedProjectedCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::DerivingConversion => {
                let tmp = DerivingConversion::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::DerivingConversion(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Dynamic => {
                let tmp = DynamicCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::DynamicCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::EDatum => {
                let tmp = EngineeringDatum::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::EngineeringDatum(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Ellipsoid => {
                let tmp = Ellipsoid::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::Ellipsoid(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::EngCrs => {
                let tmp = EngineeringCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::EngineeringCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::EngineeringCrs => {
                let tmp = EngineeringCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::EngineeringCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::EngineeringDatum => {
                let tmp = EngineeringDatum::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::EngineeringDatum(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Ensemble => {
                if let Ok(tmp) = GeodeticDatumEnsemble::from_nodes(iter.clone()) {
                    return Ok(WktBaseTypeResult {
                        result: Self::GeodeticDatumEnsemble(tmp.result),
                        consumed: tmp.consumed,
                    });
                }

                if let Ok(tmp) = VerticalDatumEnsemble::from_nodes(iter) {
                    return Ok(WktBaseTypeResult {
                        result: Self::VerticalDatumEnsemble(tmp.result),
                        consumed: tmp.consumed,
                    });
                }

                return Err(WktParseError::CouldNotDetermineType);
            }
            crate::keywords::Keywords::EnsembleAccuracy => {
                let tmp = DatumEnsembleAccuracy::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::DatumEnsembleAccuracy(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Epoch => {
                let tmp = CoordinateEpoch::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::CoordinateEpoch(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::FrameEpoch => {
                let tmp = FrameEpoch::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::FrameEpoch(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::GeodCrs => {
                let tmp = GeodeticCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::GeodeticCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::GeodeticCrs => {
                let tmp = DerivedGeodeticCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::DerivedGeodeticCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::GeodeticDatum => {
                let tmp = GeodeticReferenceFrame::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::GeodeticReferenceFrame(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::GeogCrs => {
                let tmp = DerivedGeodeticCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::DerivedGeodeticCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::GeographicCrs => {
                let tmp = DerivedGeodeticCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::DerivedGeodeticCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::GeoidModel => {
                let tmp = GeoidModelId::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::GeoidModel(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Id => {
                let tmp = Id::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::Identifier(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::InterpolationCrs => {
                let tmp = InterpolationCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::InterpolationCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::LengthUnit => {
                let tmp = LengthUnit::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::LengthUnit(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Member => {
                let tmp = DatumEnsembleMember::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::DatumEnsembleMember(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Meridian => {
                let tmp = Meridian::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::Meridian(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Method => {
                if let Ok(tmp) = DerivedCrsConversionMethod::from_nodes(iter.clone()) {
                    return Ok(WktBaseTypeResult {
                        result: Self::DerivedCrsConversionMethod(tmp.result),
                        consumed: tmp.consumed,
                    });
                }

                if let Ok(tmp) = MapProjectionMethod::from_nodes(iter.clone()) {
                    return Ok(WktBaseTypeResult {
                        result: Self::MapProjectionMethod(tmp.result),
                        consumed: tmp.consumed,
                    });
                }

                if let Ok(tmp) = OperationMethod::from_nodes(iter) {
                    return Ok(WktBaseTypeResult {
                        result: Self::OperationMethod(tmp.result),
                        consumed: tmp.consumed,
                    });
                }

                return Err(WktParseError::CouldNotDetermineType);
            }
            crate::keywords::Keywords::Model => {
                let tmp = DeformationModelId::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::DeformationModelId(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::OperationAccuracy => {
                let tmp = OperationAccuracy::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::OperationAccuracy(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Order => {
                let tmp = Order::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::Order(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Parameter => {
                if let Ok(tmp) = MapProjectionParameter::from_nodes(iter.clone()) {
                    return Ok(WktBaseTypeResult {
                        result: Self::MapProjectionParameter(tmp.result),
                        consumed: tmp.consumed,
                    });
                }

                if let Ok(tmp) = OperationParameter::from_nodes(iter.clone()) {
                    return Ok(WktBaseTypeResult {
                        result: Self::OperationParameter(tmp.result),
                        consumed: tmp.consumed,
                    });
                }

                return Err(WktParseError::CouldNotDetermineType);
            }
            crate::keywords::Keywords::ParameterFile => {
                let tmp = OperationParameterFile::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::OperationParameterFile(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::ParametricCrs => {
                let tmp = ParametricCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::ParametricCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::ParametricDatum => {
                let tmp = ParametricDatum::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::ParametricDatum(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::ParametricUnit => {
                let tmp = ParametricUnit::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::ParametricUnit(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::PDatum => {
                let tmp = ParametricDatum::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::ParametricDatum(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::PointMotionOperation => {
                let tmp = PointMotionOperation::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::PointMotionOperation(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::PrimeM => {
                let tmp = PrimeMeridian::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::PrimeMeridian(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::PrimeMeridian => {
                let tmp = PrimeMeridian::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::PrimeMeridian(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::ProjCrs => {
                let tmp = ProjectedCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::ProjectedCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::ProjectedCrs => {
                let tmp = ProjectedCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::ProjectedCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Projection => {
                let tmp = MapProjectionMethod::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::MapProjectionMethod(tmp.result),
                    consumed: tmp.consumed,
                })
            }
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
            crate::keywords::Keywords::Scope => {
                let tmp = Scope::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::Scope(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::SourceCrs => {
                let tmp = SourceCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::SourceCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Spheroid => {
                let tmp = Ellipsoid::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::Ellipsoid(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Step => {
                let tmp = Step::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::Step(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::TargetCrs => {
                let tmp = TargetCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::TargetCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::TDatum => {
                let tmp = TemporalDatum::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::TemporalDatum(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::TRF => {
                let tmp = GeodeticReferenceFrame::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::GeodeticReferenceFrame(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::TemporalQuantity => {
                let tmp = TimeUnit::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::TimeUnit(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::TimeCrs => {
                let tmp = TimeCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::TimeCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::TimeDatum => {
                let tmp = TemporalDatum::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::TemporalDatum(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::TimeExtent => {
                let tmp = TemporalExtent::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::TemporalExtent(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::TimeOrigin => {
                let tmp = TimeOrigin::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::TimeOrigin(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::TimeUnit => {
                let tmp = TimeUnit::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::TimeUnit(tmp.result),
                    consumed: tmp.consumed,
                })
            }
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
            crate::keywords::Keywords::Usage => {
                let tmp = Usage::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::Usage(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::VDatum => {
                let tmp = VerticalReferenceFrame::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::VerticalReferenceFrame(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::VelocityGrid => {
                let tmp = DeformationModelId::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::DeformationModelId(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::Version => {
                let tmp = OperationVersion::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::OperationVersion(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::VertCrs => {
                let tmp = VerticalCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::VerticalCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::VerticalCrs => {
                let tmp = VerticalCrs::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::VerticalCrs(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::VerticalDatum => {
                let tmp = VerticalReferenceFrame::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::VerticalReferenceFrame(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::VerticalExtent => {
                let tmp = VerticalExtent::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::VerticalExtent(tmp.result),
                    consumed: tmp.consumed,
                })
            }
            crate::keywords::Keywords::VRF => {
                let tmp = VerticalReferenceFrame::from_nodes(iter)?;
                Ok(WktBaseTypeResult {
                    result: Self::VerticalReferenceFrame(tmp.result),
                    consumed: tmp.consumed,
                })
            }
        };
    }
}
