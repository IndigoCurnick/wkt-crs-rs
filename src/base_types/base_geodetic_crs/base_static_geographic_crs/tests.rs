use crate::{
	ast::parse_wkt,
	base_types::{Ellipsoid, GeodeticReferenceFrame, Id, LengthUnit},
	compound_types::GeodeticData,
	data_types::NumText,
	types::WktBaseType,
};

use super::BaseStaticGeographicCrs;

const EXAMPLE1: &str = r#"
BASEGEOGCRS["ETRS89",
    DATUM["ETRS89",
        ELLIPSOID["GRS 80",6378137,298.25,LENGTHUNIT["metre",1.0]]
    ],
    ID["EuroGeographics","ETRS89-LatLon"]
]
"#;

const EXAMPLE2: &str = r#"
BASEGEOGCRS[
	"NAD83(HARN)",
	DATUM[
		"NAD83 (High Accuracy Reference Network)",
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
		ID["EPSG",6152]
	],
	ID["EPSG",4152]
]
"#;

#[test]
fn test_base_static_geographic_crs() {
	test_example_1();
	test_example_2();
}

fn test_example_1() {
	let correct = BaseStaticGeographicCrs {
		base_crs_name: "ETRS89".into(),
		geodetic_data: GeodeticData::GeodeticReferenceFrame(
			GeodeticReferenceFrame {
				datum_name: "ETRS89".into(),
				ellipsoid: Ellipsoid {
					ellipsoid_name: "GRS 80".into(),
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
		),
		ellipsoidal_cs_unit: None,
		identifier: Some(Id {
			authority_name: "EuroGeographics".into(),
			authority_unique_identifier: NumText::Text("ETRS89-LatLon".into()),
			version: None,
			authority_citation: None,
			id_uri: None,
		}),
	};

	let ast = parse_wkt(EXAMPLE1).unwrap();
	assert_eq!(ast.len(), 1);
	let bsgc = BaseStaticGeographicCrs::from_nodes(&ast).unwrap();

	assert_eq!(correct, bsgc.result);
}

fn test_example_2() {
	let correct = BaseStaticGeographicCrs {
		base_crs_name: "NAD83(HARN)".to_string(),
		geodetic_data: GeodeticData::GeodeticReferenceFrame(
			GeodeticReferenceFrame {
				datum_name: "NAD83 (High Accuracy Reference Network)"
					.to_string(),
				ellipsoid: Ellipsoid {
					ellipsoid_name: "GRS 1980".to_string(),
					semi_major_axis: 6378137.0,
					inverse_flattening: 298.257222101,
					length_unit: Some(LengthUnit {
						unit_name: "metre".to_string(),
						conversion_factor: 1.0,
						identifier: Some(Id {
							authority_name: "EPSG".to_string(),
							authority_unique_identifier: NumText::Int(9001),
							version: None,
							authority_citation: None,
							id_uri: None,
						}),
					}),
					identifier: Some(Id {
						authority_name: "EPSG".to_string(),
						authority_unique_identifier: NumText::Int(7019),
						version: None,
						authority_citation: None,
						id_uri: None,
					}),
				},
				anchor: None,
				anchor_epoch: None,
				identifier: Some(Id {
					authority_name: "EPSG".to_string(),
					authority_unique_identifier: NumText::Int(6152),
					version: None,
					authority_citation: None,
					id_uri: None,
				}),
				prime_meridian: None,
			},
		),
		ellipsoidal_cs_unit: None,
		identifier: Some(Id {
			authority_name: "EPSG".to_string(),
			authority_unique_identifier: NumText::Int(4152),
			version: None,
			authority_citation: None,
			id_uri: None,
		}),
	};

	let ast = parse_wkt(EXAMPLE2).unwrap();
	assert_eq!(ast.len(), 1);
	let bsgc = BaseStaticGeographicCrs::from_nodes(&ast).unwrap();

	assert_eq!(correct, bsgc.result);
}
