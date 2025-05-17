use crate::{ast::parse_wkt, id::Id, units::AngleUnit, utils::NumText};

use super::map_projection::{
    MapProjection, MapProjectionMethod, MapProjectionParameter, MapProjectionParameterUnit,
};

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

#[test]
fn test_map_projection() {
    let correct = MapProjection {
        map_projection_name: "Kyrgyzstan zone 3".to_string(),
        map_projection_method: MapProjectionMethod {
            map_projection_method_name: "Transverse Mercator".to_string(),
            identifier: Some(Id {
                authority_name: "EPSG".to_string(),
                authority_unique_identifier: NumText::Num(9807.0),
                version: None,
                authority_citation: None,
                id_uri: None,
            }),
        },
        map_projection_parameters: Some(vec![
            MapProjectionParameter {
                parameter_name: "Latitude of natural origin".to_string(),
                parameter_value: 0.0,
                map_projection_parameter_unit: Some(MapProjectionParameterUnit::AngleUnit(
                    AngleUnit {
                        unit_name: "degree".to_string(),
                        conversion_factor: 0.0174532925199433,
                        identifier: Some(Id {
                            authority_name: "EPSG".to_string(),
                            authority_unique_identifier: NumText::Num(9102.0),
                            version: None,
                            authority_citation: None,
                            id_uri: None,
                        }),
                    },
                )),
                identifier: Some(Id {
                    authority_name: "EPSG".to_string(),
                    authority_unique_identifier: NumText::Num(8801.0),
                    version: None,
                    authority_citation: None,
                    id_uri: None,
                }),
            },
            MapProjectionParameter {
                parameter_name: "Longitude of natural origin".to_string(),
                parameter_value: 74.516666666667,
                map_projection_parameter_unit: Some(MapProjectionParameterUnit::AngleUnit(
                    AngleUnit {
                        unit_name: "degree".to_string(),
                        conversion_factor: 0.0174532925199433,
                        identifier: Some(Id {
                            authority_name: "EPSG".to_string(),
                            authority_unique_identifier: NumText::Num(9102.0),
                            version: None,
                            authority_citation: None,
                            id_uri: None,
                        }),
                    },
                )),
                identifier: Some(Id {
                    authority_name: "EPSG".to_string(),
                    authority_unique_identifier: NumText::Num(8802.0),
                    version: None,
                    authority_citation: None,
                    id_uri: None,
                }),
            },
        ]),
        identifier: Some(Id {
            authority_name: "EPSG".to_string(),
            authority_unique_identifier: NumText::Num(7689.0),
            version: None,
            authority_citation: None,
            id_uri: None,
        }),
    };

    let ast = parse_wkt(EXAMPLE1);

    let map_proj = MapProjection::try_from(&ast).unwrap();

    assert_eq!(correct, map_proj);
}
