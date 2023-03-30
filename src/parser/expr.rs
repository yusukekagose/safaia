use super::*;

impl Parser {
    pub fn parse_additive_expr(&mut self) -> Result<Expr, String> {
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

    fn parse_multiplicative_expr(&mut self) -> Result<Expr, String> {
        let mut left = Parser::parse_primary_expr(self)?;

        while let Some(token) = self.tokens.get(self.position) {
            let op = match token {
                Token::Multiply => BinaryOperator::Mul,
                Token::Divide => BinaryOperator::Div,
                _ => break,
            };
            self.position += 1;

            let right = Parser::parse_primary_expr(self)?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }

        Ok(left)
    }
}
