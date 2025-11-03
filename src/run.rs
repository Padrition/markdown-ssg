use std::fs;
use std::io;
use std::io::Write;

use crate::lexer::lexer::Lexer;
use crate::parser::ast_printer::AstPrinter;
use crate::parser::parser::Parser;
use crate::transformer::html_ast_transform::HtmlAstTransformer;
use crate::transformer::html_node::HtmlNode;
use crate::transformer::html_transform::HtmlTransformer;

pub fn run_on_file(file_name: &String) {
    println!("Lexing file: {file_name}");
    let source = fs::read_to_string(&file_name).unwrap();
    run(source);
}

pub fn run_repl() {
    println!("Enter markdown code");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        run(input);
    }
}

fn run(source: String) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    println!("Lexer output:");
    tokens.iter().for_each(|x| print!("{x}"));
    println!();

    let mut parser = Parser::new(tokens);
    let asts = parser.parse();
    let mut printer = AstPrinter;

    println!("Parser output:");
    asts.iter().for_each(|ast| print!("{}", printer.print(ast)));
    println!();

    let mut ast_trans = HtmlAstTransformer;

    println!("Transform to html:");
    let hast: Vec<HtmlNode> = asts.iter().map(|ast| ast_trans.transform(ast)).collect();
    hast.iter().for_each(|h| print!("{:?}", h));
    println!();

    let html_trans = HtmlTransformer;

    println!("Html output:");
    let out: String = hast
        .iter()
        .map(|h| html_trans.transform(h))
        .collect::<Vec<_>>()
        .join("");
    println!("{}", out);
}
