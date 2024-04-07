use Token::*;

#[derive(Debug)]
pub enum Token {
    Num(u32),
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
            if c.is_alphanumeric() {
                tokens.push(self.read_num());
                continue;
            }

            return Err(format!("invalid token '{}'", self.chs));
        }

        Ok(tokens)
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
