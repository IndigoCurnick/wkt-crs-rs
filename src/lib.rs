use crate::{ast::parse_wkt, error::WktParseError};

pub use types::{WktBaseType, WktCrsTypes};

mod arity;
mod ast;
pub mod base_types;
pub mod compound_types;
pub mod data_types;
pub mod enumerations;
mod error;
mod keywords;
mod types;

pub fn parse_wkt_crs(text: &str) -> Result<Vec<WktCrsTypes>, WktParseError> {
	let ast = parse_wkt(text);

	let mut out = vec![];

	let mut i = 0;

	let len = ast.len();

	while i < len {
		let arr = &ast[i..len];

		let tmp = <WktCrsTypes as WktBaseType>::from_nodes(arr.iter())?;

		if tmp.consumed == 0 {
			panic!("Bug: consumed 0");
		}

		out.push(tmp.result);

		i += tmp.consumed;
	}

	return Ok(out);
}
