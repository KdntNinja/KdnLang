use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("Lexing error occurred")]
#[diagnostic(
    code(kdnlang::lexer::error),
    help("Check the syntax near the error location.")
)]
pub struct LexingError {
    #[label("Invalid token here")]
    span: SourceSpan,

    #[source_code]
    source_code: String,
}

impl LexingError {
    pub fn new(source_code: String, span: SourceSpan) -> Self {
        Self { source_code, span }
    }
}
