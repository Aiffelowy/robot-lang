use std::collections::HashMap;
use crate::ast::ast::{BlockStmt, Expr, Stmt, Type};
use crate::token::Token;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    }
);

#[derive(Debug)]
pub enum SymbolError {
    SomeError
}

#[derive(Clone, Debug, PartialEq)]
enum Symbol {
    Type(Type),
    Var(String, Box<Symbol>),
    Unknown
}


struct SymbolTable {
    table: HashMap<String, Symbol>
}

impl SymbolTable {

    pub fn new() -> Self {
        Self { table: map!(
            "int".to_string() => Symbol::Type(Type::new(Token::IntType, false)),
            "float".to_string() => Symbol::Type(Type::new(Token::FloatType, false)),
            "null".to_string() => Symbol::Type(Type::new(Token::Null, false))
        ) }
    }

    pub fn define(&mut self, symbol: Symbol) {
        println!("define: {:?}", symbol);
        match &symbol {
            Symbol::Type(t) => self.table.insert(t.to_string(), symbol),
            Symbol::Var(name, _) => self.table.insert(name.to_string(), symbol),
            Symbol::Unknown => panic!()
        };
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.table.get(name)
    }
}

pub struct SymbolTableBuilder {
    symtable: SymbolTable
}

impl SymbolTableBuilder {
    pub fn new() -> Self {
        Self { symtable: SymbolTable::new() }
    }

    fn visit_block(&mut self, blk: &BlockStmt) -> Result<(), SymbolError> {
        for stmt in &blk.statements { self.visit_stmt(stmt)? }
        Ok(())
    }

    fn visit_infix(lhs: &Symbol, rhs: &Symbol) -> Result<Symbol, SymbolError> {
        if lhs == rhs { return Ok(lhs.clone()) }
        Err(SymbolError::SomeError)
    }

    fn visit_prefix(rhs: &Symbol) -> Result<Symbol, SymbolError> {
        Ok(rhs.clone())
    }

    fn visit_expr(&mut self, expr: &Expr) -> Result<Symbol, SymbolError> {
        use Expr::*;

        match expr {
            NumLit(_) => Ok(Symbol::Type(Type::new(Token::IntType, false))),
            FloatLit(_) => Ok(Symbol::Type(Type::new(Token::FloatType, false))),
            Noop => Ok(Symbol::Type(Type::new(Token::Null, false))),

            Var(ref var_id) => {
                match self.symtable.lookup(var_id) {
                    None => Err(SymbolError::SomeError),
                    Some(s) => Ok(s.clone())
                }
            }

            Infix(ref lhs, _, ref rhs) => { 
                let l = self.visit_expr(lhs)?;
                let r = self.visit_expr(rhs)?;
                Self::visit_infix(&l, &r)
            }
            Prefix(_, ref rhs) => {
                let r = self.visit_expr(rhs)?;
                Self::visit_prefix(&r)
            }
        }
    }

    fn visit_stmt(&mut self, statement: &Stmt) -> Result<(), SymbolError> {
        match statement {
            Stmt::Expr(ref expr) => { self.visit_expr(expr)?; Ok(()) },
            Stmt::Block(ref blk) => self.visit_block(blk),
            Stmt::Let(ref lhs, ref t, ref rhs) => { 
                if let Expr::Var(name) = lhs {
                    let var_symbol = match self.symtable.lookup(&t.to_string()) {
                        None => return Err(SymbolError::SomeError),
                        Some(s) => s.clone()
                    };

                    let expr_symbol = self.visit_expr(rhs)?;
                    if var_symbol != expr_symbol { return Err(SymbolError::SomeError) }
                    self.symtable.define(Symbol::Var(name.clone(), Box::new(Symbol::Type(t.clone()))));
                }

                Ok(())
            },
            Stmt::Return(ref expr) => { self.visit_expr(expr)?; Ok(()) },
            Stmt::Assign(ref lhs, ref rhs) => {
                if let Expr::Var(name) = lhs {
                    let var_symbol = match self.symtable.lookup(&name) {
                        None => return Err(SymbolError::SomeError),
                        Some(s) => match s {
                                Symbol::Var(_, s) => *s.clone(),
                                _ => unreachable!()
                        }
                    };
                    
                    if let Symbol::Type(ref t) = var_symbol {
                        if !t.mutable { return Err(SymbolError::SomeError) }
                    }

                    let expr_symbol = self.visit_expr(rhs)?;
                    if var_symbol != expr_symbol { return Err(SymbolError::SomeError) }
                }

                Ok(())
            }
        }
    }

    pub fn check(&mut self, ast: &BlockStmt) -> Result<(), SymbolError> {
        self.visit_block(ast)?;
        Ok(())
    }
}
