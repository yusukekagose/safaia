// parser.rs
use crate::ast::{Expr, BinaryOperator, LiteralValue};
use crate::tokenizer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    // パーサーを初期化する関数
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    // 式をパースする関数
    fn parse_function_call(&mut self, function_name: String) -> Result<Expr, String> {
        if let Some(Token::OpenParenthesis) = self.tokens.get(self.position) {
            self.position += 1;

            let mut arguments = Vec::new();
            while let Ok(expr) = self.parse_additive_expr() {
                arguments.push(expr);
                if let Some(Token::Comma) = self.tokens.get(self.position) {
                    self.position += 1;
                } else {
                    break;
                }
            }

            if let Some(Token::CloseParenthesis) = self.tokens.get(self.position) {
                self.position += 1;
                Ok(Expr::FunctionCall(function_name, arguments))
            } else {
                Err("Expected ')'".to_string())
            }
        } else {
            Err("Expected '('".to_string())
        }
    }

    fn parse_open_brace(&mut self) -> Result<(), String> {
        if let Some(Token::OpenBrace) = self.tokens.get(self.position) {
            self.position += 1;
            Ok(())
        } else {
            Err("Expected '{'".to_string())
        }
    }

    fn parse_close_brace(&mut self) -> Result<(), String> {
        if let Some(Token::CloseBrace) = self.tokens.get(self.position) {
            self.position += 1;
            Ok(())
        } else {
            Err("Expected '}'".to_string())
        }
    }


    // 関数定義をパースする関数
    fn parse_function_definition(&mut self) -> Result<Expr, String> {
        if let Some(Token::DefKeyword) = self.tokens.get(self.position) {
            self.position += 1;

            if let Some(Token::Identifier(function_name)) = self.tokens.get(self.position) {
                let function_name = function_name.clone();
                self.position += 1;

                if let Some(Token::OpenParenthesis) = self.tokens.get(self.position) {
                    self.position += 1;

                    let mut parameters = Vec::new();
                    while let Some(Token::Identifier(param)) = self.tokens.get(self.position) {
                        parameters.push(param.clone());
                        self.position += 1;

                        if let Some(Token::Comma) = self.tokens.get(self.position) {
                            self.position += 1;
                        } else {
                            break;
                        }
                    }

                    if let Some(Token::CloseParenthesis) = self.tokens.get(self.position) {
                        self.position += 1;

                        self.parse_open_brace()?; // Add this line
                        let body = self.parse()?;
                        self.parse_close_brace()?; // Add this line

                        Ok(Expr::FunctionDefinition(
                            function_name.clone(),
                            parameters,
                            Box::new(body),
                        ))
                    } else {
                        Err("Expected ')'".to_string())
                    }
                } else {
                    Err("Expected '('".to_string())
                }
            } else {
                Err("Expected function name".to_string())
            }
        } else {
            Err("Expected 'def' keyword".to_string())
        }
    }

 
    // 数値や括弧、変数で囲まれた式をパースする関数
    fn parse_primary_expr(&mut self) -> Result<Expr, String> {
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


    // 乗算・除算の式をパースする関数
    fn parse_multiplicative_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_primary_expr()?;

        while let Some(token) = self.tokens.get(self.position) {
            let op = match token {
                Token::Multiply => BinaryOperator::Mul,
                Token::Divide => BinaryOperator::Div,
                _ => break,
            };
            self.position += 1;

            let right = self.parse_primary_expr()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }

        Ok(left)
    }

    // 加算・減算の式をパースする関数
    fn parse_additive_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative_expr()?;

        while let Some(token) = self.tokens.get(self.position) {
            let op = match token {
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Sub,
                _ => break,
            };
            self.position += 1;

            let right = self.parse_multiplicative_expr()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }

        Ok(left)
    }

    // パースを開始する関数
    pub fn parse(&mut self) -> Result<Expr, String> {
        let result = self.parse_additive_expr();

        if self.position < self.tokens.len() {
            Err(format!(
                "Unexpected token at position {}: {:?}",
                self.position,
                self.tokens.get(self.position)
            ))
        } else {
            result
        }
    }
}
