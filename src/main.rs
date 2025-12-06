mod lexer;
mod logger;
mod parser;
mod run;
mod transformer;

use run::{run_on_file, run_repl};
use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use log::error;

use crate::run::run_on_dir;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    // Optional markdown file name
    path: Option<String>,

    // Optional output file name
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    logger::init(cli.verbose);

    if let Some(path_str) = cli.path.as_deref() {
        let path = Path::new(path_str);
        let output_path = cli.output.as_deref().map(PathBuf::from);

        match fs::metadata(path) {
            Ok(m) if m.is_file() => run_on_file(path, output_path),
            Ok(m) if m.is_dir() => run_on_dir(path),
            Ok(_) => error!("Path is neither file nor dir"),
            Err(err) => error!("Path can't be accepted {err}"),
        }
    } else {
        run_repl();
    };
}
