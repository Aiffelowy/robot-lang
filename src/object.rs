
#[derive(Debug, Clone)]
pub enum Object {
    Int(i64),
    Float(f64),
    Str(String),
    Return(Box<Object>),
    Null,
}
