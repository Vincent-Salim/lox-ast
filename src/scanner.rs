use crate::type::*;
use crate::token_type::*;
use crate::error::*;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner { source, tokens: Vec::new(), }
    }
    pub fn scan_tokens(&self) -> Result<&Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::eof());
        Ok(self.tokens)
    }
    
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len();
    }
    pub fn scan_token(&self) {

    }
}
