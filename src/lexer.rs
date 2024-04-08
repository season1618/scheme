use Token::*;

#[derive(Debug, Clone)]
pub enum Token {
    Num(u32),
    Bool(bool),
    Str(String),
}

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

    fn tokenize(&mut self) -> Result<Vec<Token>, String> {
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

            if c.is_alphanumeric() {
                tokens.push(self.read_num());
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

    fn read_string(&mut self) -> Result<Token, String> {
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

    fn read_num(&mut self) -> Token {
        let mut val = 0;
        while let Some(c) = self.peek_char() {
            if let Some(d) = c.to_digit(10) {
                val = 10 * val + d;
                self.next_char();
            } else {
                break;
            }
        }
        Num(val)
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
