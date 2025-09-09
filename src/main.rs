use std::io;

fn output(expression: &mut str){
    println!("Expression: {}", expression);
}
fn main() {
    println!("Enter desired expression: ");
    let mut expression = String::new();
    io::stdin().read_line(&mut expression).expect("Failed to read line");
    output(&mut expression);
}
