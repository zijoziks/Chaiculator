mod lexer;
mod lisp;
mod calculate;
mod traits;

use rug;

pub fn return_string_result(expression: &str) -> Result<String, String> {
    match deduct_type(expression) {
        Type::Integer => {
            let result = calculate::begin_calculation::<rug::Integer>(expression)?;
            Ok(result.to_string())
        }

        Type::Float => {
            let result = calculate::begin_calculation::<rug::Float>(expression)?;
            Ok(result.to_string())
        }
    }
}

enum Type {
    Integer,
    Float,
}

fn deduct_type (expression: &str) -> Type {
    if contains_float_operator(expression) {
        return Type::Float;
    }

    Type::Integer
}

fn contains_float_operator (expression: &str) -> bool {
    let required_operators: Vec<char> = vec![ '/', ',', '%'];

    for c in expression.chars() {
        if required_operators.contains(&c) {
            return true;
        }
    }

    false
}