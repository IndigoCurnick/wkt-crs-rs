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

#[test]
fn test_base_static_geographic_crs() {
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
				},
				anchor: None,
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

	let ast = parse_wkt(EXAMPLE1);
	assert_eq!(ast.len(), 1);
	let map_proj = BaseStaticGeographicCrs::from_nodes(&ast).unwrap();

	assert_eq!(correct, map_proj.result);
}
