mod lexer;
mod markdown;

use std::env;
use std::io;
use std::io::Write;

use crate::lexer::ErrorHandler;
use crate::lexer::lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Use :");
    } else if args.len() == 2 {
        println!("Lexing file:")
    } else {
        println!("Enter markdown code");
        run_repl();
    }
}

fn run_repl() {
    loop {
        println!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        run(input);
    }
}

fn run(source: String) {
    let mut error_handler = ErrorHandler::new();
    let mut lexer = Lexer::new(source, &mut error_handler);
    let tokens = lexer.scan_tokens();

    tokens.iter().for_each(|x| println!("{x}"));
}
