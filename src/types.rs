use crate::{
	ast::{WktArg, WktNode},
	base_types::{
		AbridgedCoordinateTransformation, AngleUnit, AreaDescription, Axis,
		BaseEngineeringCrs, BaseGeodeticCrs, BaseParametricCrs,
		BaseProjectedCrs, BaseTemporalCrs, BaseVerticalCrs, Bearing, BoundCrs,
		Calendar, Citation, CompoundCrs, ConcatenatedOperation,
		CoordinateEpoch, CoordinateMetadata, CoordinateOperation,
		CoordinateSystem, DatumAnchor, DatumEnsembleAccuracy,
		DatumEnsembleMember, DeformationModelId, DerivedCrsConversionMethod,
		DerivedEngineeringCrs, DerivedGeodeticCrs, DerivedParametricCrs,
		DerivedProjectedCrs, DerivedTemporalCrs, DerivedVerticalCrs,
		DerivingConversion, DynamicCrs, Ellipsoid, EngineeringCrs,
		EngineeringDatum, Extent, FrameEpoch, GeodeticCrs,
		GeodeticDatumEnsemble, GeodeticReferenceFrame, GeographicBoundingBox,
		GeographicCrs, GeoidModelId, Id, InterpolationCrs, LengthUnit,
		MapProjection, MapProjectionMethod, MapProjectionParameter, Meridian,
		OperationAccuracy, OperationMethod, OperationParameter,
		OperationParameterFile, OperationVersion, Order, ParametricCrs,
		ParametricDatum, ParametricUnit, PointMotionOperation, PrimeMeridian,
		ProjectedCrs, Remark, ScaleUnit, Scope, SourceCrs, TargetCrs,
		TemporalDatum, TemporalExtent, TimeCrs, TimeOrigin, TimeUnit, Uri,
		Usage, VerticalCrs, VerticalDatumEnsemble, VerticalExtent,
		VerticalReferenceFrame,
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
	fn from_nodes<'a, I>(
		wkt_nodes: I,
	) -> Result<WktBaseTypeResult<Self>, WktParseError>
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
	fn from_args<'a, I>(
		wkt_args: I,
	) -> Result<WktInlineResult<Self>, WktParseError>
	where
		I: IntoIterator<Item = &'a WktArg>;
}

#[derive(Debug, PartialEq)]
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

// It is essential this is never inlined - the match is so huge that it can easily
// overflow the stack!
#[inline(never)]
fn process<T, F>(
	iter: Vec<&WktNode>,
	wrap: F,
) -> Result<WktBaseTypeResult<WktCrsTypes>, WktParseError>
where
	T: WktBaseType,
	F: FnOnce(T) -> WktCrsTypes,
{
	let tmp = T::from_nodes(iter)?;
	Ok(WktBaseTypeResult {
		result: wrap(tmp.result),
		consumed: tmp.consumed,
	})
}

impl WktBaseType for WktCrsTypes {
	fn from_nodes<'a, I>(
		wkt_nodes: I,
	) -> Result<WktBaseTypeResult<Self>, WktParseError>
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
				process::<AbridgedCoordinateTransformation, _>(
					iter,
					Self::AbridgedCoordinateTransformation,
				)
			}
			crate::keywords::Keywords::Anchor => {
				process::<DatumAnchor, _>(iter, Self::DatumAnchor)
			}
			crate::keywords::Keywords::AngleUnit => {
				process::<AngleUnit, _>(iter, Self::AngleUnit)
			}
			crate::keywords::Keywords::Area => {
				process::<AreaDescription, _>(iter, Self::AreaDescription)
			}
			crate::keywords::Keywords::Axis => {
				process::<Axis, _>(iter, Self::Axis)
			}
			crate::keywords::Keywords::BaseEngCrs => {
				process::<BaseEngineeringCrs, _>(iter, Self::BaseEngineeringCrs)
			}
			crate::keywords::Keywords::BaseGeodCrs => {
				process::<BaseGeodeticCrs, _>(iter, Self::BaseGeodeticCrs)
			}
			crate::keywords::Keywords::BaseGeogCrs => {
				process::<BaseGeodeticCrs, _>(iter, Self::BaseGeodeticCrs)
			}
			crate::keywords::Keywords::BaseParamCrs => {
				process::<BaseParametricCrs, _>(iter, Self::BaseParametricCrs)
			}
			crate::keywords::Keywords::BaseProjCrs => {
				process::<BaseProjectedCrs, _>(iter, Self::BaseProjectedCrs)
			}
			crate::keywords::Keywords::BaseTimeCrs => {
				process::<BaseTemporalCrs, _>(iter, Self::BaseTemporalCrs)
			}
			crate::keywords::Keywords::BaseVertCrs => {
				process::<BaseVerticalCrs, _>(iter, Self::BaseVerticalCrs)
			}
			crate::keywords::Keywords::BBox => {
				process::<GeographicBoundingBox, _>(
					iter,
					Self::GeographicBoundingBox,
				)
			}
			crate::keywords::Keywords::Bearing => {
				process::<Bearing, _>(iter, Self::Bearing)
			}
			crate::keywords::Keywords::BoundCrs => {
				process::<BoundCrs, _>(iter, Self::BoundCrs)
			}
			crate::keywords::Keywords::Calendar => {
				process::<Calendar, _>(iter, Self::Calendar)
			}
			crate::keywords::Keywords::Citation => {
				process::<Citation, _>(iter, Self::Citation)
			}
			crate::keywords::Keywords::CompoundCrs => {
				process::<CompoundCrs, _>(iter, Self::CompoundCrs)
			}
			crate::keywords::Keywords::ConcatenatedOperation => {
				process::<ConcatenatedOperation, _>(
					iter,
					Self::ConcatenatedOperation,
				)
			}
			crate::keywords::Keywords::Conversion => {
				process::<MapProjection, _>(iter, Self::MapProjection)
			}
			crate::keywords::Keywords::CoordEpoch => {
				process::<CoordinateEpoch, _>(iter, Self::CoordinateEpoch)
			}
			crate::keywords::Keywords::CoordinateMetadata => {
				process::<CoordinateMetadata, _>(iter, Self::CoordinateMetadata)
			}
			crate::keywords::Keywords::CoordinateOperation => {
				process::<CoordinateOperation, _>(
					iter,
					Self::CoordinateOperation,
				)
			}
			crate::keywords::Keywords::Cs => {
				process::<CoordinateSystem, _>(iter, Self::CoordinateSystem)
			}
			crate::keywords::Keywords::Datum => {
				process::<GeodeticReferenceFrame, _>(
					iter,
					Self::GeodeticReferenceFrame,
				)
			}
			crate::keywords::Keywords::DerivedProjCrs => {
				process::<DerivedProjectedCrs, _>(
					iter,
					Self::DerivedProjectedCrs,
				)
			}
			crate::keywords::Keywords::DerivingConversion => {
				process::<DerivingConversion, _>(iter, Self::DerivingConversion)
			}
			crate::keywords::Keywords::Dynamic => {
				process::<DynamicCrs, _>(iter, Self::DynamicCrs)
			}
			crate::keywords::Keywords::EDatum => {
				process::<EngineeringDatum, _>(iter, Self::EngineeringDatum)
			}
			crate::keywords::Keywords::Ellipsoid => {
				process::<Ellipsoid, _>(iter, Self::Ellipsoid)
			}
			crate::keywords::Keywords::EngCrs => {
				process::<EngineeringCrs, _>(iter, Self::EngineeringCrs)
			}
			crate::keywords::Keywords::EngineeringCrs => {
				process::<EngineeringCrs, _>(iter, Self::EngineeringCrs)
			}
			crate::keywords::Keywords::EngineeringDatum => {
				process::<EngineeringDatum, _>(iter, Self::EngineeringDatum)
			}
			crate::keywords::Keywords::Ensemble => {
				if let Ok(tmp) = GeodeticDatumEnsemble::from_nodes(iter.clone())
				{
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
				process::<DatumEnsembleAccuracy, _>(
					iter,
					Self::DatumEnsembleAccuracy,
				)
			}
			crate::keywords::Keywords::Epoch => {
				process::<CoordinateEpoch, _>(iter, Self::CoordinateEpoch)
			}
			crate::keywords::Keywords::FrameEpoch => {
				process::<FrameEpoch, _>(iter, Self::FrameEpoch)
			}
			crate::keywords::Keywords::GeodCrs => {
				process::<GeodeticCrs, _>(iter, Self::GeodeticCrs)
			}
			crate::keywords::Keywords::GeodeticCrs => {
				process::<GeodeticCrs, _>(iter, Self::GeodeticCrs)
			}
			crate::keywords::Keywords::GeodeticDatum => {
				process::<GeodeticReferenceFrame, _>(
					iter,
					Self::GeodeticReferenceFrame,
				)
			}
			crate::keywords::Keywords::GeogCrs => {
				process::<DerivedGeodeticCrs, _>(iter, Self::DerivedGeodeticCrs)
			}
			crate::keywords::Keywords::GeographicCrs => {
				process::<DerivedGeodeticCrs, _>(iter, Self::DerivedGeodeticCrs)
			}
			crate::keywords::Keywords::GeoidModel => {
				process::<GeoidModelId, _>(iter, Self::GeoidModel)
			}
			crate::keywords::Keywords::Id => {
				process::<Id, _>(iter, Self::Identifier)
			}
			crate::keywords::Keywords::InterpolationCrs => {
				process::<InterpolationCrs, _>(iter, Self::InterpolationCrs)
			}
			crate::keywords::Keywords::LengthUnit => {
				process::<LengthUnit, _>(iter, Self::LengthUnit)
			}
			crate::keywords::Keywords::Member => {
				process::<DatumEnsembleMember, _>(
					iter,
					Self::DatumEnsembleMember,
				)
			}
			crate::keywords::Keywords::Meridian => {
				process::<Meridian, _>(iter, Self::Meridian)
			}
			crate::keywords::Keywords::Method => {
				if let Ok(tmp) =
					DerivedCrsConversionMethod::from_nodes(iter.clone())
				{
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
				process::<DeformationModelId, _>(iter, Self::DeformationModelId)
			}
			crate::keywords::Keywords::OperationAccuracy => {
				process::<OperationAccuracy, _>(iter, Self::OperationAccuracy)
			}
			crate::keywords::Keywords::Order => {
				process::<Order, _>(iter, Self::Order)
			}
			crate::keywords::Keywords::Parameter => {
				if let Ok(tmp) =
					MapProjectionParameter::from_nodes(iter.clone())
				{
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
				process::<OperationParameterFile, _>(
					iter,
					Self::OperationParameterFile,
				)
			}
			crate::keywords::Keywords::ParametricCrs => {
				process::<ParametricCrs, _>(iter, Self::ParametricCrs)
			}
			crate::keywords::Keywords::ParametricDatum => {
				process::<ParametricDatum, _>(iter, Self::ParametricDatum)
			}
			crate::keywords::Keywords::ParametricUnit => {
				process::<ParametricUnit, _>(iter, Self::ParametricUnit)
			}
			crate::keywords::Keywords::PDatum => {
				process::<ParametricDatum, _>(iter, Self::ParametricDatum)
			}
			crate::keywords::Keywords::PointMotionOperation => {
				process::<PointMotionOperation, _>(
					iter,
					Self::PointMotionOperation,
				)
			}
			crate::keywords::Keywords::PrimeM => {
				process::<PrimeMeridian, _>(iter, Self::PrimeMeridian)
			}
			crate::keywords::Keywords::PrimeMeridian => {
				process::<PrimeMeridian, _>(iter, Self::PrimeMeridian)
			}
			crate::keywords::Keywords::ProjCrs => {
				process::<ProjectedCrs, _>(iter, Self::ProjectedCrs)
			}
			crate::keywords::Keywords::ProjectedCrs => {
				process::<ProjectedCrs, _>(iter, Self::ProjectedCrs)
			}
			crate::keywords::Keywords::Projection => {
				process::<MapProjectionMethod, _>(
					iter,
					Self::MapProjectionMethod,
				)
			}
			crate::keywords::Keywords::Remark => {
				process::<Remark, _>(iter, Self::Remark)
			}
			crate::keywords::Keywords::ScaleUnit => {
				process::<ScaleUnit, _>(iter, Self::ScaleUnit)
			}
			crate::keywords::Keywords::Scope => {
				process::<Scope, _>(iter, Self::Scope)
			}
			crate::keywords::Keywords::SourceCrs => {
				process::<SourceCrs, _>(iter, Self::SourceCrs)
			}
			crate::keywords::Keywords::Spheroid => {
				process::<Ellipsoid, _>(iter, Self::Ellipsoid)
			}
			crate::keywords::Keywords::Step => {
				process::<Step, _>(iter, Self::Step)
			}
			crate::keywords::Keywords::TargetCrs => {
				process::<TargetCrs, _>(iter, Self::TargetCrs)
			}
			crate::keywords::Keywords::TDatum => {
				process::<TemporalDatum, _>(iter, Self::TemporalDatum)
			}
			crate::keywords::Keywords::TRF => {
				process::<GeodeticReferenceFrame, _>(
					iter,
					Self::GeodeticReferenceFrame,
				)
			}
			crate::keywords::Keywords::TemporalQuantity => {
				process::<TimeUnit, _>(iter, Self::TimeUnit)
			}
			crate::keywords::Keywords::TimeCrs => {
				process::<TimeCrs, _>(iter, Self::TimeCrs)
			}
			crate::keywords::Keywords::TimeDatum => {
				process::<TemporalDatum, _>(iter, Self::TemporalDatum)
			}
			crate::keywords::Keywords::TimeExtent => {
				process::<TemporalExtent, _>(iter, Self::TemporalExtent)
			}
			crate::keywords::Keywords::TimeOrigin => {
				process::<TimeOrigin, _>(iter, Self::TimeOrigin)
			}
			crate::keywords::Keywords::TimeUnit => {
				process::<TimeUnit, _>(iter, Self::TimeUnit)
			}
			crate::keywords::Keywords::Triaxial => todo!(),
			crate::keywords::Keywords::Unit => {
				process::<Unit, _>(iter, Self::Unit)
			}
			crate::keywords::Keywords::Uri => {
				process::<Uri, _>(iter, Self::Uri)
			}
			crate::keywords::Keywords::Usage => {
				process::<Usage, _>(iter, Self::Usage)
			}
			crate::keywords::Keywords::VDatum => {
				process::<VerticalReferenceFrame, _>(
					iter,
					Self::VerticalReferenceFrame,
				)
			}
			crate::keywords::Keywords::VelocityGrid => {
				process::<DeformationModelId, _>(iter, Self::DeformationModelId)
			}
			crate::keywords::Keywords::Version => {
				process::<OperationVersion, _>(iter, Self::OperationVersion)
			}
			crate::keywords::Keywords::VertCrs => {
				process::<VerticalCrs, _>(iter, Self::VerticalCrs)
			}
			crate::keywords::Keywords::VerticalCrs => {
				process::<VerticalCrs, _>(iter, Self::VerticalCrs)
			}
			crate::keywords::Keywords::VerticalDatum => {
				process::<VerticalReferenceFrame, _>(
					iter,
					Self::VerticalReferenceFrame,
				)
			}
			crate::keywords::Keywords::VerticalExtent => {
				process::<VerticalExtent, _>(iter, Self::VerticalExtent)
			}
			crate::keywords::Keywords::VRF => {
				process::<VerticalReferenceFrame, _>(
					iter,
					Self::VerticalReferenceFrame,
				)
			}
		};
	}
}
