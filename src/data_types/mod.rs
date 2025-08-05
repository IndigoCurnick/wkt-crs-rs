use horologium::Temporal;

#[derive(Debug, PartialEq)]
pub enum DateOrString {
    Date(Temporal),
    String(String),
}

#[derive(Debug, PartialEq)]
pub enum NumText {
    Int(i32),
    Text(String),
    Float(f64),
}

impl From<&str> for NumText {
    fn from(value: &str) -> Self {
        return if let Ok(float) = value.parse() {
            Self::Float(float)
        } else if let Ok(int) = value.parse() {
            Self::Int(int)
        } else {
            Self::Text(value.to_string())
        };
    }
}
