use crate::data::Token;

use Token::*;

pub fn tokenize(code: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(code);
    lexer.tokenize()
}

#[derive(Debug)]
struct Lexer<'a> {
    chs: &'a str,
}

impl<'a> Lexer<'a> {
    fn new(code: &'a str) -> Self {
        Lexer { chs: code }
    }

    fn tokenize(&mut self) -> Result<Vec<Token<'a>>, String> {
        let mut tokens = Vec::new();
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.next_char();
                continue;
            }

            if c == '"' {
                tokens.push(self.read_string()?);
                continue;
            }

            if self.next_if("(") {
                tokens.push(OpenParen);
                continue;
            }

            if self.next_if(")") {
                tokens.push(CloseParen);
                continue;
            }

            if self.next_if("'") {
                tokens.push(SingleQuote);
                continue;
            }

            if c.is_ascii_digit() {
                tokens.push(self.read_num());
                continue;
            }

            if is_ident_char(c) {
                tokens.push(self.read_keyword_ident()?);
                continue;
            }

            if self.next_if("#t") {
                tokens.push(Bool(true));
                continue;
            }

            if self.next_if("#f") {
                tokens.push(Bool(false));
                continue;
            }

            return Err(format!("invalid token '{}'", self.chs));
        }

        Ok(tokens)
    }

    fn read_keyword_ident(&mut self) -> Result<Token<'a>, String> {
        let mut chs = self.chs.char_indices();
        let prefix;
        (prefix, self.chs) = loop {
            match chs.next() {
                Some((_, c)) if is_ident_char(c) => continue,
                Some((i, _)) => break (&self.chs[..i], &self.chs[i..]),
                None => break (self.chs, &self.chs[self.chs.len()..]),
            }
        };

        if let Some(keyword) = ["define", "lambda", "let", "let*", "letrec", "set!", "quote", "begin", "if", "cond", "and", "or", "do"].iter().find(|&&keyword| keyword == prefix) {
            Ok(Keyword(keyword))
        } else if let Some(operator) = ["cons", "car", "cdr", "set-car!", "set-cdr!", "not", "eq?", "neq?", "equal?", "list?", "pair?", "procedure?", "symbol?", "number?", "boolean?", "string?", "null?", "procedure?", "list", "length", "memq", "last", "append", "=", "<=", "<", ">=", ">", "+", "-", "*", "/", "string-append", "symbol->string", "string->symbol", "number->string", "string->number"].iter().find(|&&operator| operator == prefix) {
            Ok(Operator(operator))
        } else {
            Ok(Ident(prefix))
        }
    }

    fn read_string(&mut self) -> Result<Token<'a>, String> {
        self.next_char();

        let mut s = String::new();
        while let Some(c) = self.next_char() {
            match c {
                '"' => return Ok(Str(s)),
                '\\' => s.push(self.next_char().unwrap()),
                c => s.push(c),
            }
        }
        Err(String::from("expect '\"'"))
    }

    fn read_num(&mut self) -> Token<'a> {
        let mut val = 0;
        while let Some(c) = self.peek_char() {
            if let Some(d) = c.to_digit(10) {
                val = 10 * val + d;
                self.next_char();
            } else {
                break;
            }
        }
        Num(val as f32)
    }

    fn next_if(&mut self, expected: &str) -> bool {
        if let Some(rest) = self.chs.strip_prefix(expected) {
            self.chs = rest;
            true
        } else {
            false
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.chs.chars().next()
    }

    fn next_char(&mut self) -> Option<char> {
        let mut chs = self.chs.chars();
        if let Some(c) = chs.next() {
            self.chs = chs.as_str();
            Some(c)
        } else {
            None
        }
    }
}

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || "!$%&*+-./<=>?@^_".contains(c)
}
