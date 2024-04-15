use crate::data::{Token, TopLevel, Defn, Expr, OprKind};

use Token::*;
use Expr::*;
use OprKind::*;

pub fn parse<'a>(tokens: Vec<Token<'a>>) -> Result<Vec<TopLevel>, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    idx: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser {
            tokens,
            idx: 0,
        }
    }

    fn parse(&mut self) -> Result<Vec<TopLevel>, String> {
        let mut vec = Vec::new();
        while self.idx < self.tokens.len() {
            vec.push(self.parse_toplevel()?);
        }
        Ok(vec)
    }

    fn parse_toplevel(&mut self) -> Result<TopLevel, String> {
        Ok(if let Ok(defn) = self.parse_defn() {
            TopLevel::Defn(defn)
        } else {
            let expr = self.parse_expr()?;
            TopLevel::Expr(expr)
        })
    }

    fn parse_defn(&mut self) -> Result<Defn, String> {
        if self.next_if(OpenParen) {
            if self.next_if(Keyword("define")) {
                let ident = self.next_ident()?;
                let expr = self.parse_expr()?;
                self.next_force(CloseParen)?;

                return Ok(Defn { ident, expr });
            }
            self.idx -= 1;
        }
        Err(String::from(""))
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let expr = match self.next_token()? {
            OpenParen => {
                match self.peek_token()? {
                    Keyword("lambda") => {
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
                    Keyword(s) if s == "let" || s == "let*" || s == "letrec" => {
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

                        match s {
                            "let" => Let { binds, expr: Box::new(expr) },
                            "let*" => LetStar { binds, expr: Box::new(expr) },
                            _ => LetRec { binds, expr: Box::new(expr) },
                        }
                    },
                    Keyword("set!") => {
                        self.next_token()?;

                        let ident = self.next_ident()?;
                        let expr = self.parse_expr()?;
                        self.next_force(CloseParen)?;

                        Set { ident, expr: Box::new(expr) }
                    },
                    CloseParen => Expr::Nil,
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
            Keyword(keyword) => {
                Opr(match &keyword as &str {
                    "=" => Eq,
                    "<" => Lt,
                    "<=" => Le,
                    ">" => Gt,
                    ">=" => Ge,
                    "+" => Add,
                    "-" => Sub,
                    "*" => Mul,
                    "/" => Div,
                    _ => return Err(format!("{:?} is not an operator", keyword)),
                })
            },
            Ident(ident) => Var(ident.to_string()),
            Token::Num(val) => Expr::Num(val),
            Token::Bool(val) => Expr::Bool(val),
            Token::Str(val) => Expr::Str(val),
            token => return Err(format!("unexpected token {:?}", token)),
        };
        Ok(expr)
    }

    fn next_if(&mut self, expected: Token<'a>) -> bool {
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

    fn next_force(&mut self, expected: Token<'a>) -> Result<(), String> {
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
            Ok(ident.to_string())
        } else {
            Err(String::from("expect identifier"))
        }
    }

    fn peek_token(&self) -> Result<Token<'a>, String> {
        if self.idx < self.tokens.len() {
            let token = self.tokens[self.idx].clone();
            Ok(token)
        } else {
            Err(String::from("no token"))
        }
    }

    fn next_token(&mut self) -> Result<Token<'a>, String> {
        if self.idx < self.tokens.len() {
            let token = self.tokens[self.idx].clone();
            self.idx += 1;
            Ok(token)
        } else {
            Err(String::from("no token"))
        }
    }
}
