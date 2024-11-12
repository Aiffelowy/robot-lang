use crate::ast::ast::{BlockStmt, Stmt, Expr};
use crate::token::Token;
use crate::errors::ParseError;
use crate::lexer::Lexer;

pub struct Parser {
    lexer: Lexer,
    current_token: Token
}

impl Parser {
    //create a new Interpreter instance
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            current_token: Token::EOF,
        }
    }

    pub fn feed_next_line(&mut self, text: String, append: bool) {
        self.lexer.feed_next_line(text, append);
        self.current_token = self.lexer.next_token().unwrap();
    }
    

    fn eat(&mut self, expected_token: Token) -> Result<(), ParseError> {
        if self.current_token == Token::EOF && expected_token != Token::EOF {
            return Err(ParseError::WaitForInput)
        }

        if self.current_token != expected_token {
            return Err(ParseError::WrongToken(self.lexer.pos, expected_token, self.current_token.clone()))
        }

        self.current_token = self.lexer.next_token()?;
        Ok(())
    }


    fn scope_statement(&mut self) -> Result<BlockStmt, ParseError> {
        self.eat(Token::LeftCurly)?;
        let nodes = self.statement_block()?;
        self.eat(Token::RightCurly)?;

        Ok(nodes)
    }

    fn statement_block(&mut self) -> Result<BlockStmt, ParseError> {
        let node = self.statement()?;
        let mut res :Vec<Stmt> = vec![node];

        while self.current_token == Token::Semicolon {
            self.eat(Token::Semicolon)?;
            res.push(self.statement()?);
        }

        if let Token::ID(_) = &self.current_token {
            return Err(ParseError::UnexpectedToken(self.lexer.pos, self.current_token.clone()))
        }

        return Ok(BlockStmt { statements: res });
    }

    fn empty() -> Expr {
        Expr::Noop
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        return match &self.current_token {
            Token::LeftCurly => Ok(Stmt::Block(self.statement_block()?)),
            Token::Let => self.assignment_statement(),
            Token::Ret =>  { self.eat(Token::Ret)?; Ok(Stmt::Return(*self.expr()?)) },
            _ => Ok(Stmt::Expr(Parser::empty()))
        }
    }

    fn assignment_statement(&mut self) -> Result<Stmt, ParseError> {
        self.eat(Token::Let)?;
        let left = self.variable()?;
        self.eat(Token::Equal)?;
        let right = self.expr()?;

        Ok(Stmt::Let(*left, *right))
    }

    fn variable(&mut self) -> Result<Box<Expr>, ParseError> {
        if let Token::ID(id) = &self.current_token {
            let id = id.clone().to_string();
            self.eat(Token::ID(id.clone()))?;
            return Ok(Box::new(Expr::Var(id)));
        }

        return Err(ParseError::UnknownToken(self.lexer.pos, 'y'));
    }
    
    //return INT token value  factor: INTEGER
    fn factor(&mut self) -> Result<Box<Expr>, ParseError> {
        use Token::*;

        let old_token = self.current_token.clone();

        if let Number(n) = old_token {
            self.eat(Number(n))?;
            return Ok(Box::new(Expr::NumLit(n)));
        }

        if let Float(f) = old_token {
            self.eat(Float(f))?;
            return Ok(Box::new(Expr::FloatLit(f)));
        }

        if let Minus = old_token {
            self.eat(Minus)?;
            return Ok(Box::new(Expr::Prefix(old_token, self.factor()?)))
        }

        if let LeftParen = old_token {
            self.eat(LeftParen)?;
            let res = self.expr()?;
            self.eat(RightParen)?;
            return Ok(res);
        }
        
        return self.variable()
    }

    fn term(&mut self) -> Result<Box<Expr>, ParseError> {
        use Token::*;

        let mut node = self.factor()?;

        while self.current_token.is(&[Multiply, Divide]) {
            let token = self.current_token.clone();
            self.eat(token.clone())?;
            node = Box::new(Expr::Infix(node, token, self.factor()?));
        }

        Ok(node)
    }

    //evaluates the expression
    fn expr(&mut self) -> Result<Box<Expr>, ParseError> {
        use Token::*;
        
        let mut node = self.term()?;
        while self.current_token.is(&[Plus, Minus]) {
            let token = self.current_token.clone();
            self.eat(token.clone())?;
            node = Box::new(Expr::Infix(node, token, self.factor()?));
        }

        Ok(node)
    }

    pub fn parse(&mut self) -> Result<BlockStmt, ParseError> {
        let program = self.statement_block()?;
        if self.current_token != Token::EOF { return Err(ParseError::WrongToken(self.lexer.pos, Token::EOF, self.current_token.clone())) }

        Ok(program)
    }
}
