mod lexer;
mod parser;
mod run;
mod transformer;

use run::{run_on_file, run_repl};
use std::{fs, io::Write, path::Path};

use clap::Parser;
use env_logger::Builder;
use log::{LevelFilter, error};

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

    //Custom log messages
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

    match cli.path {
        Some(path) => {
            let path = Path::new(&path);

            match fs::metadata(path) {
                Ok(metadata) => {
                    if metadata.is_file() {
                        run_on_file(path, cli.output.as_deref())
                    } else if metadata.is_dir() {
                        run_on_dir(path);
                    }
                }
                Err(err) => error!("Path can't be accepted {err}"),
            }
        }
        None => run_repl(),
    }
}
