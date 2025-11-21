mod lexer;
mod parser;
mod run;
mod transformer;

use run::{run_on_file, run_repl};
use std::env;

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    // Optional markdown file name
    file: Option<String>,

    // Optional output file name
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.file {
        Some(file_name) => run_on_file(&file_name, cli.output.as_deref()),
        None => run_repl(),
    }
}
