use crate::parser::ASTNode;
use std::io::{self, Stdout, StdoutLock, Write};

pub fn print_fn(args: Vec<ASTNode>) -> ASTNode {
    let stdout: Stdout = io::stdout();
    let mut handle: StdoutLock = stdout.lock();

    if args.is_empty() {
        return ASTNode::Void;
    }

    let output: String = args
        .iter()
        .map(|arg| match arg {
            ASTNode::StringLiteral(s) => {
                s.trim_matches(|c: char| c == '\'' || c == '"').to_string()
            }
            ASTNode::Number(n) => n.to_string(),
            ASTNode::Identifier(id) => id.clone(),
            _ => format!("{:?}", arg),
        })
        .collect::<Vec<_>>()
        .join(" ");

    write!(handle, "{}", output).unwrap();
    handle.flush().unwrap();

    ASTNode::Void
}
