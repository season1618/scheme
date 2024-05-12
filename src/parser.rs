use crate::data::{Token, TopLevel, Body, Defn, Expr, Value};

use std::rc::Rc;
use std::cell::RefCell;

use Token::*;
use Expr::*;
use Value::*;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<TopLevel>, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

struct Parser {
    tokens: Vec<Token>,
    idx: usize,
}

impl<'a> Parser {
    fn new(tokens: Vec<Token>) -> Self {
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

    fn parse_body(&mut self) -> Result<Body, String> {
        let mut defns = Vec::new();
        let mut exprs = Vec::new();
        while let Ok(defn) = self.parse_defn() {
            defns.push(defn);
        }
        while let Ok(expr) = self.parse_expr() {
            exprs.push(expr);
        }
        self.idx -= 1;

        Ok(Body { defns, exprs })
    }

    fn parse_defn(&mut self) -> Result<Defn, String> {
        if self.next_if(OpenParen) {
            if self.next_if(Keyword("define")) {
                if self.next_if(OpenParen) {
                    let ident = self.next_ident()?;

                    let mut params = Vec::new();
                    while !self.next_if(CloseParen) {
                        let param = self.next_ident()?;
                        params.push(param);
                    }

                    let body = self.parse_body()?;

                    self.next_force(CloseParen)?;

                    return Ok(Defn { ident, expr: Lambda { params, body }})
                } else {
                    let ident = self.next_ident()?;
                    let expr = self.parse_expr()?;
                    self.next_force(CloseParen)?;

                    return Ok(Defn { ident, expr });
                }
            }
            self.idx -= 1;
        }
        Err(String::from(""))
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        match self.next_token()? {
            OpenParen => {
                let expr = self.parse_apply()?;
                self.next_force(CloseParen)?;
                Ok(expr)
            },
            SingleQuote => Ok(Quote(Box::new(self.parse_s_expr()?))),
            Operator(operator) => Ok(Opr(operator)),
            Token::Ident(ident) => Ok(Var(ident)),
            Token::Num(val) => Ok(Expr::Num(val)),
            Token::Bool(val) => Ok(Expr::Bool(val)),
            Token::Str(val) => Ok(Expr::Str(val)),
            token => Err(format!("unexpected token {:?}", token)),
        }
    }

    fn parse_apply(&mut self) -> Result<Expr, String> {
        if let Ok(keyword) = self.next_keyword() {
            match keyword {
                "lambda" => {
                    let mut params = Vec::new();
                    self.next_force(OpenParen)?;
                    while let Ok(ident) = self.next_ident() {
                        params.push(ident);
                    }
                    self.next_force(CloseParen)?;

                    let body = self.parse_body()?;

                    Ok(Lambda { params, body })
                },
                s if s == "let" || s == "let*" || s == "letrec" => {
                    let mut binds = Vec::new();
                    self.next_force(OpenParen)?;
                    while self.next_if(OpenParen) {
                        let ident = self.next_ident()?;
                        let expr = self.parse_expr()?;
                        binds.push((ident, expr));
                        self.next_force(CloseParen)?;
                    }
                    self.next_force(CloseParen)?;

                    let body = self.parse_body()?;

                    Ok(match s {
                        "let" => Let { binds, body },
                        "let*" => LetStar { binds, body },
                        _ => LetRec { binds, body },
                    })
                },
                "set!" => {
                    let ident = self.next_ident()?;
                    let expr = self.parse_expr()?;

                    Ok(Set { ident, expr: Box::new(expr) })
                },
                "quote" => {
                    Ok(Quote(Box::new(self.parse_s_expr()?)))
                },
                "begin" => {
                    let mut exprs = Vec::new();
                    while !self.peek_if(CloseParen) {
                        exprs.push(self.parse_expr()?);
                    }
                    Ok(Begin(exprs))
                },
                "if" => {
                    let cond = self.parse_expr()?;
                    let expr1 = self.parse_expr()?;
                    let expr2 = self.parse_expr()?;

                    Ok(If { cond: Box::new(cond), expr1: Box::new(expr1), expr2: Box::new(expr2) })
                },
                "cond" => {
                    let mut cond_then = Vec::new();
                    while self.next_if(OpenParen) {
                        let cond = self.parse_expr()?;
                        let then = self.parse_expr()?;
                        cond_then.push((cond, then));
                        self.next_force(CloseParen)?;
                    }
                    
                    Ok(Cond { cond_then })
                },
                "and" => {
                    let mut args = Vec::new();
                    while !self.peek_if(CloseParen) {
                        args.push(self.parse_expr()?);
                    }

                    Ok(And { args })
                },
                "or" => {
                    let mut args = Vec::new();
                    while !self.peek_if(CloseParen) {
                        args.push(self.parse_expr()?);
                    }

                    Ok(Or { args })
                },
                "do" => {
                    let mut binds = Vec::new();
                    self.next_force(OpenParen)?;
                    while self.next_if(OpenParen) {
                        let ident = self.next_ident()?;
                        let init = self.parse_expr()?;
                        let update = self.parse_expr()?;
                        binds.push((ident, init, update));
                        self.next_force(CloseParen)?;
                    }
                    self.next_force(CloseParen)?;

                    self.next_force(OpenParen)?;
                    let test = self.parse_expr()?;
                    let mut exprs = Vec::new();
                    loop {
                        let expr = self.parse_expr()?;
                        exprs.push(expr);
                        if self.next_if(CloseParen) {
                            break;
                        }
                    }

                    let body = self.parse_expr()?;

                    Ok(Do { binds, test: Box::new(test), exprs, body: Box::new(body) })
                }
                _ => Err(format!("{} is unavailable", keyword)),
            }
        } else if self.peek_if(CloseParen) {
            Ok(Expr::Nil)
        } else {
            let proc = self.parse_expr()?;
            let mut args = Vec::new();
            while !self.peek_if(CloseParen) {
                args.push(self.parse_expr()?);
            }

            Ok(Apply { proc: Box::new(proc), args })
        }
    }

    fn parse_s_expr(&mut self) -> Result<Value, String> {
        match self.next_token()? {
            OpenParen => Ok(self.parse_list()?),
            Keyword(keyword) => Ok(Symbol(keyword.to_string())),
            Operator(operator) => Ok(Symbol(operator.to_string())),
            Token::Ident(ident) => Ok(Symbol(ident.to_string())),
            Token::Num(val) => Ok(Value::Num(val)),
            Token::Bool(val) => Ok(Value::Bool(val)),
            Token::Str(val) => Ok(Value::Str(Rc::new(val))),
            token => return Err(format!("unexpected token {:?}", token)),
        }
    }

    fn parse_list(&mut self) -> Result<Value, String> {
        Ok(if self.next_if(CloseParen) {
            Value::Nil
        } else {
            Pair(Rc::new(RefCell::new((self.parse_s_expr()?, self.parse_list()?))))
        })
    }

    fn peek_if(&self, expected: Token) -> bool {
        self.idx < self.tokens.len() && self.tokens[self.idx] == expected
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

    fn next_keyword(&mut self) -> Result<&'a str, String> {
        if let Keyword(keyword) = self.peek_token()? {
            self.idx += 1;
            Ok(keyword)
        } else {
            Err(String::from("expect keyword"))
        }
    }

    fn next_ident(&mut self) -> Result<String, String> {
        if let Token::Ident(ident) = self.peek_token()? {
            self.idx += 1;
            Ok(ident.to_string())
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
