use lang_lang::{token::Token, lexer::Lexer};

pub struct LexerTestGen {
    tests: Vec<(&'static str, Vec<Token<'static>>)>,
    lexer: Lexer<'static>
}

impl LexerTestGen {
    pub fn new(tests: Vec<(&'static str, Vec<Token<'static>>)>) -> Self {
        LexerTestGen { tests, lexer: Lexer::from_source("") }
    }

    pub fn run_tests(&mut self) {
        for (src, tokens) in self.tests.iter() {
            self.lexer.update_source(src);
            for expected in tokens {
                let got = &self.lexer.get_token();
                if got != expected {
                    panic!("\n\nSource = `{}`\nExpected: {}\nGot       {}\n\n", src, expected, got);
                }
            }
        };
    }
}