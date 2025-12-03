use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use log::debug;
use log::error;

use crate::lexer::lexer::Lexer;
use crate::parser::ast_printer::AstPrinter;
use crate::parser::parser::Parser;
use crate::transformer::html_ast_transform::HtmlAstTransformer;
use crate::transformer::html_node::HtmlNode;
use crate::transformer::html_transform::HtmlTransformer;
use crate::transformer::html_wrapper::HtmlWrapper;

pub fn run_on_file(path: &Path, output_file_name: Option<&str>) {
    let source = fs::read_to_string(&path).unwrap();

    let html = run(source);

    let path = output_file_name
        .map(|s| s.to_string())
        .unwrap_or_else(|| match path.file_name() {
            Some(e) => Path::new(e)
                .with_extension("html")
                .to_string_lossy()
                .to_string(),
            None => String::from("new_file.html"),
        });
    fs::write(path, html).unwrap_or_else(|err| eprintln!("Failed to write file: {err}"));
}

pub fn run_on_dir(path: &Path) {
    let dir = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(err) => {
            error!("Error reading directory: {err}");
            return;
        }
    };

    let mut files = vec![];

    for entry_result in dir {
        let entry = match entry_result {
            Ok(e) => e,
            Err(err) => {
                error!("Error reading entry: {err}");
                continue;
            }
        };

        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    files.push(path);
                }
            }
        }
    }

    files.iter().for_each(|f| {
        let output_dir_name = "output";
        let mut output_path = PathBuf::from(output_dir_name);

        output_path.push(f.file_name().unwrap());
        output_path = output_path.with_extension("html");

        let _ = fs::create_dir_all(output_dir_name);
        run_on_file(f, Some(&output_path.to_string_lossy().to_string()));
    });
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
