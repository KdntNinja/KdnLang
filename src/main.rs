use clap::Parser;
use miette::Result;
use std::fs;
use crate::lexer::Token;

mod errors;
mod lexer;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the .kdn file to load
    #[arg(short, long)]
    file: String,
}

fn main() -> Result<()> {
    // Parse command-line arguments
    let args: Cli = Cli::parse();

    // Load code from the specified .kdn file
    let code: String = fs::read_to_string(&args.file)
        .map_err(|e: std::io::Error| miette::miette!("Failed to read {}: {}", args.file, e))?;

    // Pass the code to the lexer
    let tokens: Vec<(Token, String)> = lexer::tokenize(&code)?;

    // Print tokens for now
    for token in tokens.iter() {
        println!("{:?}", token);
    }

    Ok(())
}
