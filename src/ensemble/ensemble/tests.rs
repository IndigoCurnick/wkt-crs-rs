use crate::{
    ast::parse_wkt,
    ellipsoid::Ellipsoid,
    ensemble::{
        ensemble_accuracy::EnsembleAccuracy, ensemble_member::EnsembleMember,
        geodetic_ensemble::GeodeticEnsemble, vertical_ensemble::VerticalEnsemble,
    },
    prime_meridian::PrimeMeridian,
    units::{AngleUnit, LengthUnit},
};

use super::DataEnsemble;

const EXAMPLE1: &str = r#"ENSEMBLE["WGS 84 ensemble",
MEMBER["WGS 84 (TRANSIT)"],
MEMBER["WGS 84 (G730)"],
ELLIPSOID["WGS 84",6378137,298.2572236,LENGTHUNIT["metre",1.0]],
ENSEMBLEACCURACY[2.0]
],
PRIMEM["Greenwich",0.0,ANGLEUNIT["degree",0.017]]
"#;

const EXAMPLE2: &str = r#"ENSEMBLE["EVRS ensemble",
MEMBER["EVRF2000"],
MEMBER["EVRF2007"],
ENSEMBLEACCURACY[0.01]]
"#;

#[test]
fn test_ensemble() {
    // Example 1

    let correct = DataEnsemble::GeodeticEnsemble(GeodeticEnsemble {
        datum_ensemble_name: "WGS 84 ensemble".into(),
        datum_ensemble_member: vec![
            EnsembleMember {
                ensemble_member_name: "WGS 84 (TRANSIT)".into(),
                identifier: None,
            },
            EnsembleMember {
                ensemble_member_name: "WGS 84 (G730)".into(),
                identifier: None,
            },
        ],
        ellipsoid: Ellipsoid {
            ellipsoid_name: "WGS 84".into(),
            semi_major_axis: 6378137.0,
            inverse_flattening: 298.2572236,
            length_unit: Some(LengthUnit {
                unit_name: "metre".into(),
                conversion_factor: 1.0,
            }),
        },
        datum_ensemble_accuracy: EnsembleAccuracy(2.0),
        prime_meridian: PrimeMeridian {
            prime_meridian_name: "Greenwich".into(),
            irm_longitude: 0.0,
            angle_unit: Some(AngleUnit {
                unit_name: "degree".into(),
                conversion_factor: 0.017,
                identifier: None,
            }),
            identifier: None,
        },
    });

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 2);

    let en = DataEnsemble::try_from((&ast[0], Some(&ast[1]))).unwrap();

    assert_eq!(correct, en);

    // Example 2
    let correct = DataEnsemble::VerticalEnsemble(VerticalEnsemble {
        datum_ensemble_name: "EVRS ensemble".into(),
        datum_ensemble_member: vec![
            EnsembleMember {
                ensemble_member_name: "EVRF2000".into(),
                identifier: None,
            },
            EnsembleMember {
                ensemble_member_name: "EVRF2007".into(),
                identifier: None,
            },
        ],
        datum_ensemble_accuracy: EnsembleAccuracy(0.01),
        identifier: None,
    });

    let ast = parse_wkt(EXAMPLE2);

    assert_eq!(ast.len(), 1);

    let en = DataEnsemble::try_from((&ast[0], None)).unwrap();

    assert_eq!(correct, en);
}
