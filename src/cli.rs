use std;
use crate::eval;



pub fn cli() {
    println!("Welcome to Chaiculator!");


    loop {
        println!("Please enter your expression:");

        let mut expression = String::new();
        std::io::stdin().read_line(&mut expression).expect("Failed to read line");

        let output;

        match eval::return_string_result(&expression){
            Ok(result) => output = result,
            Err(error) => {
                eprintln!("Error caught! {}", error);
                output = String::from("0");
            }
        };

        println!("\n\n----------------------------------------------");
        println!("Current expression: {}", expression);
        println!("Result: {}\n", output);
    }
}