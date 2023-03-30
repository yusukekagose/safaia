use super::*;

impl Parser{
    pub fn parse_function_definition(&mut self) -> Result<Expr, String> {
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
                        let body = self.parse_additive_expr()?;
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

    pub fn parse_function_call(&mut self, function_name: String) -> Result<Expr, String> {
        if let Some(Token::OpenParenthesis) = self.tokens.get(self.position) {
            self.position += 1;

            let mut arguments = Vec::new();
            while let Ok(expr) = Parser::parse_additive_expr(self) {
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
}