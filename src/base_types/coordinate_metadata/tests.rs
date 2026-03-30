use crate::{
	ast::parse_wkt,
	base_types::{
		AngleUnit, Axis, CoordinateEpoch, CoordinateSystem, DynamicCrs,
		DynamicGeographicCrs, Ellipsoid, FrameEpoch, GeodeticReferenceFrame,
		LengthUnit, SpatialCoordinateSystem,
		coordinate_metadata::coordinate_metadata::{
			CoordinateMetadata, DynamicCoordinateMetadata,
		},
	},
	compound_types::{
		DynamicCrsCoordinateMetadata, ScopeExtentIdentifierRemark, SpatialUnit,
		Unit,
	},
	enumerations::{AxisDirection, Dimension, SpatialCsType},
	types::WktBaseType,
};

const EXAMPLE1: &str = r#"COORDINATEMETADATA[
    GEOGCRS["WGS 84 (G1762)",
        DYNAMIC[FRAMEEPOCH[2005.0]],
        DATUM["World Geodetic System 1984 (G1762)",
            ELLIPSOID["WGS 84",6378137,298.25,LENGTHUNIT["metre",1.0]]
        ],
        CS[ellipsoidal,3],
            AXIS["(lat)",north,ANGLEUNIT["degree",0.017]],
            AXIS["(lon)",east,ANGLEUNIT["degree",0.017]],
            AXIS["ellipsoidal height (h)",up,LENGTHUNIT["metre",1.0]]
    ],
    EPOCH[2016.47]
]
"#;

#[test]
fn test_coordinate_metadata() {
	let correct = CoordinateMetadata::DynamicCoordinateMetadata(
		DynamicCoordinateMetadata {
			dynamic_coordinate_metadata:
				DynamicCrsCoordinateMetadata::DynamicGeographicCrs(
					DynamicGeographicCrs {
						crs_name: "WGS 84 (G1762)".into(),
						dynamic_crs: DynamicCrs {
							frame_reference_epoch: FrameEpoch(2005.0),
							deformation_model_id: None,
						},
						geodetic_reference_frame: GeodeticReferenceFrame {
							datum_name: "World Geodetic System 1984 (G1762)"
								.into(),
							ellipsoid: Ellipsoid {
								ellipsoid_name: "WGS 84".into(),
								semi_major_axis: 6378137.0,
								inverse_flattening: 298.25,
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
						coordinate_system: CoordinateSystem::SpatialCS(
							SpatialCoordinateSystem {
								spatial_cs_type: SpatialCsType::Ellipsoidal,
								dimension: Dimension::Three,
								identifier: None,
								spatial_axis: vec![
									Axis {
										axis_name_abbreviation: "(lat)".into(),
										axis_direction: AxisDirection::North(
											None,
										),
										axis_order: None,
										unit: Some(Unit::SpatialUnit(
											SpatialUnit::AngleUnit(AngleUnit {
												unit_name: "degree".into(),
												conversion_factor: 0.017,
												identifier: None,
											}),
										)),
										identifier: None,
									},
									Axis {
										axis_name_abbreviation: "(lon)".into(),
										axis_direction: AxisDirection::East,
										axis_order: None,
										unit: Some(Unit::SpatialUnit(
											SpatialUnit::AngleUnit(AngleUnit {
												unit_name: "degree".into(),
												conversion_factor: 0.017,
												identifier: None,
											}),
										)),
										identifier: None,
									},
									Axis {
										axis_name_abbreviation:
											"ellipsoidal height (h)".into(),
										axis_direction: AxisDirection::Up,
										axis_order: None,
										unit: Some(Unit::SpatialUnit(
											SpatialUnit::LengthUnit(
												LengthUnit {
													unit_name: "metre".into(),
													conversion_factor: 1.0,
													identifier: None,
												},
											),
										)),
										identifier: None,
									},
								],
								cs_unit: None,
							},
						),
						defining_transformation_id: None,
						scope_extent_identifier_remark:
							ScopeExtentIdentifierRemark {
								identifier: None,
								remark: None,
								usage: None,
							},
					},
				),
			metadata_coordinate_epoch: CoordinateEpoch(2016.47),
		},
	);

	let ast = parse_wkt(EXAMPLE1).unwrap();

	assert_eq!(ast.len(), 1);

	let acc = CoordinateMetadata::from_nodes(&ast).unwrap();

	assert_eq!(correct, acc.result);
}
