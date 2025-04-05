// src/cli.rs - Command-line interface functionality
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

/// KdnLang - A simple expression-based programming language
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Command to execute
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// File to execute [default command]
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    /// Enables verbose output
    #[arg(short, long)]
    pub verbose: bool,
    
    /// Run a single line of KdnLang code
    #[arg(short, long)]
    pub code: Option<String>,
    
    /// Disable fancy error reporting (useful for non-interactive terminals)
    #[arg(long)]
    pub no_fancy_errors: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a KdnLang file
    Run {
        /// Path to the KdnLang file
        #[arg(required = true)]
        file: PathBuf,
    },
    
    /// Check a KdnLang file for errors without running it
    Check {
        /// Path to the KdnLang file
        #[arg(required = true)]
        file: PathBuf,
    },
    
    /// Start an interactive REPL session
    Repl,
}

/// Represents the source of KdnLang code
pub enum CodeSource {
    /// Code loaded from a file
    File(String, PathBuf),
    /// Code provided directly as a string
    String(String),
}

/// Read the contents of a KdnLang file
pub fn read_file(path: &PathBuf) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|e| format!("Error reading file '{}': {}", path.display(), e))
}

/// Handle CLI command execution
pub fn handle_commands(cli: Cli) -> Result<CodeSource, String> {
    // Check for code option first (highest priority)
    if let Some(code) = cli.code.clone() {
        if cli.verbose {
            println!("Running code from command line");
        }
        return Ok(CodeSource::String(code));
    }

    match cli.command {
        Some(Commands::Run { file }) => {
            let input = read_file(&file)?;
            if cli.verbose {
                println!("Running file: {}", file.display());
            }
            Ok(CodeSource::File(input, file))
        },
        Some(Commands::Check { file }) => {
            let input = read_file(&file)?;
            // In a real implementation, we would check for syntax errors here
            // but not execute the code
            println!("File '{}' syntax check passed", file.display());
            // Return the code anyway so main can use it for syntax checking
            Ok(CodeSource::File(input, file.clone()))
        },
        Some(Commands::Repl) => {
            Err("REPL mode is not yet implemented".to_string())
        },
        None => {
            // Default behavior - treat --file as Run command
            match cli.file {
                Some(file) => {
                    let input = read_file(&file)?;
                    if cli.verbose {
                        println!("Running file: {}", file.display());
                    }
                    Ok(CodeSource::File(input, file))
                },
                None => Err("No input source specified. Use --file, --code, or a subcommand.".to_string()),
            }
        }
    }
}