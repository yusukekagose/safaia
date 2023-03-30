use crate::ast::{Expr, LiteralValue};
use crate::tokenizer::Token;
use crate::parser::Parser;

impl Parser {
    // 数値や括弧、変数で囲まれた式をパースする関数
    pub fn parse_primary_expr(&mut self) -> Result<Expr, String> {
        match self.tokens.get(self.position) {
            Some(Token::Identifier(name)) => {
                self.position += 1;
                // 次のトークンが '(' の場合、関数呼び出しとしてパース
                if let Some(Token::OpenParenthesis) = self.tokens.get(self.position) {
                    self.parse_function_call(name.clone())
                } else {
                    // 関数呼び出しでない場合、単なる識別子として扱う
                    Ok(Expr::Variable(name.clone()))
                }
            },
            Some(Token::DefKeyword) => {
                // 関数定義をパース
                self.parse_function_definition()
            },
            Some(Token::Number(n)) => {
                self.position += 1;
                Ok(Expr::Literal(LiteralValue::Number(n.parse().unwrap())))
            }
            Some(Token::OpenParenthesis) => {
                self.position += 1;
                let expr = self.parse_additive_expr()?;
                if let Some(Token::CloseParenthesis) = self.tokens.get(self.position) {
                    self.position += 1;
                    Ok(expr)
                } else {
                    Err("Expected ')'".to_string())
                }
            }
            _ => Err("Expected a primary expression".to_string()),
        }
    }
}