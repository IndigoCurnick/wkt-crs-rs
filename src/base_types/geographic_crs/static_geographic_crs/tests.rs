use crate::{
	WktBaseType,
	ast::parse_wkt,
	base_types::{
		AngleUnit, Axis, CoordinateSystem, DefiningTransformation, Ellipsoid,
		GeodeticReferenceFrame, Id, LengthUnit, SpatialCoordinateSystem,
		StaticGeographicCrs,
	},
	compound_types::{
		GeodeticData, ScopeExtentIdentifierRemark, SpatialUnit, Unit,
	},
	enumerations::{AxisDirection, Dimension, SpatialCsType},
};

const EXAMPLE: &str = r#"
GEOGCRS[
	"WC05-IRF",
	DATUM[
		"WC05 Intermediate Reference Frame",
		ELLIPSOID[
			"GRS 1980",
			6378137,
			298.257222101,
			LENGTHUNIT[
				"metre",
				1,
				ID["EPSG",9001]
			],
			ID["EPSG",7019]
		],
		ID["EPSG",1386]
	],
	CS[
		ellipsoidal,
		2,
		ID["EPSG",6422]
	],
	AXIS[
		"Geodetic latitude (Lat)",
		north
	],
	AXIS[
		"Geodetic longitude (Lon)",
		east
	],
	ANGLEUNIT[
		"degree",
		0.0174532925199433,
		ID["EPSG",9102]
	],
	DEFININGTRANSFORMATION[
		"ETRS89 to WC05-IRF (1)",
		ID["EPSG",10629]
	],
	ID["EPSG",10628]
]
"#;

#[test]
fn test_static_geographic_crs() {
	let correct = StaticGeographicCrs {
		crs_name: "WC05-IRF".to_string(),
		frame: GeodeticData::GeodeticReferenceFrame(GeodeticReferenceFrame {
			datum_name: "WC05 Intermediate Reference Frame".to_string(),
			ellipsoid: Ellipsoid {
				ellipsoid_name: "GRS 1980".to_string(),
				semi_major_axis: 6378137.0,
				inverse_flattening: 298.257222101,
				length_unit: Some(LengthUnit {
					unit_name: "metre".to_string(),
					conversion_factor: 1.0,
					identifier: Some(Id::new_epsg(9001)),
				}),
				identifier: Some(Id::new_epsg(7019)),
			},
			anchor: None,
			anchor_epoch: None,
			identifier: Some(Id::new_epsg(1386)),
			prime_meridian: None,
		}),
		coordinate_system: CoordinateSystem::SpatialCS(
			SpatialCoordinateSystem {
				spatial_cs_type: SpatialCsType::Ellipsoidal,
				dimension: Dimension::Two,
				identifier: Some(Id::new_epsg(6422)),
				spatial_axis: vec![
					Axis {
						axis_name_abbreviation: "Geodetic latitude (Lat)"
							.to_string(),
						axis_direction: AxisDirection::North(None),
						axis_order: None,
						unit: None,
						identifier: None,
					},
					Axis {
						axis_name_abbreviation: "Geodetic longitude (Lon)"
							.to_string(),
						axis_direction: AxisDirection::East,
						axis_order: None,
						unit: None,
						identifier: None,
					},
				],
				cs_unit: Some(Unit::SpatialUnit(SpatialUnit::AngleUnit(
					AngleUnit {
						unit_name: "degree".to_string(),
						conversion_factor: 0.0174532925199433,
						identifier: Some(Id::new_epsg(9102)),
					},
				))),
			},
		),
		defining_transformation_id: Some(DefiningTransformation {
			defining_transformation_name: "ETRS89 to WC05-IRF (1)".to_string(),
			identifier: Some(Id::new_epsg(10629)),
		}),
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: None,
			identifier: Some(vec![Id::new_epsg(10628)]),
			remark: None,
		},
	};

	let ast = parse_wkt(EXAMPLE).unwrap();

	assert_eq!(ast.len(), 1);

	let datum = StaticGeographicCrs::from_nodes(&ast).unwrap();

	assert_eq!(correct, datum.result);
	assert_eq!(datum.consumed, 1);
}
