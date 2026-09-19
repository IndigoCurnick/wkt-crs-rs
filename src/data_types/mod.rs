use horologium::Temporal;

#[derive(Debug, Clone, PartialEq)]
pub enum DateOrString {
	Date(Temporal),
	String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumText {
	Int(i32),
	Text(String),
	Float(f64),
}

impl From<&str> for NumText {
	fn from(value: &str) -> Self {
		return if let Ok(int) = value.parse() {
			Self::Int(int)
		} else if let Ok(float) = value.parse() {
			Self::Float(float)
		} else {
			Self::Text(value.to_string())
		};
	}
}
