// These traits are necessary for methods that need copying
use std::str;
use super::traits::{Number, ParseNumber};
#[derive(Debug, Clone)]
pub enum Token<T> {
    Number(T),
    Op(char),
    EOF,
}

impl<T> Token<T> 
where T: Number {
    pub fn unwrap_token_num(&self) -> Result<T, String> {
        if let Token::Number(num) = self {
            Ok(num.clone())
        } else {
            Err(String::from("Expected a number in unwrap_token_num()."))
        }
    }
}

pub struct Lexer<T> {
    pub tokens: Vec<Token<T>>,
}

impl<T> Lexer<T> 
where T: Number + ParseNumber {
    fn deduct_token(deduct_from: char) -> Result<Token<T>, String> {
        match deduct_from {
            '+' => Ok(Token::Op('+')),
            '-' => Ok(Token::Op('-')),
            '*' => Ok(Token::Op('*')),
            '/' => Ok(Token::Op('/')),
            _ => {
                Err(format!("Invalid token found: {}", deduct_from))
            }
        }
    }

    pub fn new(expression: &str) -> Result<Lexer<T>, String> {

        if !is_expression_valid(expression) {
            return Err(format!("Invalid expression: {}", expression));
        }

        let mut tokens: Vec<Token<T>> = Vec::new();

        let mut number = String::new();

        // We'll use the following string because it makes our lexer work properly
        let mut for_expression = expression.to_string();
        for_expression.push('\n');


        // FOR LOOP BEGINS HERE
        for c in for_expression.chars() {
            if c.is_ascii_digit() {
                number.push(c);
            }

            // Encounter an operator before a number
            if is_operator(c) && number.is_empty() {
                if c == '-' {
                    number.push(c);
                    continue;
                } else {
                    return Err(format!("Invalid operator: {}", c))
                }
            }

            // Encounter an operator
            if (!c.is_ascii_digit() || c == '\n') && !number.is_empty() {
                let num = ParseNumber::parse_number(&number)?;
                tokens.push(Token::Number(num));
                number.clear();

                // End of expression
                if c == '\n' {
                    break;
                }

                // Push the operator
                let token = Lexer::deduct_token(c)?;
                tokens.push(token);
            }
        }

        tokens.reverse();

        Ok(Lexer { tokens })
    }



    // Following 2 methods assume the vector is reversed
    pub fn next(&mut self) -> Token<T> {
        self.tokens.pop().unwrap_or(Token::EOF)
    }

    pub fn peek(&mut self) -> Token<T> {
        self.tokens.last().cloned().unwrap_or(Token::EOF)
    }
}

fn is_expression_valid(expression: &str) -> bool {
    let allowed: Vec<char> = vec!
        [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
            '+', '-', '*', '/',
        ];
    expression.chars().all(|c| allowed.contains(&c))
}

fn is_operator(operator: char) -> bool {
    let allowed_operators: Vec<char> =
        vec![
            '+', '-', '*', '/'
        ];

    for c in allowed_operators {
        if operator == c {
            return true;
        }
    }

    false
}