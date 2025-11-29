use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

use log::debug;

use crate::lexer::lexer::Lexer;
use crate::parser::ast_printer::AstPrinter;
use crate::parser::parser::Parser;
use crate::transformer::html_ast_transform::HtmlAstTransformer;
use crate::transformer::html_node::HtmlNode;
use crate::transformer::html_transform::HtmlTransformer;
use crate::transformer::html_wrapper::HtmlWrapper;

pub fn run_on_file(file_name: &str, output_file_name: Option<&str>) {
    let source = fs::read_to_string(&file_name).unwrap();

    let html = run(source);

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
        println!("Html output:\n{}", run(input));
    }
}

fn run(source: String) -> String {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    debug!(
        "Lexer output:\n{}\n",
        tokens.iter().map(|t| t.to_string()).collect::<String>()
    );

    let mut parser = Parser::new(tokens);
    let asts = parser.parse();
    let mut printer = AstPrinter;

    debug!(
        "Parser output:\n{}\n",
        asts.iter().map(|t| printer.print(t)).collect::<String>()
    );

    let mut ast_trans = HtmlAstTransformer;

    let hast: Vec<HtmlNode> = asts.iter().map(|ast| ast_trans.transform(ast)).collect();

    debug!(
        "Transform to html:\n{}\n",
        hast.iter().map(|t| format!("{:?}", t)).collect::<String>()
    );

    let hast = HtmlWrapper::wrap(hast);

    let html_trans = HtmlTransformer;

    let out = html_trans.transform(&hast);

    out
}
