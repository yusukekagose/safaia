// ast.rs
#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(LiteralValue),
    BinaryOp(Box<Expr>, BinaryOperator, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
pub enum LiteralValue {
    Number(f64),
}