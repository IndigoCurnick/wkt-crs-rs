use crate::{
	ast::parse_wkt,
	base_types::{AngleUnit, Id, LengthUnit, Method, Parameter, ScaleUnit},
	compound_types::{SpatialUnit, Unit},
	data_types::NumText,
	types::WktBaseType,
};

use super::map_projection::MapProjection;

const EXAMPLE1: &str = r#"CONVERSION["Kyrgyzstan zone 3",
    METHOD["Transverse Mercator",
        ID["EPSG",9807]],
    PARAMETER["Latitude of natural origin",0,
        ANGLEUNIT["degree",0.0174532925199433,
            ID["EPSG",9102]],
        ID["EPSG",8801]],
    PARAMETER["Longitude of natural origin",74.516666666667,
        ANGLEUNIT["degree",0.0174532925199433,
            ID["EPSG",9102]],
        ID["EPSG",8802]],
ID["EPSG",7689]]"#;

const EXAMPLE2: &str = r#"CONVERSION["UTM zone 10N",
    METHOD["Transverse Mercator",ID["EPSG",9807]],
    PARAMETER["Latitude of natural origin",0,
        ANGLEUNIT["degree",0.017],
        ID["EPSG",8801]],
    PARAMETER["Longitude of natural origin",-123,
        ANGLEUNIT["degree",0.017],ID["EPSG",8802]],
    PARAMETER["Scale factor at natural origin",0.9996,
        SCALEUNIT["unity",1.0],ID["EPSG",8805]],
    PARAMETER["False easting",500000,
        LENGTHUNIT["metre",1.0],ID["EPSG",8806]],
    PARAMETER["False northing",0,LENGTHUNIT["metre",1.0],ID["EPSG",8807]]
]
"#;

const EXAMPLE3: &str = r#"CONVERSION["UTM zone 10N",
    METHOD["Transverse Mercator"],
    PARAMETER["Latitude of natural origin",0,
        ANGLEUNIT["degree",0.017]],
    PARAMETER["Longitude of natural origin",-123,
        ANGLEUNIT["degree",0.017]],
    PARAMETER["Scale factor at natural origin",0.9996,
        SCALEUNIT["unity",1.0]],
    PARAMETER["False easting",500000,
        LENGTHUNIT["metre",1.0]],
    PARAMETER["False northing",0,LENGTHUNIT["metre",1.0]],
    ID["EPSG",16010]
]
"#;

#[test]
fn test_map_projection() {
	test_example_1();
	test_example_2();
	test_example_3();
}

fn test_example_1() {
	let correct = MapProjection {
		map_projection_name: "Kyrgyzstan zone 3".to_string(),
		map_projection_method: Method {
			method_name: "Transverse Mercator".to_string(),
			identifier: Some(Id {
				authority_name: "EPSG".to_string(),
				authority_unique_identifier: NumText::Int(9807),
				version: None,
				authority_citation: None,
				id_uri: None,
			}),
		},
		map_projection_parameters: Some(vec![
			Parameter {
				parameter_name: "Latitude of natural origin".to_string(),
				parameter_value: 0.0,
				parameter_unit: Some(Unit::SpatialUnit(
					SpatialUnit::AngleUnit(AngleUnit {
						unit_name: "degree".to_string(),
						conversion_factor: 0.0174532925199433,
						identifier: Some(Id {
							authority_name: "EPSG".to_string(),
							authority_unique_identifier: NumText::Int(9102),
							version: None,
							authority_citation: None,
							id_uri: None,
						}),
					}),
				)),
				identifier: Some(Id {
					authority_name: "EPSG".to_string(),
					authority_unique_identifier: NumText::Int(8801),
					version: None,
					authority_citation: None,
					id_uri: None,
				}),
			},
			Parameter {
				parameter_name: "Longitude of natural origin".to_string(),
				parameter_value: 74.516666666667,
				parameter_unit: Some(Unit::SpatialUnit(
					SpatialUnit::AngleUnit(AngleUnit {
						unit_name: "degree".to_string(),
						conversion_factor: 0.0174532925199433,
						identifier: Some(Id {
							authority_name: "EPSG".to_string(),
							authority_unique_identifier: NumText::Int(9102),
							version: None,
							authority_citation: None,
							id_uri: None,
						}),
					}),
				)),
				identifier: Some(Id {
					authority_name: "EPSG".to_string(),
					authority_unique_identifier: NumText::Int(8802),
					version: None,
					authority_citation: None,
					id_uri: None,
				}),
			},
		]),
		identifier: Some(Id {
			authority_name: "EPSG".to_string(),
			authority_unique_identifier: NumText::Int(7689),
			version: None,
			authority_citation: None,
			id_uri: None,
		}),
	};

	let ast = parse_wkt(EXAMPLE1).unwrap();
	assert_eq!(ast.len(), 1);
	let map_proj = MapProjection::from_nodes(&ast).unwrap();

	assert_eq!(correct, map_proj.result);
}

fn test_example_2() {
	let correct = MapProjection {
		map_projection_name: "UTM zone 10N".to_string(),
		map_projection_method: Method {
			method_name: "Transverse Mercator".to_string(),
			identifier: Some(Id {
				authority_name: "EPSG".to_string(),
				authority_unique_identifier: NumText::Int(9807),
				version: None,
				authority_citation: None,
				id_uri: None,
			}),
		},
		map_projection_parameters: Some(vec![
			Parameter {
				parameter_name: "Latitude of natural origin".to_string(),
				parameter_value: 0.0,
				parameter_unit: Some(Unit::SpatialUnit(
					SpatialUnit::AngleUnit(AngleUnit {
						unit_name: "degree".to_string(),
						conversion_factor: 0.017,
						identifier: None,
					}),
				)),
				identifier: Some(Id {
					authority_name: "EPSG".to_string(),
					authority_unique_identifier: NumText::Int(8801),
					version: None,
					authority_citation: None,
					id_uri: None,
				}),
			},
			Parameter {
				parameter_name: "Longitude of natural origin".to_string(),
				parameter_value: -123.0,
				parameter_unit: Some(Unit::SpatialUnit(
					SpatialUnit::AngleUnit(AngleUnit {
						unit_name: "degree".to_string(),
						conversion_factor: 0.017,
						identifier: None,
					}),
				)),
				identifier: Some(Id {
					authority_name: "EPSG".to_string(),
					authority_unique_identifier: NumText::Int(8802),
					version: None,
					authority_citation: None,
					id_uri: None,
				}),
			},
			Parameter {
				parameter_name: "Scale factor at natural origin".to_string(),
				parameter_value: 0.9996,
				parameter_unit: Some(Unit::SpatialUnit(
					SpatialUnit::ScaleUnit(ScaleUnit {
						unit_name: "unity".into(),
						conversion_factor: 1.0,
						identifier: None,
					}),
				)),
				identifier: Some(Id {
					authority_name: "EPSG".to_string(),
					authority_unique_identifier: NumText::Int(8805),
					version: None,
					authority_citation: None,
					id_uri: None,
				}),
			},
			Parameter {
				parameter_name: "False easting".to_string(),
				parameter_value: 500000.0,
				parameter_unit: Some(Unit::SpatialUnit(
					SpatialUnit::LengthUnit(LengthUnit {
						unit_name: "metre".into(),
						conversion_factor: 1.0,
						identifier: None,
					}),
				)),
				identifier: Some(Id {
					authority_name: "EPSG".to_string(),
					authority_unique_identifier: NumText::Int(8806),
					version: None,
					authority_citation: None,
					id_uri: None,
				}),
			},
			Parameter {
				parameter_name: "False northing".to_string(),
				parameter_value: 0.0,
				parameter_unit: Some(Unit::SpatialUnit(
					SpatialUnit::LengthUnit(LengthUnit {
						unit_name: "metre".into(),
						conversion_factor: 1.0,
						identifier: None,
					}),
				)),
				identifier: Some(Id {
					authority_name: "EPSG".to_string(),
					authority_unique_identifier: NumText::Int(8807),
					version: None,
					authority_citation: None,
					id_uri: None,
				}),
			},
		]),
		identifier: None,
	};

	let ast = parse_wkt(EXAMPLE2).unwrap();
	assert_eq!(ast.len(), 1);
	let map_proj = MapProjection::from_nodes(&ast).unwrap();

	assert_eq!(correct, map_proj.result);
}

fn test_example_3() {
	let correct = MapProjection {
		map_projection_name: "UTM zone 10N".to_string(),
		map_projection_method: Method {
			method_name: "Transverse Mercator".to_string(),
			identifier: None,
		},
		map_projection_parameters: Some(vec![
			Parameter {
				parameter_name: "Latitude of natural origin".to_string(),
				parameter_value: 0.0,
				parameter_unit: Some(Unit::SpatialUnit(
					SpatialUnit::AngleUnit(AngleUnit {
						unit_name: "degree".to_string(),
						conversion_factor: 0.017,
						identifier: None,
					}),
				)),
				identifier: None,
			},
			Parameter {
				parameter_name: "Longitude of natural origin".to_string(),
				parameter_value: -123.0,
				parameter_unit: Some(Unit::SpatialUnit(
					SpatialUnit::AngleUnit(AngleUnit {
						unit_name: "degree".to_string(),
						conversion_factor: 0.017,
						identifier: None,
					}),
				)),
				identifier: None,
			},
			Parameter {
				parameter_name: "Scale factor at natural origin".to_string(),
				parameter_value: 0.9996,
				parameter_unit: Some(Unit::SpatialUnit(
					SpatialUnit::ScaleUnit(ScaleUnit {
						unit_name: "unity".into(),
						conversion_factor: 1.0,
						identifier: None,
					}),
				)),
				identifier: None,
			},
			Parameter {
				parameter_name: "False easting".to_string(),
				parameter_value: 500000.0,
				parameter_unit: Some(Unit::SpatialUnit(
					SpatialUnit::LengthUnit(LengthUnit {
						unit_name: "metre".into(),
						conversion_factor: 1.0,
						identifier: None,
					}),
				)),
				identifier: None,
			},
			Parameter {
				parameter_name: "False northing".to_string(),
				parameter_value: 0.0,
				parameter_unit: Some(Unit::SpatialUnit(
					SpatialUnit::LengthUnit(LengthUnit {
						unit_name: "metre".into(),
						conversion_factor: 1.0,
						identifier: None,
					}),
				)),
				identifier: None,
			},
		]),
		identifier: Some(Id {
			authority_name: "EPSG".into(),
			authority_unique_identifier: NumText::Int(16010),
			version: None,
			authority_citation: None,
			id_uri: None,
		}),
	};

	let ast = parse_wkt(EXAMPLE3).unwrap();
	assert_eq!(ast.len(), 1);
	let map_proj = MapProjection::from_nodes(&ast).unwrap();

	assert_eq!(correct, map_proj.result);
}
