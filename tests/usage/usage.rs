use wkt_crs_rs::{
    WktCrsTypes,
    base_types::{AreaDescription, Extent, GeographicBoundingBox, Scope, Usage},
    parse_wkt_crs,
};

const EXAMPLE: &str = r#"USAGE[SCOPE["Low accuracy applications."],
        AREA["Australia onshore"],BBOX[-43.7,112.85,-9.87,153.68]]"#;

#[test]
fn test_usage() {
    let correct = Usage {
        scope: Scope("Low accuracy applications.".into()),
        extent: Extent {
            area_description: Some(AreaDescription("Australia onshore".into())),
            geographic_bounding_box: Some(GeographicBoundingBox {
                lower_left_latitude: -43.7,
                lower_left_longitude: 112.85,
                upper_right_latitude: -9.87,
                upper_right_longitude: 153.68,
            }),
            temporal_extent: None,
            vertical_extent: None,
        },
    };

    let correct = vec![WktCrsTypes::Usage(correct)];

    let ast = parse_wkt_crs(EXAMPLE).unwrap();

    assert_eq!(ast, correct);
}
