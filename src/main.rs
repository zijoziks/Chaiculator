mod lexer;
mod lisp;
mod calculate;

use std::io;
use crate::lisp::{Lisp};
use crate::calculate::calculate;

fn main() {
    println!("Enter desired expression: ");
    let mut expression = String::new();
    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read line");
    let lisp_expression: Lisp = lisp::expression(expression.trim());
    let result = calculate(&lisp_expression);
    println!("{}", result);
}