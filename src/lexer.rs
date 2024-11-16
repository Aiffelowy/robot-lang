use crate::misc::defer;
use crate::{defer, expr};
use crate::token::{Token, RESERVED_KEYWORDS};
use crate::errors::ParseError;

#[derive(Clone)]
pub struct Lexer {
    text: String,
    pub pos: usize,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            text: "".to_string(),
            pos: 0
        }
    }

    //give next line to the interpreter to interpret; clones the string - MUST BE ASCII
    pub fn feed_next_line(&mut self, line: String, append: bool) {
        if append {
            self.text.push_str(&line);
        } else {
            self.text = line;
        }
        self.pos = 0;
    }

    //gets the char at self.pos in the given string. Returns None if self.pos is beyond bounds
    fn get_current_char(&mut self) -> Option<char> {
        if self.pos > self.text.len()-1 {
            return None
        }
        
        Some(self.text.as_bytes()[self.pos] as char)
    }

    //moves self.pos to the next position
    fn advance(&mut self) {
        self.pos += 1;
    }

    fn peek(&self) -> Option<char> {
        if self.pos > self.text.len() - 2 {
            return None
        }

        Some(self.text.as_bytes()[self.pos + 1] as char)
    }

    //skips the whitespace in the string to the next non-whitespace char
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.get_current_char() {
            if !c.is_whitespace() { break; }
            self.advance();
        }
    }

    fn skip_comment(&mut self) {
        self.advance();
        while let Some(c) = self.get_current_char() {
            if c == '#' || c == '\n' { break; }
            self.advance();
        }

        self.advance();
    }
    
    fn id(&mut self) -> Result<Token, ParseError> {
        let mut res = String::new();
        while let Some(cur_char) = self.get_current_char() {
            if !cur_char.is_alphanumeric() { break; }
            res.push(cur_char);
            self.advance();
        }

        if let Some(t) = RESERVED_KEYWORDS.get(&*res) {
            return Ok(t.clone());
        }

        return Ok(Token::ID(res));
    }

    //iterates over the adjecient digits returning the token with the whole number
    fn number(&mut self) -> Result<Token, ParseError> {
        let mut i :String = String::new();
        let mut float :bool = false;
        while let Some(cur_char) = self.get_current_char() {
            if !float && cur_char == '.' {
                float = true;
            } else if !cur_char.is_digit(10) {
                break;
            }

            i.push(cur_char);
            self.advance();
        }

        if float {
            match i.parse::<f64>() {
                Ok(i) => return Ok(Token::Float(i)),
                Err(_) => return Err(ParseError::InternalError)
            };
        }

        match i.parse::<i64>() {
            Ok(i) => return Ok(Token::Number(i)),
            Err(_) => return Err(ParseError::InternalError)
        };
    }

    //Lexical Analyzer; breaks the sentence into tokens, returns the next token in the stream
    pub fn next_token(&mut self) -> Result<Token, ParseError> {
        use Token::*;

        self.skip_whitespace();
        while let Some(cur_char) = self.get_current_char() {
            macro_rules! define_token {
                {$($c:literal => $t:expr),+} => {
                    match cur_char {
                        $(
                            $c => return Ok($t),
                        )+
                        _ => (),
                    }
                };

                ($e: expr, $b: expr) => {
                    if $e { $b };
                }
            }

            if cur_char == '#' {
                self.skip_comment();
                self.skip_whitespace();
                continue;
            }

            let pos = self.pos;

            define_token!(cur_char.is_digit(10), return self.number());
            define_token!(cur_char.is_alphanumeric(), return self.id());

            defer!(self.advance());
            define_token!{
                '+' => Plus,
                '-' => Minus,
                '*' => Multiply,
                '/' => Divide,
                '(' => LeftParen,
                ')' => RightParen,
                '{' => LeftCurly,
                '}' => RightCurly,
                '=' => Equal,
                ';' => Semicolon,
                ':' => Colon,
                ',' => Comma
            }

            return Err(ParseError::UnknownToken(pos, cur_char));
        }

        return Ok(Token::EOF);
    }

}
