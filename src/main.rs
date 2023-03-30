mod ast;
mod parser;
mod tokenizer;

use parser::Parser;

fn main() {
    let input = "def function(x, y) { x + 42 * y }";
    let (_, tokens) = tokenizer::tokenize(input).unwrap();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    match ast {
        Ok(ast) => println!("{:?}", ast),
        Err(error) => println!("Error: {}", error),
    }
}

