mod lexer;
mod lisp;
mod calculate;
mod cli;

use crate::calculate::{begin_calculation};
use crate::cli::cli;

use clap::{Parser, ArgGroup};

// CLAP
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("mode")
    .required(true)
    .args(["expression", "cli"]),
))]
struct Args {
    #[arg(short, long)]
    expression: Option<String>,

    #[arg(short('c'), long("cli"))]
    cli: bool,
}

fn main() {
    let args = Args::parse();

    if args.cli {
        cli();
    } else if let Some(expression) = args.expression {
        let result = begin_calculation::<rug::Integer>(&expression);
        match result {
            Ok(result) => println!("{}", result),
            Err(error) => eprintln!("{}", error),
        }
    }
}