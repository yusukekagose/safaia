// ast.rs
#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(LiteralValue),
    Variable(String), // New variant for variables
    Binary(Box<Expr>, BinaryOperator, Box<Expr>), // BinaryOpをBinaryに変更
    FunctionDefinition(String, Vec<String>, Box<Expr>),
    FunctionCall(String, Vec<Expr>),
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
