mod lexer;
mod error;
mod interpreter;
mod parser;
mod cli;

use clap::Parser;
use error::Result;
use crate::cli::{CodeSource, Cli};

fn main() -> Result<()> {
    let cli: Cli = cli::Cli::parse();
    let source_code: CodeSource = match cli::handle_commands(cli) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };
    
    // Interpret the KdnLang code
    interpreter::interpret(&source_code)
}