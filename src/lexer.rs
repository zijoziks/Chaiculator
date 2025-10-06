// These traits are necessary for methods that need copying
#[derive(Clone, Copy, Debug)]
pub enum Token {
    Number(i128),
    Op(char),
    EOF,
    Invalid,
}

impl Token {
    pub fn unwrap_token_num(self) -> Result<i128, String> {
        if let Token::Number(num) = self {
            Ok(num)
        } else {
            Err(String::from("Expected a number in unwrap_token_num()."))
        }
    }
}

pub struct Lexer {
    pub tokens: Vec<Token>,
}

impl Lexer {
    fn deduct_token(deduct_from: char) -> Token {
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

    pub fn new(expression: &str) -> Result<Lexer, String> {

        if !is_expression_valid(expression) {
            return Err(format!("Invalid expression: {}", expression));
        }

        let mut tokens: Vec<Token> = Vec::new();

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

            // Encounter an operator
            if (!c.is_ascii_digit() || c == '\n') && !first_index.is_none() {
                if let Some(index) = first_index {
                    let mut num= for_expression[index..i].parse().expect("Couldn't parse number");

                    if minus_sign {
                        num *= -1;
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

            // Encounter an operator before a number
            if (is_operator(c) && first_index.is_none()) {
                if c == '-' && minus_sign == false {
                    minus_sign = true;
                } else {
                    return Err(format!("Invalid operator: {}", c))
                }
            }
        }

        tokens.reverse();

        Ok(Lexer { tokens })
    }



    // Following 2 methods assume the vector is reversed
    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::EOF)
    }

    pub fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::EOF)
    }
}

fn is_expression_valid(expression: &str) -> bool {
    let allowed: Vec<char> =
        [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
            '+', '-', '*', '/',
        ]
            .into_iter().collect();
    expression.chars().all(|c| allowed.contains(&c))
}

fn is_operator(operator: char) -> bool {
    let allowed_operators: Vec<char> =
        [
            '+', '-', '*', '/'
        ]
    .into_iter().collect();

    for c in allowed_operators {
        if operator == c {
            return true;
        }
    }

    false
}