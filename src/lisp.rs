use crate::lexer::{Token, Lexer};

pub enum Lisp {
    Atom(Token),
    Cons(Token, Vec<Lisp>)
}

pub fn expression(input: &str) -> Lisp {
    let mut lexer = Lexer::new(input);
    expression_bp(&mut lexer, 0)
}

fn expression_bp(lexer: &mut Lexer, min_bp: u8) -> Lisp {
    // First part
    let mut lhs = match lexer.next() {
        Token::Number(it) => Lisp::Atom(Token::Number(it)),
        _ => panic!("Bad token"),
    };

    loop {
        // Second part
        let op = match lexer.peek() {
            Token::EOF => break,
            Token::Op(op) => op,
            _ => panic!("Bad token"),
        };

        if let Some((left_bp, right_bp)) = infix_binding_power(op) {
            if left_bp < min_bp {
                break;
            }

            lexer.next();

            // Third part
            let rhs = expression_bp(lexer, right_bp);
            lhs = Lisp::Cons(Token::Op(op), vec![lhs, rhs])
        }

        break;
    }

    lhs
}

// This functions gives the binding power to the operator and determines which one comes first
fn infix_binding_power(op: char) -> Option<(u8, u8)> {
    let result = match op {
        '+' | '-' => (2, 1),
        '*' | '/' => (3, 4),
        _ => return None,
    };
    Some(result)
}