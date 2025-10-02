mod lexer;
mod lisp;
mod calculate;
mod cli;

use crate::lisp::{Lisp};
use crate::calculate::calculate;
use crate::cli::cli;

use clap::Parser;

// CLAP
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        conflicts_with("cli"),
        required_unless_present("cli"),
    )]
    expression: Option<String>,

    #[arg(
        short('c'),
        long("cli"),
        conflicts_with("expression"),
        required_unless_present("expression"),
    )]
    cli: bool,
}

fn main() {
    let args = Args::parse();

    if args.cli {
        cli();
    } else if let Some(expression) = args.expression{
        let lisp_expression: Lisp = lisp::expression(expression.trim());
        let result = calculate(&lisp_expression);
        println!("{}", result);
    } else {
        eprintln!("Unexpected behaviour in main().");
        std::process::exit(1);
    }
}