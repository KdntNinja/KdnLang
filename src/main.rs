use crate::lexer::TokenWithSpan;
use crate::parser::{ASTNode, ParseError};
use clap::Parser;
use miette::Result;
use std::fs;

mod error_handling;
mod lexer;
mod parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Cli = Cli::parse();

    let code: String = fs::read_to_string(&args.file)
        .map_err(|e: std::io::Error| miette::miette!("Failed to read {}: {}", args.file, e))?;

    let tokens: Vec<TokenWithSpan> = lexer::tokenize(&code)?;

    let ast: ASTNode = parser::parse_program(&tokens)
        .map_err(|e: ParseError| miette::miette!("Parser error: {}", e))?;
    println!("AST: {:#?}", ast);

    Ok(())
}
