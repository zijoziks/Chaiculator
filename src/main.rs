use std::io;

// u8 represent priority value
// the higher the number is, higher the priority
enum Token {
    Number(i32),
    Plus(char, u8),
    Minus(char, u8),
    Multiply(char, u8),
    Divide(char, u8)
}

fn output(expression: &mut str){
    println!("Expression: {}", expression);
}
fn main() {
    println!("Enter desired expression: ");
    let mut expression = String::new();
    io::stdin().read_line(&mut expression).expect("Failed to read line");
    output(&mut expression);
}
