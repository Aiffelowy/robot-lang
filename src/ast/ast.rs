use crate::token::Token;
use std::fmt::{ Display, Formatter };

pub enum Stmt {
    Expr(Expr),
    Block(BlockStmt),
    Let(Expr, Type, Expr),
    Assign(Expr, Expr),
    Return(Expr),
}

type Mutable = bool;


#[derive(Clone, Debug)]
pub struct Type {
    pub t: String,
    pub mutable: bool
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.t)
    }
}

pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

impl BlockStmt {
    pub fn new() -> Self {
        Self { statements: vec![] }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    NumLit(i64),
    FloatLit(f64),
    Infix(Box<Expr>, Token, Box<Expr>),
    Prefix(Token, Box<Expr>),
    Var(String),
    Noop,
}
