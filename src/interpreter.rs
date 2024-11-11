use crate::errors::ParseError;
use crate::{ast::visitor::Visitor, parser::Parser};
use crate::ast::ast::Expr;
use std::collections::HashMap;


pub struct Interpreter {
    parser: Parser,
    global_scope: HashMap<String, i64>
}


impl Interpreter {
    pub fn new(parser: Parser) -> Self {
        Self { parser, global_scope: HashMap::new() }
    }

    pub fn interpret(&mut self) -> Result<i64, ParseError> {
        let tree = self.parser.parse()?;
        Ok(self.visit_expr(&*tree))
    }

    pub fn feed_next_line(&mut self, line: String, append: &mut bool) {
        self.parser.feed_next_line(line, *append);
        *append = false;
    }
}


impl Visitor<i64> for Interpreter {
    fn visit_expr(&mut self, e: &crate::ast::ast::Expr) -> i64 {
        use Expr::*;

        match *e {
            NumLit(n) => n,
            Add(ref left, ref right) => self.visit_expr(left) + self.visit_expr(right),
            Sub(ref left, ref right) => self.visit_expr(left) - self.visit_expr(right),
            Mult(ref left, ref right) => self.visit_expr(left) * self.visit_expr(right),
            Div(ref left, ref right) => self.visit_expr(left) / self.visit_expr(right),
            UnaryMinus(ref expr) => -self.visit_expr(expr),
            UnaryPlus(ref expr) => self.visit_expr(expr),
            Scope(ref expr_vec) => { let mut it = expr_vec.iter().peekable(); while let Some(child) = it.next() { let res = self.visit_expr(child); if it.peek().is_none() { return res; }  } 0 },
            NoOp => 0,
            Assign(ref id, ref expr) => { if let Var(vid) = *id.clone() { let res = self.visit_expr(expr); self.global_scope.insert(vid.to_string(), res); } 0 }
            Var(ref id) => return match self.global_scope.get(id) {
                Some(i) => *i,
                None => 0
            }
        }
    }
}
