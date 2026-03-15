use crate::error::WktParseError;

pub fn match_arity(
	args: usize,
	min: usize,
	max: usize,
) -> Result<(), WktParseError> {
	return if args >= min && args <= max {
		Ok(())
	} else {
		Err(WktParseError::IncorrectArity {
			min,
			max: Some(max),
			found: args,
		})
	};
}

pub fn lower_bound_arity(args: usize, min: usize) -> Result<(), WktParseError> {
	return if args < min {
		Err(WktParseError::IncorrectArity {
			min,
			max: None,
			found: args,
		})
	} else {
		Ok(())
	};
}
