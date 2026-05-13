use wkt_crs_rs::{
	WktCrsTypes,
	base_types::{
		AngleUnit, Axis, BaseGeodeticCrs, BaseStaticGeographicCrs,
		BaseStaticVerticalCrs, BaseVerticalCrs, CompoundCrs, CoordinateSystem,
		DerivedVerticalCrs, DerivingConversion, Ellipsoid,
		GeodeticReferenceFrame, Id, LengthUnit, MapProjection, Method,
		Parameter, ProjectedCrs, SpatialCoordinateSystem,
		VerticalReferenceFrame,
	},
	compound_types::{
		GeodeticData, ScopeExtentIdentifierRemark, SingleCrs, SpatialUnit,
		Unit, VerticalFrameDatum,
	},
	enumerations::{AxisDirection, Dimension, SpatialCsType},
	parse_wkt_crs,
};

const EXAMPLE: &str = r#"
COMPOUNDCRS[
	"NAD83 / North Dakota South (ft) + NAVD88 height (ft)",
	PROJCRS[
		"NAD83 / North Dakota South (ft)",
		BASEGEOGCRS[
			"NAD83",
			DATUM[
				"North American Datum 1983",
				ELLIPSOID[
					"GRS 1980",
					6378137,
					298.257222101,
					LENGTHUNIT["metre",1,ID["EPSG",9001]],
					ID["EPSG",7019]
				],
				ID["EPSG",6269]
			],
			ID["EPSG",4269]
		],
		CONVERSION[
			"SPCS83 North Dakota South zone (international foot)",
			METHOD[
				"Lambert Conic Conformal (2SP)",
				ID["EPSG",9802]
			],
			PARAMETER[
				"Latitude of false origin",
				45.6666666666669,
				ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],
				ID["EPSG",8821]
			],
			PARAMETER[
				"Longitude of false origin",
				-100.5,
				ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],
				ID["EPSG",8822]
			],
			PARAMETER[
				"Latitude of 1st standard parallel",
				47.4833333333336,
				ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],
				ID["EPSG",8823]
			],
			PARAMETER[
				"Latitude of 2nd standard parallel",
				46.1833333333336,
				ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],
				ID["EPSG",8824]
			],
			PARAMETER[
				"Easting at false origin",
				1968503.937,
				LENGTHUNIT["foot",0.3048,ID["EPSG",9002]],
				ID["EPSG",8826]
			],
			PARAMETER[
				"Northing at false origin",
				0,
				LENGTHUNIT["foot",0.3048,ID["EPSG",9002]],
				ID["EPSG",8827]
			],
			ID["EPSG",15348]
		],
		CS[
			Cartesian,
			2,
			ID["EPSG",4495]
		],
		AXIS[
			"Easting (X)",
			east
		],
		AXIS[
			"Northing (Y)",
			north
		],
		LENGTHUNIT["foot",0.3048,ID["EPSG",9002]],
		ID["EPSG",2266]
	],
	VERTCRS[
		"NAVD88 height (ft)",
		BASEVERTCRS[
			"NAVD88 height",
			VDATUM[
				"North American Vertical Datum 1988",
				ID["EPSG",5103]
			],
			ID["EPSG",5703]
		],
		DERIVINGCONVERSION[
			"Vertical Axis Unit Conversion",
			METHOD[
				"Change of Vertical Unit",
				ID["EPSG",1104]
			],
			ID["EPSG",7813]
		],
		CS[
			vertical,
			1,
			ID["EPSG",1030]
		],
		AXIS[
			"Gravity-related height (H)",
			up
		],
		LENGTHUNIT[
			"foot",
			0.3048,
			ID["EPSG",9002]
		],
		ID["EPSG",8228]
	],
	ID["EPSG",8708]
]
"#;

#[test]
fn test_epsg_crs_8708() {
	let proj = ProjectedCrs {
		crs_name: "NAD83 / North Dakota South (ft)".to_string(),
		base_geodetic_crs: BaseGeodeticCrs::BaseStaticGeographicCrs(
			BaseStaticGeographicCrs {
				base_crs_name: "NAD83".to_string(),
				geodetic_data: GeodeticData::GeodeticReferenceFrame(
					GeodeticReferenceFrame {
						datum_name: "North American Datum 1983".to_string(),
						ellipsoid: Ellipsoid {
							ellipsoid_name: "GRS 1980".to_string(),
							semi_major_axis: 6378137.0,
							inverse_flattening: 298.257222101,
							length_unit: Some(LengthUnit::metre()),
							identifier: Some(Id::new_epsg(7019)),
						},
						anchor: None,
						anchor_epoch: None,
						identifier: Some(Id::new_epsg(6269)),
						prime_meridian: None,
					},
				),
				ellipsoidal_cs_unit: None,
				identifier: Some(Id::new_epsg(4269)),
			},
		),
		map_projection: MapProjection {
			map_projection_name:
				"SPCS83 North Dakota South zone (international foot)"
					.to_string(),
			map_projection_method: Method {
				method_name: "Lambert Conic Conformal (2SP)".to_string(),
				identifier: Some(Id::new_epsg(9802)),
			},
			map_projection_parameters: Some(vec![
				Parameter {
					parameter_name: "Latitude of false origin".to_string(),
					parameter_value: 45.6666666666669,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::AngleUnit(AngleUnit::degree()),
					)),
					identifier: Some(Id::new_epsg(8821)),
				},
				Parameter {
					parameter_name: "Longitude of false origin".to_string(),
					parameter_value: -100.5,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::AngleUnit(AngleUnit::degree()),
					)),
					identifier: Some(Id::new_epsg(8822)),
				},
				Parameter {
					parameter_name: "Latitude of 1st standard parallel"
						.to_string(),
					parameter_value: 47.4833333333336,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::AngleUnit(AngleUnit::degree()),
					)),
					identifier: Some(Id::new_epsg(8823)),
				},
				Parameter {
					parameter_name: "Latitude of 2nd standard parallel"
						.to_string(),
					parameter_value: 46.1833333333336,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::AngleUnit(AngleUnit::degree()),
					)),
					identifier: Some(Id::new_epsg(8824)),
				},
				Parameter {
					parameter_name: "Easting at false origin".to_string(),
					parameter_value: 1968503.937,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::LengthUnit(LengthUnit {
							unit_name: "foot".to_string(),
							conversion_factor: 0.3048,
							identifier: Some(Id::new_epsg(9002)),
						}),
					)),
					identifier: Some(Id::new_epsg(8826)),
				},
				Parameter {
					parameter_name: "Northing at false origin".to_string(),
					parameter_value: 0.0,
					parameter_unit: Some(Unit::SpatialUnit(
						SpatialUnit::LengthUnit(LengthUnit {
							unit_name: "foot".to_string(),
							conversion_factor: 0.3048,
							identifier: Some(Id::new_epsg(9002)),
						}),
					)),
					identifier: Some(Id::new_epsg(8827)),
				},
			]),
			identifier: Some(Id::new_epsg(15348)),
		},
		coordinate_system: CoordinateSystem::SpatialCS(
			SpatialCoordinateSystem {
				spatial_cs_type: SpatialCsType::Cartesian,
				dimension: Dimension::Two,
				identifier: Some(Id::new_epsg(4495)),
				spatial_axis: vec![
					Axis {
						axis_name_abbreviation: "Easting (X)".to_string(),
						axis_direction: AxisDirection::East,
						axis_order: None,
						unit: None,
						identifier: None,
					},
					Axis {
						axis_name_abbreviation: "Northing (Y)".to_string(),
						axis_direction: AxisDirection::North(None),
						axis_order: None,
						unit: None,
						identifier: None,
					},
				],
				cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(
					LengthUnit {
						unit_name: "foot".to_string(),
						conversion_factor: 0.3048,
						identifier: Some(Id::new_epsg(9002)),
					},
				))),
			},
		),
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: None,
			identifier: Some(vec![Id::new_epsg(2266)]),
			remark: None,
		},
	};

	let vert = DerivedVerticalCrs {
		derived_crs_name: "NAVD88 height (ft)".to_string(),
		base_vertical_crs: BaseVerticalCrs::BaseStaticVerticalCrs(
			BaseStaticVerticalCrs {
				base_crs_name: "NAVD88 height".to_string(),
				vertical_frame_datum:
					VerticalFrameDatum::VerticalReferenceFrame(
						VerticalReferenceFrame {
							datum_name: "North American Vertical Datum 1988"
								.to_string(),
							datum_anchor: None,
							identifier: Some(Id::new_epsg(5103)),
						},
					),
				identifier: Some(Id::new_epsg(5703)),
			},
		),
		deriving_conversion: DerivingConversion {
			deriving_conversion_name: "Vertical Axis Unit Conversion"
				.to_string(),
			operation_method: Method {
				method_name: "Change of Vertical Unit".to_string(),
				identifier: Some(Id::new_epsg(1104)),
			},
			operation_parameter: None,
			identifier: Some(Id::new_epsg(7813)),
		},
		coordinate_system: CoordinateSystem::SpatialCS(
			SpatialCoordinateSystem {
				spatial_cs_type: SpatialCsType::Vertical,
				dimension: Dimension::One,
				identifier: Some(Id::new_epsg(1030)),
				spatial_axis: vec![Axis {
					axis_name_abbreviation: "Gravity-related height (H)"
						.to_string(),
					axis_direction: AxisDirection::Up,
					axis_order: None,
					unit: None,
					identifier: None,
				}],
				cs_unit: Some(Unit::SpatialUnit(SpatialUnit::LengthUnit(
					LengthUnit {
						unit_name: "foot".to_string(),
						conversion_factor: 0.3048,
						identifier: Some(Id::new_epsg(9002)),
					},
				))),
			},
		),
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: None,
			identifier: Some(vec![Id::new_epsg(8228)]),
			remark: None,
		},
	};

	let comp = CompoundCrs {
		compound_crs_name:
			"NAD83 / North Dakota South (ft) + NAVD88 height (ft)".to_string(),
		crs_one: SingleCrs::ProjectedCrs(proj),
		crs_two: SingleCrs::DerivedVerticalCrs(vert),
		additional_crs: None,
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: None,
			identifier: Some(vec![Id::new_epsg(8708)]),
			remark: None,
		},
	};

	let correct = vec![WktCrsTypes::CompoundCrs(comp)];

	let ast = parse_wkt_crs(EXAMPLE).unwrap();

	assert_eq!(correct, ast);
}
