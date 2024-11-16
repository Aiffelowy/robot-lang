use std::collections::HashMap;
use std::fmt::{Display, Formatter};

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    }
);

use crate::errors::ParseError;
enum BuiltInTypes {
    Integer,
    Float
}

impl Display for BuiltInTypes {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let s = match self {
            Self::Integer => "int",
            Self::Float => "float"
        };

        write!(f, "{s}")
    }
}

enum Symbol {
    BuiltInType(BuiltInTypes),
    Var(String, BuiltInTypes)
}


struct SymbolTable {
    table: HashMap<String, Symbol>
}

impl SymbolTable {

    pub fn new() -> Self {
        Self { table: map!(
            "int".to_string() => Symbol::BuiltInType(BuiltInTypes::Integer),
            "float".to_string() => Symbol::BuiltInType(BuiltInTypes::Float)
    ) }
    }

    pub fn define(&mut self, symbol: Symbol) {
        match &symbol {
            Symbol::BuiltInType(t) => self.table.insert(t.to_string(), symbol),
            Symbol::Var(name, _) => self.table.insert(name.to_string(), symbol),
        };
    }

    pub fn lookup(&self, name: &str) -> Result<&Symbol, ParseError> {
        match self.table.get(name) {
            Some(s) => Ok(s),
            None => Err(ParseError::UndefinedSymbol(name.to_string()))
        }
    }
}
