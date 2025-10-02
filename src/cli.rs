use std::io;
use crate::calculate::begin_calculation;

pub fn cli() {
    println!("Welcome to Chaiculator!");

    let mut current_expression = String::new();
    let mut result = 0;

    loop {
        println!("\n\n----------------------------------------------");
        println!("Current expression: {}", current_expression);
        println!("Result: {}\n", result);
        println!("Please enter your expression:");

        current_expression.clear();
        io::stdin().read_line(&mut current_expression).expect("Failed to read line");

        result = begin_calculation(current_expression.as_str());
    }
}