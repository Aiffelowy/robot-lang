use std::io::{self, BufRead};

use lexer::Lexer;
use interpreter::Interpreter;
use object::Object;
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
    let args: Vec<String> = std::env::args().collect();
    let lexer = Lexer::new();
    let parser = Parser::new(lexer);
    let mut inter = Interpreter::new(parser);
    let mut append :bool = false;

    if args.len() == 2 {
        let contents = std::fs::read_to_string(args[1].clone())
            .expect("Should have been able to read the file");
        inter.feed_next_line(contents, &mut append);
        match inter.interpret() {
            Ok(res) if res == Object::Null => (),
            Ok(res) => println!("{}", res),
            Err(e) => match e {
                _ => println!("{:?}", e),
            }
        }
        return;
    }

    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        if let Ok(line) = l {
            if line.len() == 0 { continue; }
            inter.feed_next_line(line, &mut append);
            match inter.interpret() {
                Ok(res) if res == Object::Null => (),
                Ok(res) => println!("{}", res),
                Err(e) => match e {
                    _ => println!("{:?}", e),
                }
            }
        }
    }
}
