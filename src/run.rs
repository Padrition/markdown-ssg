use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

use crate::lexer::lexer::Lexer;
use crate::parser::ast_printer::AstPrinter;
use crate::parser::parser::Parser;
use crate::transformer::html_ast_transform::HtmlAstTransformer;
use crate::transformer::html_node::HtmlNode;
use crate::transformer::html_transform::HtmlTransformer;

pub fn run_on_file(file_name: &str, output_file_name: Option<&str>, verbose: bool) {
    let source = fs::read_to_string(&file_name).unwrap();

    let html = run(source, verbose);

    let path = output_file_name.map(|s| s.to_string()).unwrap_or_else(|| {
        Path::new(file_name)
            .with_extension("html")
            .to_string_lossy()
            .to_string()
    });
    fs::write(path, html).unwrap_or_else(|err| eprintln!("Failed to write file: {err}"));
}

pub fn run_repl() {
    println!("Enter markdown code");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        run(input, true);
    }
}

fn run(source: String, verbose: bool) -> String {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    if verbose {
        println!("Lexer output:");
        tokens.iter().for_each(|x| print!("{x}"));
        println!();
    }

    let mut parser = Parser::new(tokens);
    let asts = parser.parse();
    let mut printer = AstPrinter;

    if verbose {
        println!("Parser output:");
        asts.iter().for_each(|ast| print!("{}", printer.print(ast)));
        println!();
    }

    let mut ast_trans = HtmlAstTransformer;

    let hast: Vec<HtmlNode> = asts.iter().map(|ast| ast_trans.transform(ast)).collect();

    if verbose {
        println!("Transform to html:");
        hast.iter().for_each(|h| print!("{:?}", h));
        println!();
    }

    let html_trans = HtmlTransformer;

    let out: String = hast
        .iter()
        .map(|h| html_trans.transform(h))
        .collect::<Vec<_>>()
        .join("");

    if verbose {
        println!("Html output:");
        println!("{}", out);
    }
    out
}
