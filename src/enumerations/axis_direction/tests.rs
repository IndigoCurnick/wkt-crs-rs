use std::str::FromStr;

use crate::{
	ast::WktArg,
	enumerations::{
		AxisDirection, axis_direction::axis_direction::AxisDirectionInner,
	},
};

#[test]
fn test_axis_drection() {
	{
		let example = WktArg::Data("north".to_string());

		let correct = AxisDirection::North(None);

		let ans = AxisDirection::try_from((&example, None)).unwrap();

		assert_eq!(ans, correct);
	}

	{
		let example = "east";

		let correct = AxisDirection::East;

		let ans = AxisDirection::from_str(example).unwrap();

		assert_eq!(ans, correct);
	}
}

#[test]
fn test_axis_direction_inner() {
	{
		let example = "north";

		let correct = AxisDirectionInner::North;

		let ans = AxisDirectionInner::from_str(example).unwrap();

		assert_eq!(ans, correct);
	}
}
