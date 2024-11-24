use crate::token::Token;
use std::fmt::{ Display, Formatter };

pub enum Stmt {
    Expr(Expr),
    Block(BlockStmt),
    Let(Expr, Type, Expr),
    Assign(Expr, Expr),
    Return(Expr),
}

#[derive(Clone, Debug)]
pub struct Type {
    pub t: Token,
    pub mutable: bool
}

impl PartialEq for Type {
    fn eq(&self, t: &Type) -> bool {
        if self.t == t.t { return true }
        false
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        use Token::*;

        let s = match self.t {
            IntType => "int",
            FloatType => "float",
            ID(ref s) => s,
            _ => unreachable!()
        };

        write!(f, "{s}")
    }
}

impl Type {
    pub fn new(t: Token, mutable: bool) -> Self {
        Self { t, mutable }
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
