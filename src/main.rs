use miette::{IntoDiagnostic, Result};
use std::env;
mod cli;
mod compiler;
mod errors;
mod lexer;
mod parser;
mod typechecker;

fn main() -> Result<()> {
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .context_lines(4)
                .tab_width(2)
                .build(),
        )
    }))?;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        cli::print_usage(&args[0]);
        return Ok(());
    }

    cli::process_args(args)
}
