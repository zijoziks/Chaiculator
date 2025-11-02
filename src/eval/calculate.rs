use std::str;
use super::lisp::{Lisp, State};
use super::lexer::{Token, NumberKind};
use super::lisp;
use super::traits::Number;

pub fn begin_calculation<T> (expression: &str) -> Result<T, String>
where T: Number {
    let lisp_expression = lisp::expression(expression.trim())?;
    let result = calculate(&lisp_expression)?;
    result.unwrap()
}

fn calculate<T> (lisp: &Lisp<T>) -> Result<NumberKind<T>, String>
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

    Ok(apply(operator, first_number, second_number)?)
}

// TODO error handling here
fn apply<T> (operator: &Token<T>, first_number: NumberKind<T>, second_number: NumberKind<T>) -> Result<NumberKind<T>, String>
where T: Number {

    // Percentage case is when we get X ± Y%
    // wherein we have to calculate it like so
    // X ± X * Y/100
    // unwrap by default returns Y/100 if the number is percent
    let percentage_case;
    if first_number.is_normal() && second_number.is_percent(){
        percentage_case = true;
    } else {
        percentage_case = false;
    }

    let left_side = first_number.unwrap()?;
    let right_side = second_number.unwrap()?;
    let result;



    match operator {
        Token::Op('+') => {
            if percentage_case {
                result = left_side.clone() + (left_side * right_side);
            } else {
                result = left_side + right_side
            }
        },
        Token::Op('-') => {
            if percentage_case {
                result = left_side.clone() - (left_side * right_side);
            } else {
                result = left_side - right_side
            }
        },
        Token::Op('*') => result = left_side * right_side,
        Token::Op('/') => result = left_side / right_side,
        _ => return Err(String::from("Unexpected token in apply()"))
    };

    Ok(NumberKind::Normal(result))
}