use std;
use rug;
use crate::calculate;

enum CalculationResults {
    Integer(rug::Integer),
    Float(rug::Float),
}

impl std::fmt::Display for CalculationResults {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CalculationResults::Integer(n) => write!(f, "{}", n),
            CalculationResults::Float(n) => write!(f, "{}", n),
        }
    }
}

pub fn cli() {
    println!("Welcome to Chaiculator!");


    loop {
        println!("Please enter your expression:");

        let mut expression = String::new();
        std::io::stdin().read_line(&mut expression).expect("Failed to read line");

        let result = match calculate::deduct_type(&expression) {
            calculate::Type::Integer => {
                match calculate::begin_calculation::<rug::Integer>(&expression) {
                    Ok(result) => CalculationResults::Integer(result),
                    Err(error) => {
                        eprintln!("ERROR: {}", error);
                        continue;
                    },
                }
            } calculate::Type::Float => {
                match calculate::begin_calculation::<rug::Float>(&expression) {
                    Ok(result) => CalculationResults::Float(result),
                    Err(error) => {
                        eprintln!("ERROR: {}", error);
                        continue;
                    },
                }
            }
        };

        println!("\n\n----------------------------------------------");
        println!("Current expression: {}", expression);
        println!("Result: {}\n", result);
    }
}