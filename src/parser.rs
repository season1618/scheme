use crate::lexer::Token;

use Token::*;
use Expr::*;

#[derive(Debug)]
pub enum Expr {
    Let { binds: Vec<(String, Expr)>, expr: Box<Expr> },
    Ident(String),
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
                match self.next_token()? {
                    Keyword(s) if s == "let" => {
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

                        Let { binds, expr: Box::new(expr) }
                    },
                    token => return Err(format!("unexpected token {:?}", token)),
                }
            },
            Token::Ident(ident) => Expr::Ident(ident),
            Token::Num(val) => Expr::Num(val),
            Token::Bool(val) => Expr::Bool(val),
            Token::Str(val) => Expr::Str(val),
            _ => panic!(),
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
        if let Token::Ident(ident) = self.next_token()? {
            Ok(ident)
        } else {
            Err(String::from("expect identifier"))
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
