use clap::Parser;
use miette::{IntoDiagnostic, Result};
use std::fs;
use std::path::Path;

mod error_handling;
mod interpreter;
mod lexer;
mod parser;
mod stdlib;
mod typecheck; // Add the new module

#[derive(Parser)]
#[command(
    author = "KdntNinja",
    version = "0.0.1",
    about = "KdnLang - A Statically-Typed, Pythonic Language with Rust-Like Syntax"
)]
struct Cli {
    #[arg(short, long, help = "Path to the KdnLang source file to execute")]
    file: String,
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();
    let source_file: &str = &args.file;

    // Get the actual filename (not the full path) for error reporting
    let filename: &str = Path::new(source_file)
        .file_name()
        .and_then(|name: &std::ffi::OsStr| name.to_str())
        .unwrap_or(source_file);

    // Read the source file
    let code: String = fs::read_to_string(source_file)
        .into_diagnostic()
        .map_err(|_| miette::miette!("Failed to read {}", source_file))?;

    // Lexical analysis
    let tokens: Vec<lexer::tokens::TokenWithSpan> = lexer::tokenize(&code, filename)?;

    // Parsing
    let ast: parser::ASTNode = parser::parse_program(&tokens, filename)?;

    // Type checking (new step)
    typecheck::typecheck_program(&ast, filename, &code)?;

    // Execution
    let mut interpreter: interpreter::Interpreter =
        interpreter::Interpreter::with_source(&code, filename);
    let _result: interpreter::Value = interpreter.interpret(&ast)?;

    Ok(())
}
