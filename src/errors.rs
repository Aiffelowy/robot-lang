use std::error::Error;
use std::fmt::{self, Formatter};

use crate::token::Token;


#[derive(Debug)]
pub enum ParseError {
    UnknownToken(usize, char),
    WrongToken(usize, Token, Token),
    UnexpectedToken(usize, Token),
    UninitializedValue,
    InternalError,
    WaitForInput
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let fmt_str :String = match self {
            Self::UnknownToken(_pos, c) => format!("Unknown token: {c}"),
            Self::WrongToken(_pos, tte, ttf) => format!("Unexpected token: expecting {tte}, found {ttf}"),
            Self::UnexpectedToken(_pos, ttf) => format!("Unexpected token: {ttf}"),
            Self::UninitializedValue => format!("Uninitialized value accessed"),
            Self::InternalError => "Internal Error".to_string(),
            Self::WaitForInput => "".to_string(),
        };

        write!(f, "{fmt_str}")
    }
}

impl Error for ParseError {}

