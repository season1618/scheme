use crate::lexer::Token;

use Token::*;
use Expr::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Apply { proc: Box<Expr>, args: Vec<Expr> },
    Lambda { params: Vec<String>, expr: Box<Expr> },
    Let { binds: Vec<(String, Expr)>, expr: Box<Expr> },
    Var(String),
    Num(u32),
    Bool(bool),
    Str(String),
}

pub fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
    let mut parser = Parser::new(tokens);
    parser.parse_expr()
}

struct Parser {
    tokens: Vec<Token>,
    idx: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            idx: 0,
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let expr = match self.next_token()? {
            OpenParen => {
                match self.peek_token()? {
                    Keyword(s) if s == "lambda" => {
                        self.next_token()?;

                        let mut params = Vec::new();
                        self.next_force(OpenParen)?;
                        while let Ok(ident) = self.next_ident() {
                            params.push(ident);
                        }
                        self.next_force(CloseParen)?;

                        let expr = self.parse_expr()?;

                        self.next_force(CloseParen)?;

                        Lambda { params, expr: Box::new(expr) }
                    },
                    Keyword(s) if s == "let" => {
                        self.next_token()?;

                        let mut binds = Vec::new();
                        self.next_force(OpenParen)?;
                        while self.next_if(OpenParen) {
                            let ident = self.next_ident()?;
                            let expr = self.parse_expr()?;
                            binds.push((ident, expr));
                            self.next_force(CloseParen)?;
                        }
                        self.next_force(CloseParen)?;

                        let expr = self.parse_expr()?;

                        self.next_force(CloseParen)?;

                        Let { binds, expr: Box::new(expr) }
                    },
                    _ => {
                        let proc = self.parse_expr()?;
                        let mut args = Vec::new();
                        while !self.next_if(CloseParen) {
                            args.push(self.parse_expr()?);
                        }

                        Apply { proc: Box::new(proc), args }
                    },
                }
            },
            Ident(ident) => Var(ident),
            Token::Num(val) => Expr::Num(val),
            Token::Bool(val) => Expr::Bool(val),
            Token::Str(val) => Expr::Str(val),
            token => return Err(format!("unexpected token {:?}", token)),
        };
        Ok(expr)
    }

    fn next_if(&mut self, expected: Token) -> bool {
        if self.idx < self.tokens.len() {
            if self.tokens[self.idx] == expected {
                self.idx += 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn next_force(&mut self, expected: Token) -> Result<(), String> {
        let actual = self.next_token()?;
        if expected == actual {
            Ok(())
        } else {
            Err(format!("expect {:?}, actual {:?}",  expected, actual))
        }
    }

    fn next_ident(&mut self) -> Result<String, String> {
        if let Ident(ident) = self.peek_token()? {
            self.idx += 1;
            Ok(ident)
        } else {
            Err(String::from("expect identifier"))
        }
    }

    fn peek_token(&self) -> Result<Token, String> {
        if self.idx < self.tokens.len() {
            let token = self.tokens[self.idx].clone();
            Ok(token)
        } else {
            Err(String::from("no token"))
        }
    }

    fn next_token(&mut self) -> Result<Token, String> {
        if self.idx < self.tokens.len() {
            let token = self.tokens[self.idx].clone();
            self.idx += 1;
            Ok(token)
        } else {
            Err(String::from("no token"))
        }
    }
}