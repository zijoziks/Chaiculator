use std::ops;
use std::str;
use crate::lexer::{Token, Lexer};

#[derive(Debug)]
pub enum Lisp<T> {
    Atom(Token<T>),
    Cons(Token<T>, Vec<Lisp<T>>)
}

pub enum State {
    Atoms,
    Cons,
    Both
}

impl<T> Lisp<T> {
    pub fn is_atom(&self) -> bool {
        matches!(self, Lisp::Atom(_))
    }

    pub fn is_cons(&self) -> bool {
        matches!(self, Lisp::Cons(_, _))
    }

    pub fn vec_state(&self) -> State {
        if self.is_atom() {
            panic!("Called is_vec_regular() on Lisp::Atom.")
        }

        let vec = self.unwrap_cons_vec();

        if vec.len() < 1 || vec.len() > 2 {
            panic!("Invalid vector length: {}", vec.len())
        }

        let first = vec.first().unwrap();
        let second = vec.last().unwrap();

        // Atom & Cons
        if (first.is_atom() && second.is_cons()) ||
            (first.is_cons() && second.is_atom()) {
            State::Both
        } else if first.is_atom() && second.is_atom() { // Atom & Atom
            State::Atoms
        } else if first.is_cons() && second.is_cons(){
            State::Cons
        } else {
            panic!("Unexpected beahviour when determining vector state.")
        }
    }

    pub fn unwrap_atom(&self) -> &Token<T> {
        match self {
            Lisp::Atom(token) => token,
            Lisp::Cons(_,_) => panic!("Called unwrap_atom on a Lisp::Cons.")
        }
    }

    pub fn unwrap_cons_op(&self) -> &Token<T> {
        match self {
            Lisp::Cons(token, _) => token,
            Lisp::Atom(_) => panic!("Called unwrap_cons_op on a Lisp::Atom")
        }
    }

    pub fn unwrap_cons_vec(&self) -> &Vec<Lisp<T>> {
        match self {
            Lisp::Cons(_, vec) => vec,
            Lisp::Atom(_) => panic!("Called unwrap_cons_vec on a Lisp::Cons.")
        }
    }
}

pub fn expression<T> (input: &str) -> Result<Lisp<T>, String>
where T: ops::MulAssign + Clone + From<i32> + str::FromStr {
    let mut lexer = Lexer::new(input)?;
    Ok(expression_bp(&mut lexer, 0))
}

fn expression_bp<T> (lexer: &mut Lexer<T>, min_bp: u8) -> Lisp<T>
where T: ops::MulAssign + Clone + From<i32> + str::FromStr {
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
            lhs = Lisp::Cons(Token::Op(op), vec![lhs, rhs]);

            continue;
        }

        break;
    }

    lhs
}

// This functions gives the binding power to the operator and determines which one comes first
fn infix_binding_power(op: char) -> Option<(u8, u8)> {
    let result = match op {
        '+' | '-' => (1, 2),
        '*' | '/' => (3, 4),
        _ => return None,
    };
    Some(result)
}