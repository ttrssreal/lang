use std::vec;

use lang_lang::token::{Token, TokenType};

mod util;
use util::lexer_util::LexerTestGen;

#[test]
fn comments() {
    let t_type = TokenType::Comment;
    let mut lexer_test_gen = LexerTestGen::new(vec![
        ("----",                        vec![ Token { t_type, len: 4 } ]),
        ("-- this is a comment --",     vec![ Token { t_type, len: 23 } ]),
        ("--this is also a comment--",  vec![ Token { t_type, len: 26 } ]),
        ("--!--",                       vec![ Token { t_type, len: 5 } ]),
        ("--com----ment--",             vec![ Token { t_type, len: 7 },
                                              Token { t_type, len: 8 } ]),
        ("------",                      vec![ Token { t_type, len: 4 } ]),
        ("-o---wow--",                  vec![ Token { t_type: TokenType::Operator { value: "-" }, len: 1 },
                                              Token { t_type: TokenType::Unknown, len: 1 },
                                              Token { t_type: TokenType::Comment, len: 8 } ]),
    ]);
    lexer_test_gen.run_tests();
}

#[test]
fn literals() {
    let mut lexer_test_gen = LexerTestGen::new(vec![
        (r#""""#,                       vec![ Token { t_type: TokenType::StringLiteral { value: "" }, len: 2 } ]),
        (r#""this is a string""#,       vec![ Token { t_type: TokenType::StringLiteral { value: "this is a string" }, len: 18 } ]),
        (r#""this is brkn strin"g""#,   vec![ Token { t_type: TokenType::StringLiteral { value: "this is brkn strin" }, len: 20 },
                                              Token { t_type: TokenType::Unknown, len: 1 }]),

        ("69420",                       vec![ Token { t_type: TokenType::NumericLiteral { value: 69420 }, len: 5 } ]),
        ("6942000000000000000",         vec![ Token { t_type: TokenType::NumericLiteral { value: 6942000000000000000 }, len: 19 } ]),
        ("142",         vec![ Token { t_type: TokenType::NumericLiteral { value: 142 }, len: 3 } ]),

    ]);
    lexer_test_gen.run_tests();
}