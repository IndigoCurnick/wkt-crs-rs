use std::str::FromStr;

use crate::enumerations::SpatialCsType;

#[test]
fn test_spatial_cs_type() {
	let example = "Cartesian";

	let correct = SpatialCsType::Cartesian;

	let ans = SpatialCsType::from_str(example).unwrap();

	assert_eq!(ans, correct);
}
