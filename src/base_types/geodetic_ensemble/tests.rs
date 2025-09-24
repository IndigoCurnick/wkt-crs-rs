use crate::{
    ast::parse_wkt,
    base_types::{
        AngleUnit, DatumEnsembleAccuracy, DatumEnsembleMember, Ellipsoid, LengthUnit, PrimeMeridian,
    },
    types::WktBaseType,
};

use super::GeodeticDatumEnsemble;

const EXAMPLE1: &str = r#"ENSEMBLE["WGS 84 ensemble",
MEMBER["WGS 84 (TRANSIT)"],
MEMBER["WGS 84 (G730)"],
ELLIPSOID["WGS 84",6378137,298.2572236,LENGTHUNIT["metre",1.0]],
ENSEMBLEACCURACY[2.0]
],
PRIMEM["Greenwich",0.0,ANGLEUNIT["degree",0.017]]
"#;

#[test]
fn test_geodetic_ensemble() {
    let correct = GeodeticDatumEnsemble {
        datum_ensemble_name: "WGS 84 ensemble".into(),
        datum_ensemble_member: vec![
            DatumEnsembleMember {
                ensemble_member_name: "WGS 84 (TRANSIT)".into(),
                identifier: None,
            },
            DatumEnsembleMember {
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
        datum_ensemble_accuracy: DatumEnsembleAccuracy(2.0),
        identifier: None,
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
    };

    let ast = parse_wkt(EXAMPLE1);

    assert_eq!(ast.len(), 2);

    let geo = GeodeticDatumEnsemble::from_nodes(&ast).unwrap();

    assert_eq!(correct, geo.result);
    assert_eq!(geo.consumed, 2);
}
