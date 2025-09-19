// These traits are necessary for methods that need copying
#[derive(Clone, Copy)]
pub enum Token {
    Number(i32),
    Op(char),
    EOF,
    Invalid,
}

pub struct Lexer {
    // TODO make getter
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

    pub fn new(expression: &str) -> Lexer {
        let mut tokens: Vec<Token> = Vec::new();

        let mut first_index: Option<usize> = None;
        
        // We'll use the following string because it makes our lexer work properly
        let mut for_expression = expression.to_string();
        for_expression.push('\n');
            
        for (i, c) in for_expression.chars().enumerate() {
            // Encounter first digit
            if c.is_ascii_digit() && first_index.is_none() {
                first_index = Some(i);
            }

            // Encounter an operator
            if (!c.is_ascii_digit() || c == '\n') && !first_index.is_none() {
                if let Some(index) = first_index {
                    let num: i32 = expression[index..i].parse().expect("Couldn't parse number");
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

        Lexer { tokens }
    }

    // Following 2 methods assume the vector is reversed
    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::EOF)
    }

    pub fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::EOF)
    }
}