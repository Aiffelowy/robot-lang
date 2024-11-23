use std::collections::HashMap;
use crate::ast::ast::{BlockStmt, Expr, Stmt, Type};

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

enum SymbolError {}

#[derive(Clone, Debug)]
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
            "int".to_string() => Symbol::Type(Type { t: "int".to_string(), mutable: false }),
            "float".to_string() => Symbol::Type(Type { t: "float".to_string(), mutable: false }),
            "null".to_string() => Symbol::Type(Type{ t: "null", mutable: false })
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

    fn visit_block(&mut self, blk: &BlockStmt) {
        for stmt in &blk.statements { self.visit_stmt(stmt) }
    }

    fn visit_expr(&mut self, expr: &Expr) -> Result<Symbol, SymbolError> {
        use Expr::*;

        match expr {
            NumLit(_) => Ok(Symbol::Type(Type{t: "int", mutable: false})),
            FloatLit(_) => Ok(Symbol::Type(Type{t: "float", mutable: false})),
            Noop => (),

            Var(ref var_id) => {
                match self.symtable.lookup(var_id) {
                    None => println!("GOOD"),
                    _ => ()
                }
            }

            Infix(ref lhs, _, ref rhs) => { self.visit_expr(lhs); self.visit_expr(rhs) }
            Prefix(_, ref rhs) => { self.visit_expr(rhs) }
        }
    }

    fn visit_stmt(&mut self, statement: &Stmt) {
        match statement {
            Stmt::Expr(ref expr) => self.visit_expr(expr),
            Stmt::Block(ref blk) => self.visit_block(blk),
            Stmt::Let(ref lhs, ref t, ref rhs) => { 
                if let Expr::Var(name) = lhs {
                    match self.symtable.lookup(&t.to_string()) {
                        None => println!("yeah"),
                        _ => ()
                    }

                    self.symtable.define(Symbol::Var(name.clone(), Box::new(Symbol::Type(t.clone()))));
                }

                self.visit_expr(rhs)
            },
            Stmt::Return(ref expr) => self.visit_expr(expr),
            Stmt::Assign(ref lhs, _) => {
                if let Expr::Var(name) = lhs {
                    match self.symtable.lookup(&name) {
                        None => println!("GOOOOOOOD"),
                        Some(s) => match s {
                                Symbol::Var(_, s) => match **s {
                                    Symbol::Type(ref t) => if !t.mutable { println!("GOTEM"); }
                                    _ => unreachable!(),
                                }
                                _ => unreachable!()
                        }
                    }
                }
            }
        }
    }

    pub fn check(&mut self, ast: &BlockStmt) -> () {
        self.visit_block(ast);
    }
}
