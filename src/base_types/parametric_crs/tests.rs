use crate::{
	ast::parse_wkt,
	base_types::{
		Axis, CoordinateSystem, DatumAnchor, ParametricDatum, ParametricUnit,
		SpatialCoordinateSystem,
	},
	compound_types::{ScopeExtentIdentifierRemark, SpatialUnit, Unit},
	enumerations::{AxisDirection, Dimension, SpatialCsType},
	types::WktBaseType,
};

use super::parametric_crs::ParametricCrs;

const EXAMPLE1: &str = r#"PARAMETRICCRS["WMO standard atmosphere layer 0",
    PDATUM["Mean Sea Level",ANCHOR["1013.25 hPa at 15C"]],
    CS[parametric,1],
        AXIS["presure (hPa)",up],PARAMETRICUNIT["HectoPascal",100.0]
]
"#;

#[test]
fn test_parametric_unit() {
	let correct = ParametricCrs {
		crs_name: "WMO standard atmosphere layer 0".to_string(),
		parametric_datum: ParametricDatum {
			datum_name: "Mean Sea Level".to_string(),
			datum_anchor: Some(DatumAnchor("1013.25 hPa at 15C".to_string())),
			identifier: None,
		},
		coordinate_system: CoordinateSystem::SpatialCS(
			SpatialCoordinateSystem {
				spatial_cs_type: SpatialCsType::Parametric,
				dimension: Dimension::One,
				identifier: None,
				spatial_axis: vec![Axis {
					axis_name_abbreviation: "presure (hPa)".to_string(),
					axis_direction: AxisDirection::Up,
					axis_order: None,
					unit: None,
					identifier: None,
				}],
				cs_unit: Some(Unit::SpatialUnit(SpatialUnit::ParametricUnit(
					ParametricUnit {
						unit_name: "HectoPascal".into(),
						conversion_factor: 100.0,
						identifier: None,
					},
				))),
			},
		),
		scope_extent_identifier_remark: ScopeExtentIdentifierRemark {
			usage: None,
			identifier: None,
			remark: None,
		},
	};

	let ast = parse_wkt(EXAMPLE1).unwrap();

	assert_eq!(ast.len(), 1);

	let para = ParametricCrs::from_nodes(&ast).unwrap();

	assert_eq!(correct, para.result);
}
