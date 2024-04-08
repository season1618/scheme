use crate::lexer::Token;

use Token::*;
use Expr::*;

#[derive(Debug)]
pub enum Expr {
    Num(u32),
    Bool(bool),
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
            Token::Num(val) => Expr::Num(val),
            Token::Bool(val) => Expr::Bool(val),
        };
        Ok(expr)
    }

    fn next_token(&mut self) -> Result<Token, String> {
        if self.idx < self.tokens.len() {
            let token = self.tokens[self.idx];
            self.idx += 1;
            Ok(token)
        } else {
            Err(String::from("no token"))
        }
    }
}
