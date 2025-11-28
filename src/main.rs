mod lexer;
mod parser;
mod run;
mod transformer;

use run::{run_on_file, run_repl};
use std::io::Write;

use clap::Parser;
use env_logger::Builder;
use log::LevelFilter;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    // Optional markdown file name
    file: Option<String>,

    // Optional output file name
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    Builder::new()
        .filter_level(if cli.verbose {
            LevelFilter::max()
        } else {
            LevelFilter::Warn
        })
        .format(|buf, record| {
            let warn_style = buf.default_level_style(record.level());
            let level = record.level().to_string();

            writeln!(buf, "{warn_style}{level}:{warn_style:#}{}", record.args())
        })
        .init();

    match cli.file {
        Some(file_name) => run_on_file(&file_name, cli.output.as_deref()),
        None => run_repl(),
    }
}
