use std::io;
use std::vec::Vec;

// u8 represent priority value
// the higher the number is, higher the priority
enum Token {
    Number(i32),
    Plus(u8),
    Minus(u8),
    Multiply(u8),
    Divide(u8),
    Invalid
}

// TODO replace magic variables with global constants
fn deduct_token(deduct_from: char) -> Token{
    match deduct_from {
        '+' => Token::Plus(1),
        '-' => Token::Minus(1),
        '*' => Token::Multiply(2),
        '/' => Token::Divide(2),
        _ => {
            println!("Huh?");
            Token::Invalid
        }
    }
}

fn convert_to_tokens(expression: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut first_index: Option<usize> = None;

    for (i, c) in expression.chars().enumerate() {

        // Encounter first digit
        if c.is_ascii_digit() && first_index.is_none() {
            first_index = Some(i);
        }

        // Encounter an operator
        if !c.is_ascii_digit() && !first_index.is_none() {
            if let Some(index) = first_index {

                let num: i32 = expression[index..i].parse().expect("Couldn't parse number");
                tokens.push(Token::Number(num));

                if c == '\n' {
                    break;
                }

                let token = deduct_token(c);
                // TODO error handling for invalid cases and invalid token vector
                tokens.push(token);
            }

            first_index = None;
        }

    }
    tokens
}

fn output(expression: &mut str){
    println!("Expression: {}", expression);
}
fn main() {
    println!("Enter desired expression: ");
    let mut expression = String::new();
    io::stdin().read_line(&mut expression).expect("Failed to read line");
    let tokens: Vec<Token> = convert_to_tokens(&expression);
    output(&mut expression);
}
