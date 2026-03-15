use crate::{
	WktBaseType,
	ast::parse_wkt,
	base_types::{
		AngleUnit, BaseStaticGeodeticCrs, Ellipsoid, GeodeticReferenceFrame,
		LengthUnit, PrimeMeridian,
	},
	compound_types::GeodeticData,
};

// https://github.com/OSGeo/PROJ/issues/4321#issuecomment-2470878228
const EXAMPLE: &str = r#"BASEGEODCRS[
            "OSGB 1936",
            DATUM[
                "OSGB 1936",
                ELLIPSOID[
                    "Airy 1830",
                    6377563.396,
                    299.3249646,
                    LENGTHUNIT["metre", 1.0]
                ]
            ],
            PRIMEM["Greenwich", 0],
            UNIT["degree", 0.0174532925199433]
        ]"#;

#[test]
fn test_base_static_geodetic_crs() {
	let correct = BaseStaticGeodeticCrs {
		base_crs_name: "OSGB 1936".to_string(),
		geodetic_data: GeodeticData::GeodeticReferenceFrame(
			GeodeticReferenceFrame {
				datum_name: "OSGB 1936".to_string(),
				ellipsoid: Ellipsoid {
					ellipsoid_name: "Airy 1830".to_string(),
					semi_major_axis: 6377563.396,
					inverse_flattening: 299.3249646,
					length_unit: Some(LengthUnit {
						unit_name: "metre".to_string(),
						conversion_factor: 1.0,
						identifier: None,
					}),
				},
				anchor: None,
				identifier: None,
				prime_meridian: Some(PrimeMeridian {
					prime_meridian_name: "Greenwich".to_string(),
					irm_longitude: 0.0,
					angle_unit: None,
					identifier: None,
				}),
			},
		),
		ellipsoidal_cs_unit: Some(AngleUnit {
			unit_name: "degree".to_string(),
			conversion_factor: 0.0174532925199433,
			identifier: None,
		}),
		identifier: None,
	};

	let ast = parse_wkt(EXAMPLE);
	assert_eq!(ast.len(), 1);
	let map_proj = BaseStaticGeodeticCrs::from_nodes(&ast).unwrap();

	assert_eq!(correct, map_proj.result);
}
