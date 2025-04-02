use crate::parser::ASTNode;
use std::io::{self, Write};

pub fn print_fn(args: Vec<ASTNode>) -> ASTNode {
    // Example usage documentation:
    // print("Hello, world!");  // Note the semicolon is required

    // Convert arguments to strings and print them
    for arg in args {
        match arg {
            ASTNode::StringLiteral(s) => {
                // Remove quotes from string literals
                let cleaned_string: &str = s.trim_matches(|c: char| c == '\'' || c == '"');
                print!("{}", cleaned_string);
            }
            ASTNode::Number(n) => {
                print!("{}", n);
            }
            ASTNode::Identifier(id) => {
                print!("{}", id);
            }
            _ => print!("{:?}", arg),
        }
    }
    println!(); // Add newline at the end
    io::stdout().flush().unwrap();

    // Return empty/void result
    ASTNode::Void
}
