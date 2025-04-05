mod lexer;
mod error;
mod interpreter;
mod parser;

use std::fs;
use std::path::Path;
use std::env;
use error::Result;

fn main() -> Result<()> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if we have the right arguments
    let input: String = if args.len() >= 3 && args[1] == "--file" {
        let file_path: &String = &args[2];
        fs::read_to_string(Path::new(file_path)).expect("Failed to read file")
    } else {
        eprintln!("Usage: kdnlang --file <filename>");
        std::process::exit(1);
    };
    
    // Interpret the KdnLang code
    interpreter::interpret(&input)
}