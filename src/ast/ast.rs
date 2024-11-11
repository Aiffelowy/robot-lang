/*pub enum Stmt {
    Expr(Expr),
    Let(String, Expr)
}*/

#[derive(Debug, Clone)]
pub enum Expr {
    NumLit(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    UnaryPlus(Box<Expr>),
    UnaryMinus(Box<Expr>),
    Scope(Vec<Box<Expr>>),
    Assign(Box<Expr>, Box<Expr>),
    Var(String),
    NoOp
}

