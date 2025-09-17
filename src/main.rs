mod lexer;

use std::io;
use std::vec::Vec;
use crate::lexer::Lexer;
use crate::lexer::Token;

fn output(expression: &mut str) {
    println!("Expression: {}", expression);
}
fn main() {
    println!("Enter desired expression: ");
    let mut expression = String::new();
    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read line");
    let tokens: Vec<Token> = Lexer::new(&expression).tokens;
    output(&mut expression);
}
