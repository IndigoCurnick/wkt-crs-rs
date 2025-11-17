use crate::{error::WktParseError, types::WktCrsTypes};

mod arity;
mod ast;
mod base_types;
mod compound_types;
mod data_types;
mod enumerations;
mod error;
mod keywords;
mod types;

pub fn parse_wkt_crs(text: &str) -> Result<Vec<WktCrsTypes>, WktParseError> {
    todo!();
}
