mod lexer;
mod markdown;
mod parser;

use std::env;
use std::fs;
use std::io;
use std::io::Write;

use crate::lexer::ErrorHandler;
use crate::lexer::lexer::Lexer;
use crate::parser::ast_printer::AstPrinter;
use crate::parser::in_line_node::InLineNode::{Emphasis, Strikethrough, Strong, Text};
use crate::parser::markdown_node::MarkdownNode::{Heading, Paragraph};

fn main() {
    let ast = vec![
        Heading {
            level: 2,
            content: vec![Strong(vec![Text("Hello".into())]), Text("world".into())],
        },
        Paragraph(vec![
            Text("This is".into()),
            Emphasis(vec![Text(" emphasized".into())]),
            Text(".".into()),
        ]),
    ];

    let mut printer = AstPrinter;
    for node in ast {
        println!("{}", printer.print(&node));
    }

    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Use :");
    } else if args.len() == 2 {
        let file_name = &args[1];
        println!("Lexing file: {file_name}");
        run_on_file(file_name);
    } else {
        println!("Enter markdown code");
        run_repl();
    }
}
fn run_on_file(file_name: &String) {
    let source = fs::read_to_string(&file_name).unwrap();
    run(source);
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

    tokens.iter().for_each(|x| print!("{x}"));
}
