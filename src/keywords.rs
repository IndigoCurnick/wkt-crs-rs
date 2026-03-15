use std::fmt::Display;

use strum::{AsRefStr, EnumString};

use crate::error::WktParseError;

#[derive(Debug, PartialEq, EnumString, Clone, Copy, AsRefStr)]
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
		write!(f, "{}", self.as_ref())
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
