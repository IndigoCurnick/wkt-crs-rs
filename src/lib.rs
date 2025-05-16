#[derive(Debug)]
pub struct WktNode {
    keyword: String,
    args: Vec<WktArg>,
}

#[derive(Debug)]
pub enum WktArg {
    String(String),
    Number(f64),
    Node(WktNode),
}
