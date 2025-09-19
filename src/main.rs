mod lexer;
mod lisp;

use std::io;
use crate::lisp::{Lisp,expression};

fn output(expression: &mut str) {
    println!("Expression: {}", expression);
}
fn main() {
    println!("Enter desired expression: ");
    let mut expression = String::new();
    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read line");
    let lisp_expression: Lisp = lisp::expression(expression.trim());
    output(&mut expression);
}
