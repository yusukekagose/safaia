pub mod expr;
pub mod function;
pub mod primary;

use crate::ast::{Expr, BinaryOperator};
use crate::tokenizer::Token;

// ... (rest of the code) ...


pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn parse_open_brace(&mut self) -> Result<(), String> {
        self.expect_token(Token::OpenBrace)
    }

    pub fn parse_close_brace(&mut self) -> Result<(), String> {
        self.expect_token(Token::CloseBrace)
    }

    pub fn expect_token(&mut self, expected: Token) -> Result<(), String> {
        if let Some(token) = self.tokens.get(self.position) {
            if *token == expected {
                self.position += 1;
                Ok(())
            } else {
                Err(format!("Expected {:?}, got {:?}", expected, token))
            }
        } else {
            Err(format!("Expected {:?}, but reached the end of the input", expected))
        }
    }
    // パーサーを初期化する関数
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        let result = self.parse_additive_expr();
        if self.position < self.tokens.len() {
            Err("Unexpected tokens remaining".to_string())
        } else {
            result
        }
    }
}
