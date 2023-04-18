use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub t_type: TokenType<'a>,
    pub len: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType<'a> {
    None,
    NumericLiteral { value: i64 },
    StringLiteral { value: &'a str },
    Bool { value: bool },
    Identifier { value: &'a str },
    Keyword { value: &'a str },
    Whitespace { value: &'a str },
    Comment,
    StmtBlockOpen,
    StmtBlockClose,
    ParenOpen,
    ParenClose,
    Punctuator,
    Operator { value: &'a str },
    AssignmentOperator { value: &'a str },
    EOF,
    Unknown,
}

impl<'a> Token<'a> {
    pub fn new(t_type: TokenType<'a>, len: u32) -> Self {
        Token { t_type, len }
    }
    pub fn len(&self) -> u32 { self.len }
    pub fn is_eof(&self) -> bool { self.t_type == TokenType::EOF }
    pub fn t_type(&self) -> TokenType<'a> { self.t_type }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token<{:?}>[{}]", self.t_type, self.len)
    }
}