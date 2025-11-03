mod lexer;
mod parser;
mod run;
mod transformer;

use run::{run_on_file, run_repl};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Use :");
    } else if args.len() == 2 {
        let file_name = &args[1];
        run_on_file(file_name);
    } else {
        run_repl();
    }
}
