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

    // Public API
    pub fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_addition()?;

        while matches!(
            self.current_token.token_type,
            TokenType::Eq | TokenType::NotEq | TokenType::Gt | TokenType::Lt 
            | TokenType::GtEq | TokenType::LtEq
        ) {
            let op = match self.current_token.token_type {
                TokenType::Eq => Operator::Eq,
                TokenType::NotEq => Operator::NotEq,
                TokenType::Gt => Operator::Gt,
                TokenType::Lt => Operator::Lt,
                TokenType::GtEq => Operator::GtEq,
                TokenType::LtEq => Operator::LtEq,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_addition()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
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

        while matches!(self.current_token.token_type, TokenType::Star | TokenType::Slash) {
            let op = match self.current_token.token_type {
                TokenType::Star => Operator::Multiply,
                TokenType::Slash => Operator::Divide,
                _ => unreachable!(),
            };
            self.advance();

            let right = self.parse_primary()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                operator: op,
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
            TokenType::Ident(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expr::Identifier(name))
            }
            _ => Err(format!(
                "Expected number or identifier, found: {:?} at line {}",
                self.current_token.token_type,
                self.current_token.line
            )),
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        if let TokenType::Ident(name) = &self.current_token.token_type {
            let name = name.clone();
            self.advance();
            
            if matches!(self.current_token.token_type, TokenType::Assign) {
                self.advance(); // consume `=`
                let value = self.parse_expression()?;
                return Ok(Statement::Assign { name, value });
            } else {

                let expr = Expr::Identifier(name);
                return Ok(Statement::Expr(expr));
            }
        }
        
        let expr = self.parse_expression()?;
        Ok(Statement::Expr(expr))
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();

        while !matches!(self.current_token.token_type, TokenType::EOF) {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }

        Ok(Program { statements })
    }
}