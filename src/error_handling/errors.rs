use miette::{Diagnostic, SourceSpan};
use std::ops::Range;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("Lexing error: {message}")]
#[diagnostic(code(kdnlang::lexer::error), help("Check the syntax of your input."))]
pub struct LexingError {
    #[source_code]
    src: String,

    #[label("Error occurred here")]
    span: SourceSpan,

    message: String,
}

impl LexingError {
    pub fn new(
        src: String,
        span: Range<usize>,
        message_opt: Option<String>,
        help_opt: Option<String>,
    ) -> Self {
        let message = message_opt.unwrap_or_else(|| "Invalid token".to_string());

        Self {
            src,
            span: (span.start, span.end - span.start).into(),
            message,
        }
    }
}
