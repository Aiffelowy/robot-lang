use std::fmt::{ Display, Formatter, Error };
use std::collections::HashMap;

use lazy_static::lazy_static;

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


#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(i64),
    Equal,
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    Semicolon,
    Let,
    ID(String),
    EOF
}


lazy_static! {
    pub static ref RESERVED_KEYWORDS :HashMap<&'static str, Token> = map!{
        "let" => Token::Let
    };
}


impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}


impl Token {
    pub fn is(&self, tokens: &[Token]) -> bool {
        for token in tokens {
            if self == token { return true; }
        }

        return false;
    }
}

