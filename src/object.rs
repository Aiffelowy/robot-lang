use std::fmt::{ Display, Formatter };

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Int(i64),
    Float(f64),
    Str(String),
    Return(Box<Object>),
    Null,
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Object::*;

        match self {
            Int(i) => write!(f, "{i}"),
            Float(fl) => write!(f, "{fl}"),
            Str(s) => write!(f, "{s}"),
            Return(obj) => write!(f, "{}", obj.to_string()),
            Null => write!(f, "Null")
        }
    }
}
