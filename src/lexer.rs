pub enum Token {
    Atom(i32),
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

        for (i, c) in expression.chars().enumerate() {
            // Encounter first digit
            if c.is_ascii_digit() && first_index.is_none() {
                first_index = Some(i);
            }

            // Encounter an operator
            if !c.is_ascii_digit() && !first_index.is_none() {
                if let Some(index) = first_index {
                    let num: i32 = expression[index..i].parse().expect("Couldn't parse number");
                    tokens.push(Token::Atom(num));

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

        Lexer { tokens }
    }
}