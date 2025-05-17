// ? it might make sense to turn this into NumInt and NumFloat?
#[derive(Debug, PartialEq)]
pub enum NumText {
    Num(f64),
    Text(String),
}
