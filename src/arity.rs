use crate::error::WktParseError;

pub fn match_arity(args: usize, min: usize, max: usize) -> Result<(), WktParseError> {
    return if args >= min && args <= max {
        Ok(())
    } else {
        Err(WktParseError::IncorrectArity {
            min,
            max,
            found: args,
        })
    };
}
