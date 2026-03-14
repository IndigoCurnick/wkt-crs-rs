use crate::{
    ast::parse_wkt,
    base_types::{AngleUnit, DatumAnchor, Ellipsoid, LengthUnit, PrimeMeridian},
    types::WktBaseType,
};

use super::GeodeticReferenceFrame;

const EXAMPLE1: &str = r#"DATUM["North American Datum 1983",
    ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1.0]]
]
"#;

const EXAMPLE2: &str = r#"TRF["World Geodetic System 1984",
    ELLIPSOID["WGS 84",6378388.0,298.257223563,LENGTHUNIT["metre",1.0]]
],
PRIMEM["Greenwich",0.0]
"#;

const EXAMPLE3: &str = r#"GEODETICDATUM["Tananarive 1925",
    ELLIPSOID["International 1924",6378388.0,297.0,LENGTHUNIT["metre",1.0]
    ],
    ANCHOR["Tananarive observatory:21.0191667gS, 50.23849537gE of Paris"]
],
PRIMEM["Paris",2.5969213,ANGLEUNIT["grad",0.015707]]
"#;

#[test]
fn test_datum() {
    // Example 1
    let correct = GeodeticReferenceFrame {
        datum_name: "North American Datum 1983".into(),
        ellipsoid: Ellipsoid {
            ellipsoid_name: "GRS 1980".into(),
            semi_major_axis: 6378137.0,
            inverse_flattening: 298.257222101,
            length_unit: Some(LengthUnit {
                unit_name: "metre".into(),
                conversion_factor: 1.0,
                identifier: None,
            }),
        },
        anchor: None,
        identifier: None,
        prime_meridian: None,
    };

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 1);

    let datum = GeodeticReferenceFrame::from_nodes(&ast).unwrap();

    assert_eq!(correct, datum.result);
    assert_eq!(datum.consumed, 1);

    // Example 2
    let correct = GeodeticReferenceFrame {
        datum_name: "World Geodetic System 1984".into(),
        ellipsoid: Ellipsoid {
            ellipsoid_name: "WGS 84".into(),
            semi_major_axis: 6378388.0,
            inverse_flattening: 298.257223563,
            length_unit: Some(LengthUnit {
                unit_name: "metre".into(),
                conversion_factor: 1.0,
                identifier: None,
            }),
        },
        anchor: None,
        identifier: None,
        prime_meridian: Some(PrimeMeridian {
            prime_meridian_name: "Greenwich".into(),
            irm_longitude: 0.0,
            angle_unit: None,
            identifier: None,
        }),
    };

    let ast = parse_wkt(EXAMPLE2);

    assert_eq!(ast.len(), 2);

    let datum = GeodeticReferenceFrame::from_nodes(&ast).unwrap();

    assert_eq!(correct, datum.result);
    assert_eq!(datum.consumed, 2);

    // Example 1
    let correct = GeodeticReferenceFrame {
        datum_name: "Tananarive 1925".into(),
        ellipsoid: Ellipsoid {
            ellipsoid_name: "International 1924".into(),
            semi_major_axis: 6378388.0,
            inverse_flattening: 297.0,
            length_unit: Some(LengthUnit {
                unit_name: "metre".into(),
                conversion_factor: 1.0,
                identifier: None,
            }),
        },
        anchor: Some(DatumAnchor(
            "Tananarive observatory:21.0191667gS, 50.23849537gE of Paris".into(),
        )),
        identifier: None,
        prime_meridian: Some(PrimeMeridian {
            prime_meridian_name: "Paris".into(),
            irm_longitude: 2.5969213,
            angle_unit: Some(AngleUnit {
                unit_name: "grad".into(),
                conversion_factor: 0.015707,
                identifier: None,
            }),
            identifier: None,
        }),
    };

    let ast = parse_wkt(EXAMPLE3);

    let datum = GeodeticReferenceFrame::from_nodes(&ast).unwrap();

    assert_eq!(correct, datum.result);
    assert_eq!(datum.consumed, 2);
}
