use crate::parser::ASTNode;
use std::io::{self, Write};

pub fn input_fn(args: Vec<ASTNode>) -> ASTNode {
    // Print prompt if provided
    if !args.is_empty() {
        if let ASTNode::StringLiteral(s) = &args[0] {
            // Remove quotes from string literals
            let prompt: &str = s.trim_matches(|c: char| c == '\'' || c == '"');
            print!("{}", prompt);
        } else {
            print!("{:?}", args[0]);
        }
        io::stdout().flush().unwrap();
    }

    // Read input from stdin
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Remove trailing newline and return as string literal
    let trimmed_input: String = input.trim_end().to_string();
    let formatted_result: String = format!("\"{}\"", trimmed_input);
    ASTNode::StringLiteral(formatted_result)
}
