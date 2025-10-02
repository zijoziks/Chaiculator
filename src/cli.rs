use std::io;
use crate::lisp;
use crate::lisp::Lisp;
use crate::calculate;

pub fn cli() {
    println!("Welcome to Chaiculator!");

    let mut current_expression = String::new();
    let mut result = 0;

    loop {
        println!("\n\nCurrent expression: {}", current_expression);
        println!("Result: {}\n", result);
        println!("Please enter your expression:");

        current_expression.clear();
        io::stdin().read_line(&mut current_expression).expect("Failed to read line");

        let lisp_expression: Lisp = lisp::expression(current_expression.trim());
        result = calculate(&lisp_expression);
    }
}