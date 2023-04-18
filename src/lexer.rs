use crate::token::{Token, TokenType};

pub struct TokenIterator<'a> {
    source: &'a str,
    cursor: usize,
    tok_len: u32,
}

impl<'a> TokenIterator<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, cursor: 0, tok_len: 0 }
    }
    
    fn is_whitespace(char: &str) -> bool {
        char == " "
    } 

    fn consume_char(&mut self) -> Option<&'a str> {
        if self.cursor >= self.source.len() { return None; }
        let char = self.source.get(self.cursor..self.cursor+1);
        self.cursor += 1;
        self.tok_len += 1;
        return char;
    }

    fn peek_char(&self) -> Option<&'a str> {
        if self.cursor >= self.source.len() { return None; }
        let char = self.source.get(self.cursor..self.cursor+1);
        return char;
    }

    fn consume_comment(&mut self) -> Token<'a> {
        self.consume_char();
        while let Some(char) = self.consume_char() {
            if char == "-" {
                if let Some("-") = self.peek_char() {
                    self.consume_char();
                    return Token::new(TokenType::Comment, self.tok_len);
                }
            }
        }
        Token::new(TokenType::EOF, 0)
    }
    
    fn consume_string(&mut self) -> Token<'a> {
        let start = self.cursor;
        while let Some(char) = self.consume_char() {
            if char == "\"" {
                return Token::new(
                    TokenType::StringLiteral {
                        value: &self.source[start..self.cursor-1]
                    }, self.tok_len
                );
            }
        }
        Token::new(TokenType::EOF, 0)
    }

    fn consume_numeric(&mut self) -> Token<'a> {
        let start = self.cursor-1;
        while let Some(char) = self.consume_char() {
            if Self::is_whitespace(char) { break; }
        }
        Token::new(
            TokenType::NumericLiteral {
                value: self.source[start..self.cursor].parse().expect("Non-digit.")
            },
            self.tok_len
        )
    }

    fn consume_token(&mut self) -> Token<'a> {
        match self.consume_char() {
            Some(num) if ("0".."9").contains(&num)  => self.consume_numeric(),
            Some("\"") => self.consume_string(),
            Some("-") => match self.peek_char() {
                Some("-") => self.consume_comment(),
                _ => Token::new(TokenType::Operator { value: "-" }, 1),
            },
            Some(_) => Token::new(TokenType::Unknown, self.tok_len),
            None => Token::new(TokenType::EOF, 0),
        }
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tok_len = 0;
        return Some(self.consume_token());
    }
}

pub struct Lexer<'a> {
    pub token_stream: TokenIterator<'a>
}

impl<'a> Lexer<'a> {
    pub fn from_source(code: &'a str) -> Self {
        Self {
            token_stream: TokenIterator::new(code)
        }
    }
    pub fn update_source(&mut self, source: &'a str) {
        self.token_stream = TokenIterator::new(source)
    }
    pub fn get_token(&mut self) -> Token<'a> {
        self.token_stream.next().expect("Error: get_token()")
    }
}