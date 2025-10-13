// These traits are necessary for methods that need copying
use std::ops;
use std::str;

#[derive(Debug, Clone)]
pub enum Token<T> {
    Number(T),
    Op(char),
    EOF,
    Invalid,
}

impl<T> Token<T> 
where T: Clone {
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
where T: Clone + From<i32> + ops::MulAssign + str::FromStr<Err=String> {
    fn deduct_token(deduct_from: char) -> Token<T> {
        match deduct_from {
            '+' => Token::Op('+'),
            '-' => Token::Op('-'),
            '*' => Token::Op('*'),
            '/' => Token::Op('/'),
            _ => {
                println!("Huh?");
                Token::Invalid
            }
        }
    }

    pub fn new(expression: &str) -> Result<Lexer<T>, String> {

        if !is_expression_valid(expression) {
            return Err(format!("Invalid expression: {}", expression));
        }

        let mut tokens: Vec<Token<T>> = Vec::new();

        let mut first_index: Option<usize> = None;
        let mut minus_sign: bool = false;
        
        // We'll use the following string because it makes our lexer work properly
        let mut for_expression = expression.to_string();
        for_expression.push('\n');


        // FOR LOOP BEGINS HERE
        for (i, c) in for_expression.chars().enumerate() {
            // Encounter first digit
            if c.is_ascii_digit() && first_index.is_none() {
                first_index = Some(i);
            }

            // Encounter an operator before a number
            if is_operator(c) && first_index.is_none() {
                if c == '-' && minus_sign == false {
                    minus_sign = true;
                } else {
                    return Err(format!("Invalid operator: {}", c))
                }
            }

            // Encounter an operator
            if (!c.is_ascii_digit() || c == '\n') && !first_index.is_none() {
                if let Some(index) = first_index {
                    let mut num= T::from(for_expression[index..i].to_string().as_str());

                    if minus_sign {
                        num *= T::from(-1);
                        minus_sign = false;
                    }

                    tokens.push(Token::Number(num));

                    if c == '\n' {
                        break;
                    }

                    let token = Lexer::deduct_token(c);
                    // TODO error handling for invalid cases and invalid token vector
                    tokens.push(token);
                }

                first_index = None;
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