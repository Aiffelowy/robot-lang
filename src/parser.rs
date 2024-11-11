use crate::ast::ast::Expr;
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


    fn scope_statement(&mut self) -> Result<Box<Expr>, ParseError> {
        self.eat(Token::LeftCurly)?;
        let nodes = self.statement_block()?;
        self.eat(Token::RightCurly)?;

        Ok(nodes)
    }

    fn statement_block(&mut self) -> Result<Box<Expr>, ParseError> {
        let node = self.statement()?;
        let mut res :Vec<Box<Expr>> = vec![node];

        while self.current_token == Token::Semicolon {
            self.eat(Token::Semicolon)?;
            res.push(self.statement()?);
        }

        if let Token::ID(_) = &self.current_token {
            return Err(ParseError::UnexpectedToken(self.lexer.pos, self.current_token.clone()))
        }

        return Ok(Box::new(Expr::Scope(res)));
    }

    fn empty() -> Box<Expr> {
        Box::new(Expr::NoOp)
    }

    fn statement(&mut self) -> Result<Box<Expr>, ParseError> {
        return match &self.current_token {
            Token::LeftCurly => self.scope_statement(),
            Token::Let => self.assignment_statement(),
            Token::LeftParen => self.factor(),
            _ => Ok(Parser::empty())
        }
    }

    fn assignment_statement(&mut self) -> Result<Box<Expr>, ParseError> {
        self.eat(Token::Let)?;
        let left = self.variable()?;
        self.eat(Token::Equal)?;
        let right = self.expr()?;

        Ok(Box::new(Expr::Assign(left, right)))
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

        if let Minus = old_token {
            self.eat(Minus)?;
            return Ok(Box::new(Expr::UnaryMinus(self.factor()?)))
        }

        if let Plus = old_token {
            self.eat(Plus)?;
            return Ok(Box::new(Expr::UnaryPlus(self.factor()?)))
        }

        if let LeftParen = old_token {
            self.eat(LeftParen)?;
            let res = self.expr()?;
            self.eat(RightParen)?;
            return Ok(res);
        }

        if let LeftCurly = old_token {
            //self.eat(LeftCurly)?;
            let res = self.scope_statement()?;
            //self.eat(RightCurly)?;
            return Ok(res);
        }
        
        return self.variable()
    }

    fn term(&mut self) -> Result<Box<Expr>, ParseError> {
        use Token::*;

        let mut node = self.factor()?;

        while self.current_token.is(&[Multiply, Divide]) {
            let token = &self.current_token;

            if let Multiply = token {
                self.eat(Multiply)?;
                node = Box::new(Expr::Mult(node, self.factor()?));
            } else if let Divide = token {
                self.eat(Divide)?;
                node = Box::new(Expr::Div(node, self.factor()?));
            }
        }

        Ok(node)
    }

    //evaluates the expression
    fn expr(&mut self) -> Result<Box<Expr>, ParseError> {
        use Token::*;
        
        let mut node = self.term()?;
        while self.current_token.is(&[Plus, Minus]) {
            let token = &self.current_token;

            if let Plus = token {
                self.eat(Plus)?;
                node = Box::new(Expr::Add(node, self.term()?))
            } else if let Minus = token {
                self.eat(Minus)?;
                node = Box::new(Expr::Sub(node, self.term()?))
            }
        }

        Ok(node)
    }

    pub fn parse(&mut self) -> Result<Box<Expr>, ParseError> {
        let program = self.scope_statement()?;
        if self.current_token != Token::EOF { return Err(ParseError::InternalError) }

        Ok(program)
    }
}
