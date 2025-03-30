use std::fs;
use std::env;
use std::path::Path;
use miette::{IntoDiagnostic, Result};
mod lexer;
mod parser;
mod typechecker;
mod errors;
mod compiler;

use crate::lexer::tokenize;
use crate::parser::{KdnParser, ASTNode};
use crate::typechecker::TypeChecker;
use crate::compiler::{Compiler, OptimizationLevel};

fn main() -> Result<()> {
    miette::set_hook(Box::new(|_| Box::new(miette::MietteHandlerOpts::new()
        .context_lines(4)
        .tab_width(2)
        .build())))?;

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage(&args[0]);
        return Ok(());
    }

    // Check if the first argument is a command or a file
    if args[1].ends_with(".kdn") {
        // Treat as file path - implicit compile command
        let input_file: &String = &args[1];
        let output_file: &str = if args.len() > 2 && !args[2].starts_with("-") { 
            &args[2] 
        } else { 
            &derive_output_name(input_file) 
        };
        let opt_level: OptimizationLevel = parse_optimization_level(&args);
        
        compile_file(input_file, output_file, opt_level)?;
    } else {
        // Treat as command
        match args[1].as_str() {
            "compile" => {
                if args.len() < 3 {
                    eprintln!("Error: No input file specified");
                    print_usage(&args[0]);
                    return Ok(());
                }
                
                let input_file: &String = &args[2];
                let output_file: &str = if args.len() > 3 && !args[3].starts_with("-") { 
                    &args[3] 
                } else { 
                    &derive_output_name(input_file) 
                };
                let opt_level: OptimizationLevel = parse_optimization_level(&args);
                
                compile_file(input_file, output_file, opt_level)?;
            },
            "run" => {
                if args.len() < 3 {
                    eprintln!("Error: No input file specified");
                    print_usage(&args[0]);
                    return Ok(());
                }
                
                let input_file: &String = &args[2];
                // For run command, use a temporary file for the output
                let output_file: &str = &format!("/tmp/{}", derive_output_name(input_file));
                let opt_level: OptimizationLevel = parse_optimization_level(&args);
                
                // Compile then immediately run
                compile_file(input_file, output_file, opt_level)?;
                
                // Run the compiled program
                #[cfg(unix)]
                {
                    use std::process::Command;
                    println!("Running {}...", output_file);
                    let status = Command::new(output_file)
                        .status()
                        .into_diagnostic()
                        .map_err(|err| err.context(format!("Failed to execute {}", output_file)))?;
                    
                    if !status.success() {
                        eprintln!("Program exited with code: {}", status.code().unwrap_or(-1));
                    }
                }
            },
            "help" | "--help" | "-h" => {
                print_usage(&args[0]);
            },
            _ => {
                eprintln!("Unknown command: {}", args[1]);
                print_usage(&args[0]);
            }
        }
    }
    
    Ok(())
}

fn print_usage(program_name: &str) -> () {
    println!("KdnLang Compiler");
    println!("Usage:");
    println!("  {} <input_file>.kdn [output_file] [-O0|-O1|-O2|-O3] [-d]", program_name);
    println!("  {} compile <input_file>.kdn [output_file] [-O0|-O1|-O2|-O3] [-d]", program_name);
    println!("  {} run <input_file>.kdn [-O0|-O1|-O2|-O3] [-d]", program_name);
    println!("    -O0    No optimization (default)");
    println!("    -O1    Basic optimizations");
    println!("    -O2    Moderate optimizations");
    println!("    -O3    Aggressive optimizations");
    println!("    -d     Enable debug mode (generate debug files)");
    println!("  {} help", program_name);
}

fn derive_output_name(input_file: &str) -> String {
    let path: &Path = Path::new(input_file);
    let file_stem: &str = path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
    format!("{}", file_stem)
}

fn parse_optimization_level(args: &[String]) -> OptimizationLevel {
    for arg in args {
        match arg.as_str() {
            "-O0" => return OptimizationLevel::None,
            "-O1" => return OptimizationLevel::Basic,
            "-O2" => return OptimizationLevel::Moderate,
            "-O3" => return OptimizationLevel::Aggressive,
            _ => continue,
        }
    }
    OptimizationLevel::None // Default
}

fn compile_file(file_path: &str, output_path: &str, opt_level: OptimizationLevel) -> Result<()> {
    let debug_mode: bool = check_debug_flag();
    
    eprintln!("Compiling {} to {} with optimization level {:?}...", file_path, output_path, opt_level);
    
    let source_code: String = fs::read_to_string(file_path)
        .into_diagnostic()
        .map_err(|err| err.context(format!("Failed to read {}", file_path)))?;
    
    eprintln!("Tokenizing...");
    let tokens: Vec<String> = tokenize(&source_code).map_err(miette::Report::from)?;
    let tokenized_input: String = tokens.join(" ");
    
    eprintln!("Parsing...");
    let ast: ASTNode = KdnParser::parse_program(file_path, &tokenized_input, &source_code)
        .map_err(miette::Report::from)?;
    
    // Only generate debug file if debug mode is enabled
    if debug_mode {
        // Generate and write debug file with AST data to the same directory as the input file
        let debug_file_path = format!("{}.debug", file_path);
        fs::write(&debug_file_path, format!("{:#?}", ast))
            .into_diagnostic()
            .map_err(|err| err.context(format!("Failed to write debug info to {}", debug_file_path)))?;
        eprintln!("Debug information written to '{}'", debug_file_path);
    }
    
    eprintln!("Type checking...");
    TypeChecker::check(file_path, &ast, &source_code)
        .map_err(miette::Report::from)?;
    
    eprintln!("Compiling to binary...");
    Compiler::compile(&ast, output_path, opt_level)
        .map_err(miette::Report::from)?;
    
    eprintln!("Compilation successful! Output written to '{}'", output_path);
    Ok(())
}

// Function to check if debug flag is present in command line arguments
fn check_debug_flag() -> bool {
    let args: Vec<String> = env::args().collect();
    args.iter().any(|arg| arg == "-d")
}