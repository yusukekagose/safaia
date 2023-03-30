use safire::{tokenizer, parser}; // 追加

use tokenizer::tokenize;
use parser::Parser;

fn main() {
    let input = "def function(x, y) { x + 42 * y }"; // Change this line
    let (_, tokens) = tokenize(input).unwrap();

    println!("{:?}", tokens);
}
