mod lexer;
mod lisp;
mod calculate;
mod cli;

use crate::calculate::{begin_calculation};
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
        let result = begin_calculation(expression.as_str());
        println!("{}", result);
    } else {
        eprintln!("Unexpected behaviour in main().");
        std::process::exit(1);
    }
}