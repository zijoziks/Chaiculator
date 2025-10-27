mod cli;
mod gui;
mod eval;

use crate::cli::cli;
use crate::gui::gui;

use clap::{Parser, ArgGroup};

// CLAP
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("mode")
    .required(true)
    .args(["expression", "cli", "gui"]),
))]
struct Args {
    #[arg(short, long)]
    expression: Option<String>,

    #[arg(short('c'), long("cli"))]
    cli: bool,

    #[arg(short('g'), long("gui"))]
    gui: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.cli {
        cli();
    } else if args.gui {
        gui()?;
    } else if let Some(expression) = args.expression {
        let result = eval::return_string_result(&expression);
        match result {
            Ok(result) => println!("{}", result),
            Err(error) => eprintln!("{}", error),
        }
    }

    Ok(())
}