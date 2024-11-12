use crate::token::Token;

pub enum Stmt {
    Expr(Expr),
    Block(BlockStmt),
    Let(Expr, Expr),
    Return(Expr),
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

