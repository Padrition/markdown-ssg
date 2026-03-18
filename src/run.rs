use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use log::debug;
use log::error;

use crate::lexer::scanner::Scanner;
use crate::parser::ast_printer::AstPrinter;
use crate::parser::core::Parser;
use crate::transformer::html_ast_transform::HtmlAstTransformer;
use crate::transformer::html_node::HtmlNode;
use crate::transformer::html_transform::HtmlTransformer;
use crate::transformer::html_wrapper::HtmlWrapper;

pub fn run_on_file<P: AsRef<Path>>(path: P, output_file_name: Option<PathBuf>) {
    let source = fs::read_to_string(&path).unwrap();

    let html = run(source);

    let output_path = output_file_name.unwrap_or_else(|| path.as_ref().with_extension("html"));
    fs::write(output_path, html).unwrap_or_else(|err| eprintln!("Failed to write file: {err}"));
}

pub fn run_on_dir<P: AsRef<Path>>(path: P) {
    let dir = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(err) => {
            return error!("Error reading directory: {err}");
        }
    };

    let mut files = vec![];

    for entry in dir.filter_map(|r| r.map_err(|err| error!("Error reading entry: {err}")).ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            files.push(path);
        }
    }
    let output_dir = Path::new("output");
    let _ = fs::create_dir_all(output_dir);

    for f in &files {
        let output_path = output_dir
            .join(f.file_name().unwrap())
            .with_extension("html");
        run_on_file(f, Some(output_path));
    }

    let index = HtmlWrapper::create_index(&files);
    let transformed_index = HtmlTransformer::transform(&index);

    fs::write(output_dir.join("index.html"), transformed_index)
        .unwrap_or_else(|err| eprintln!("Failed to write file: {err}"));
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
    let mut lexer = Scanner::new(source);
    let tokens = lexer.scan_tokens();

    debug!(
        "Scanner output:\n{}\n",
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
        hast.iter().map(|t| format!("{t:?}")).collect::<String>()
    );

    let hast = HtmlWrapper::wrap(hast);

    HtmlTransformer::transform(&hast)
}
