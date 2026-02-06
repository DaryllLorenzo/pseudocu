use crate::ast::{Expr, Operator, Program, Statement};
use crate::lexer::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    current_token: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Self {
            tokens,
            position: 0,
            current_token: Token {
                token_type: TokenType::EOF,
                literal: String::new(),
                line: 0,
            },
        };
        parser.advance();
        parser
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.current_token = self.tokens[self.position].clone();
            self.position += 1;
        } else {
            self.current_token = Token {
                token_type: TokenType::EOF,
                literal: String::new(),
                line: 0,
            };
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn consume(&mut self, expected_type: TokenType) -> Result<(), String> {
        if std::mem::discriminant(&self.current_token.token_type) 
           == std::mem::discriminant(&expected_type) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", 
                 expected_type, self.current_token.token_type))
        }
    }

    // Public API
    pub fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_addition()
    }

    // Private parsing methods with operator precedence
    fn parse_addition(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_multiplication()?;
        
        while matches!(
            self.current_token.token_type,
            TokenType::Plus | TokenType::Minus
        ) {
            let op = match self.current_token.token_type {
                TokenType::Plus => Operator::Plus,
                TokenType::Minus => Operator::Minus,
                _ => unreachable!(),
            };
            self.advance();
            
            let right = self.parse_multiplication()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn parse_multiplication(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        
        while matches!(self.current_token.token_type, TokenType::Star) {
            self.advance(); // Consume '*'
            
            let right = self.parse_primary()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                operator: Operator::Multiply,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match &self.current_token.token_type {
            TokenType::Number(value) => {
                let val = *value;
                self.advance();
                Ok(Expr::Number(val))
            }
            _ => Err(format!(
                "Expected number, found: {:?} at line {}", 
                self.current_token.token_type, 
                self.current_token.line
            )),
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        
        while !matches!(self.current_token.token_type, TokenType::EOF) {
            let expr = self.parse_expression()?;
            statements.push(Statement::Expr(expr));
            
            // Optional (for later) consume a semicolon if you add them later
            // if matches!(self.current_token.token_type, TokenType::Semicolon) {
            //     self.advance();
            // }
        }
        
        Ok(Program { statements })
    }
}