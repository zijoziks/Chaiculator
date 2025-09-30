use crate::lisp::Lisp;
use crate::lexer::Token;
pub fn calculate(lisp: &Lisp) -> i128 {

    // This function only accepts Lisp::Cons
    if !lisp.is_cons() {
        panic!("Expected a Lisp::Cons")
    }

    let operator = lisp.unwrap_cons_op();

    let vec = lisp.unwrap_cons_vec();
    let first = vec.first().unwrap();
    let second = vec.last().unwrap();

    let first_number;
    let second_number;

    // Atom & Cons
    if lisp.is_vec_regular() {
        // Cons & Atom
        if first.is_cons() {
            second_number = second.unwrap_atom().unwrap_token_num();
            first_number = calculate(first);
            // Atom & Cons
        } else if first.is_atom() {
            first_number = first.unwrap_atom().unwrap_token_num();
            second_number = calculate(second);
        } else {
            panic!("Unexpected behaviour")
        }
        // Atom & Atom
    } else {
        first_number = first.unwrap_atom().unwrap_token_num();
        second_number = second.unwrap_atom().unwrap_token_num();
    }

    apply(operator, first_number, second_number)
}

fn apply(operator: &Token, first_number: i128, second_number: i128) -> i128 {
    match operator {
        Token::Op('+') => first_number + second_number,
        Token::Op('-') => first_number - second_number,
        Token::Op('*') => first_number * second_number,
        Token::Op('/') => first_number / second_number,
        _ => panic!("Unexpected operator caught at apply() function.")
    }
}