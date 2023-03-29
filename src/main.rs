mod tokenizer;
mod parser;

use tokenizer::tokenize;
use parser::Parser;

fn main() {
    let input = "fn test_function(x, y) { x + 42 * y }";
    let (_, tokens) = tokenize(input).unwrap();
    println!("Tokens: {:?}", tokens);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expr();
    println!("AST: {:?}", ast);
}