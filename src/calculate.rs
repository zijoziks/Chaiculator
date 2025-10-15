use std::ops;
use std::str;
use crate::lisp::{Lisp, State};
use crate::lexer::{Token, ParseNumber};
use crate::lisp;

pub trait Number: ops::MulAssign + Clone + ParseNumber +
ops::Add<Output = Self> + ops::Sub<Output = Self> + ops::Mul<Output = Self> + ops::Div<Output = Self> { }

impl<T> Number for T where T: ops::MulAssign + Clone + ParseNumber +
ops::Add<Output = Self> + ops::Sub<Output = Self> + ops::Mul<Output = Self> + ops::Div<Output = Self> { }

pub enum Type {
    Integer,
    Float,
}

pub fn deduct_type (expression: &str) -> Type {
    if contains_float_operator(expression) {
        return Type::Float;
    }

    Type::Integer
}

fn contains_float_operator (expression: &str) -> bool {
    let required_operators: Vec<char> = vec![ '/', ','];

    for c in expression.chars() {
        if required_operators.contains(&c) {
            return true;
        }
    }

    false
}

pub fn begin_calculation<T> (expression: &str) -> Result<T, String>
where T: Number {
    let lisp_expression = lisp::expression(expression.trim())?;
    calculate(&lisp_expression)
}
fn calculate<T> (lisp: &Lisp<T>) -> Result<T, String>
where T: Number {
    // This function only accepts Lisp::Cons
    if !lisp.is_cons() {
        return Err(String::from("Expected Lisp::Cons in calculate()"));
    }

    let operator = lisp.unwrap_cons_op();

    let vec = lisp.unwrap_cons_vec();
    let first = vec.first().unwrap();
    let second = vec.last().unwrap();

    let first_number;
    let second_number;

    match lisp.vec_state() {
        State::Both => {
            // Cons & Atom
            if first.is_cons() {
                second_number = second.unwrap_atom().unwrap_token_num()?;
                first_number = calculate(first)?;
                // Atom & Cons
            } else if first.is_atom() {
                first_number = first.unwrap_atom().unwrap_token_num()?;
                second_number = calculate(second)?;
            } else {
                return Err(String::from("Unexpected behaviour in calculate()"))
            }
        }

        State::Atoms => {
            first_number = first.unwrap_atom().unwrap_token_num()?;
            second_number = second.unwrap_atom().unwrap_token_num()?;
        }

        State::Cons => {
            first_number = calculate(first)?;
            second_number = calculate(second)?;
        }
    }

    Ok(apply(operator, first_number, second_number))
}

// TODO error handling here
fn apply<T> (operator: &Token<T>, first_number: T, second_number: T) -> T
where T: Number {
    match operator {
        Token::Op('+') => first_number + second_number,
        Token::Op('-') => first_number - second_number,
        Token::Op('*') => first_number * second_number,
        Token::Op('/') => first_number / second_number,
        _ => panic!("Unexpected operator caught at apply() function.")
    }
}