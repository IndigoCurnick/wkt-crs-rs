use wkt_crs_rs::{
	WktCrsTypes,
	base_types::{
		Axis, CoordinateSystem, Ellipsoid, GeodeticCrs, GeodeticReferenceFrame,
		LengthUnit, SpatialCoordinateSystem, StaticGeodeticCrs, TargetCrs,
	},
	compound_types::{
		CoordinateReferenceSystem, GeodeticData, ScopeExtentIdentifierRemark,
		SingleCrs, SpatialUnit, Unit,
	},
	enumerations::{AxisDirection, Dimension, SpatialCsType},
	parse_wkt_crs,
};

const EXAMPLE: &str = r#"TARGETCRS[
        GEODCRS["JGD2000",
            DATUM["Japanese Geodetic Datum 2000",
                ELLIPSOID["GRS 1980",6378137.0,298.257222101,LENGTHUNIT["metre",1.0]]],
            CS[Cartesian,3],
                AXIS["(X)",geocentricX],
                AXIS["(Y)",geocentricY],
                AXIS["(Z)",geocentricZ],
                LENGTHUNIT["metre",1.0]
        ]
    ]"#;

#[test]
fn test_target_crs() {
	let correct = TargetCrs {
		coordinate_system: CoordinateReferenceSystem::SingleCrs(
			SingleCrs::GeodeticCrs(GeodeticCrs::StaticGeodeticCrs(
				StaticGeodeticCrs {
					crs_name: "JGD2000".into(),
					frame: GeodeticData::GeodeticReferenceFrame(
						GeodeticReferenceFrame {
							datum_name: "Japanese Geodetic Datum 2000".into(),
							ellipsoid: Ellipsoid {
								ellipsoid_name: "GRS 1980".into(),
								semi_major_axis: 6378137.0,
								inverse_flattening: 298.257222101,
								length_unit: Some(LengthUnit {
									unit_name: "metre".into(),
									conversion_factor: 1.0,
									identifier: None,
								}),
								identifier: None,
							},
							anchor: None,
							anchor_epoch: None,
							identifier: None,
							prime_meridian: None,
						},
					),
					coordinate_system: CoordinateSystem::SpatialCS(
						SpatialCoordinateSystem {
							spatial_cs_type: SpatialCsType::Cartesian,
							dimension: Dimension::Three,
							identifier: None,
							spatial_axis: vec![
								Axis {
									axis_name_abbreviation: "(X)".into(),
									axis_direction: AxisDirection::GeocentricX,
									axis_order: None,
									unit: None,
									identifier: None,
								},
								Axis {
									axis_name_abbreviation: "(Y)".into(),
									axis_direction: AxisDirection::GeocentricY,
									axis_order: None,
									unit: None,
									identifier: None,
								},
								Axis {
									axis_name_abbreviation: "(Z)".into(),
									axis_direction: AxisDirection::GeocentricZ,
									axis_order: None,
									unit: None,
									identifier: None,
								},
							],
							cs_unit: Some(Unit::SpatialUnit(
								SpatialUnit::LengthUnit(LengthUnit {
									unit_name: "metre".into(),
									conversion_factor: 1.0,
									identifier: None,
								}),
							)),
						},
					),
					defining_transformation_id: None,
					scope_extent_identifier_remark:
						ScopeExtentIdentifierRemark {
							usage: None,
							identifier: None,
							remark: None,
						},
				},
			)),
		),
	};

	let correct = vec![WktCrsTypes::TargetCrs(correct)];

	let ast = parse_wkt_crs(EXAMPLE).unwrap();

	assert_eq!(ast, correct);
}
