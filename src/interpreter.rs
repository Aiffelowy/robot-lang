use crate::errors::ParseError;
use crate::object::Object;
use crate::{ast::ast::*, parser::Parser};
use std::collections::HashMap;
use crate::token::Token;
use crate::symbols::{self, SymbolError, SymbolTableBuilder};

#[derive(Debug)]
pub enum InterpreterError {
    SomeError,
    ParseError(ParseError),
    SymbolError(SymbolError)
}

pub struct Interpreter {
    parser: Parser,
    symtable: SymbolTableBuilder,
    global_scope: HashMap<String, Object>
}


impl Interpreter {
    pub fn new(parser: Parser) -> Self {
        Self { parser, global_scope: HashMap::new(), symtable: SymbolTableBuilder::new() } }

    pub fn interpret(&mut self) -> Result<Object, InterpreterError> {
        let tree = match self.parser.parse() {
            Ok(tree) => tree,
            Err(e) => return Err(InterpreterError::ParseError(e))
        };
        
        match self.symtable.check(&tree) {
            Ok(()) => (),
            Err(e) => return Err(InterpreterError::SymbolError(e))
        }
        Ok(self.visit_block_stmt(&tree)?)
    }

    pub fn feed_next_line(&mut self, line: String, append: &mut bool) {
        self.parser.feed_next_line(line, *append);
        *append = false;
    }

    fn int_expr(lhs: &i64, token: &Token, rhs: &i64) -> Result<Object, InterpreterError> {
        use Token::*;
        use Object::Int;

        match token {
            Plus => return Ok(Int(lhs + rhs)), 
            Minus => return Ok(Int(lhs - rhs)),
            Multiply => return Ok(Int(lhs * rhs)),
            Divide => return Ok(Int(lhs / rhs)),
            _ => return Err(InterpreterError::SomeError)
        }
    }

    fn float_expr(lhs: &f64, token: &Token, rhs: &f64) -> Result<Object, InterpreterError> {
        use Token::*;
        use Object::Float;

        match token {
            Plus => return Ok(Float(lhs + rhs)), 
            Minus => return Ok(Float(lhs - rhs)),
            Multiply => return Ok(Float(lhs * rhs)),
            Divide => return Ok(Float(lhs / rhs)),
            _ => return Err(InterpreterError::SomeError)
        }
    }

    fn visit_infix(lhs: &Object, token: &Token, rhs: &Object) -> Result<Object, InterpreterError> {
        use Object::*;

        match (lhs, rhs) {
            (Int(i1), Int(i2)) => Self::int_expr(i1, token, i2),
            (Float(f1), Float(f2)) => Self::float_expr(f1, token, f2),
            (_, _) => Err(InterpreterError::SomeError)
        }
    }

    fn visit_prefix(token: &Token, rhs: &Object) -> Result<Object, InterpreterError> {
        use Object::*;

        match token {
            Token::Minus => match rhs {
                Int(i) => return Ok(Int(-i)),
                Float(f) => return Ok(Float(-f)),
                _ => return Err(InterpreterError::SomeError)
            }

            _ => Err(InterpreterError::SomeError)
        }
    }

    pub fn visit_expr(&self, expr: &Expr) -> Result<Object, InterpreterError> {
        use Expr::*;

        match expr {
            NumLit(n) => Ok(Object::Int(*n)),
            FloatLit(f) => Ok(Object::Float(*f)),
            Noop => Ok(Object::Null),
            Var(ref var_id) => {
                if let Some(obj) = self.global_scope.get(var_id) {
                    return Ok(obj.clone())
                }
                Err(InterpreterError::SomeError)
            }

            Infix(ref lhs, ref token, ref rhs) => {
                let l = self.visit_expr(lhs)?;
                let r = self.visit_expr(rhs)?;
                Self::visit_infix(&l, token, &r)
            },

            Prefix(ref token, ref rhs) => {
                let r = self.visit_expr(rhs)?;
                Self::visit_prefix(token, &r)
            }
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<Object, InterpreterError> {
        use Stmt::*;

        match stmt {
            Expr(ref expr) => self.visit_expr(expr),
            Return(ref expr) => Ok(Object::Return(Box::new(self.visit_expr(expr)?))),
            Block(ref stmts) => { self.visit_block_stmt(stmts) }
            Let(ref var, _, ref expr) => {
                if let crate::ast::ast::Expr::Var(id) = var {
                    self.global_scope.insert(id.to_string(), self.visit_expr(expr)?);
                    return Ok(Object::Null);
                }
                Err(InterpreterError::SomeError)
            }
            Assign(ref var, ref expr) => {
                if let crate::ast::ast::Expr::Var(id) = var {
                    self.global_scope.insert(id.to_string(), self.visit_expr(expr)?);
                    return Ok(Object::Null);
                }
                Err(InterpreterError::SomeError)
            }
        }
    }

    fn visit_block_stmt(&mut self, block: &BlockStmt) -> Result<Object, InterpreterError> {
        let mut res = Object::Null;

        for stmt in block.statements.iter() {
            res = self.visit_stmt(stmt)?;
            if let Object::Return(_) = &res { break; }
        }

        Ok(res)
    }
}
