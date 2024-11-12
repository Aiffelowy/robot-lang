use std::io::{self, BufRead};

use lexer::Lexer;
use interpreter::Interpreter;
use parser::Parser;

pub mod interpreter;
pub mod parser;
pub mod lexer;
pub mod token;
pub mod ast;
pub mod errors;
pub mod misc;
pub mod symbols;
pub mod object;


fn main() {
    let stdin = io::stdin();
    let lexer = Lexer::new();
    let parser = Parser::new(lexer);
    let mut inter = Interpreter::new(parser);
    let mut append :bool = false;

    for l in stdin.lock().lines() {
        if let Ok(line) = l {
            if line.len() == 0 { continue; }
            inter.feed_next_line(line, &mut append);
            match inter.interpret() {
                Ok(res) => println!("{:?}", res),
                Err(e) => match e {
                    _ => println!("{:?}", e),
                }
            }
        }
    }
}
