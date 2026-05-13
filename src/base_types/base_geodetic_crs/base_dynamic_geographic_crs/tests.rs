use crate::{
	WktBaseType,
	ast::parse_wkt,
	base_types::{
		BaseDynamicGeographicCrs, DynamicCrs, Ellipsoid, FrameEpoch,
		GeodeticReferenceFrame, Id, LengthUnit,
	},
};

const EXAMPLE1: &str = r#"
BASEGEOGCRS[
	"WGS 72", 
	DYNAMIC[
		FRAMEEPOCH[1972.0]
	],
	DATUM[
		"World Geodetic System 1972",
		ELLIPSOID[
			"WGS 72",
			6378135,
			298.26,
			LENGTHUNIT[
				"metre",
				1,
				ID["EPSG",9001]
			],
			ID["EPSG",7043]
		],
		ID["EPSG",6322]
	],
	ID["EPSG",4322]
]
"#;

#[test]
fn test_base_dynamic_geographic_crs_example_1() {
	let correct = BaseDynamicGeographicCrs {
		base_crs_name: "WGS 72".to_string(),
		dynamic_crs: DynamicCrs {
			frame_reference_epoch: FrameEpoch(1972.0),
			deformation_model_id: None,
		},
		geodetic_data: GeodeticReferenceFrame {
			datum_name: "World Geodetic System 1972".to_string(),
			ellipsoid: Ellipsoid {
				ellipsoid_name: "WGS 72".to_string(),
				semi_major_axis: 6378135.0,
				inverse_flattening: 298.26,
				length_unit: Some(LengthUnit::metre()),
				identifier: Some(Id::new_epsg(7043)),
			},
			anchor: None,
			anchor_epoch: None,
			identifier: Some(Id::new_epsg(6322)),
			prime_meridian: None,
		},
		ellipsoidal_cs_unit: None,
		identifier: Some(Id::new_epsg(4322)),
	};

	let ast = parse_wkt(EXAMPLE1).unwrap();

	assert_eq!(ast.len(), 1);

	let order = BaseDynamicGeographicCrs::from_nodes(&ast).unwrap();

	assert_eq!(correct, order.result);
}
