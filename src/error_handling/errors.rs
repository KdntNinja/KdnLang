use miette::{Diagnostic, NamedSource, SourceSpan};
use std::ops::Range;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("Lexing error: {message}")]
#[diagnostic(
    code(kdnlang::lexer::error),
    help("{help_text}"),
    url("https://docs.kdnlang.org/errors#lexer-error")
)]
pub struct LexingError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("Error occurred here")]
    pub span: SourceSpan,

    pub message: String,
    pub help_text: String,
}

impl LexingError {
    pub fn new(
        src: String,
        filename: &str,
        span: Range<usize>,
        message_opt: Option<String>,
    ) -> Self {
        let invalid_token: &str = match src.get(span.clone()) {
            Some(token) => token,
            None => "[invalid position]",
        };

        let message: String =
            message_opt.unwrap_or_else(|| format!("Invalid token: '{}'", invalid_token));

        // Try to provide context by getting the line
        let line_start: usize = src[..span.start].rfind('\n').map_or(0, |pos| pos + 1);
        let line_end: usize = src[span.end..]
            .find('\n')
            .map_or(src.len(), |pos| span.end + pos);
        let line: &str = match src.get(line_start..line_end) {
            Some(l) => l,
            None => "[could not extract line]",
        };

        // Calculate line and column number for better error messages
        let line_number: usize = src[..span.start].matches('\n').count() + 1; // 1-based line number
        let column: usize = span.start - line_start + 1; // 1-based column number

        let help_text: String = format!(
            "Check your syntax near '{}' at line {}, column {}. KdnLang does not recognize this token in this context.",
            line.trim(), line_number, column
        );

        Self {
            src: NamedSource::new(filename, src),
            span: (span.start, span.end - span.start).into(),
            message,
            help_text,
        }
    }
}

#[derive(Debug, Diagnostic, Error)]
#[error("Parse error: {message}")]
#[diagnostic(code(kdnlang::parser::error), help("{help_text}"))]
pub struct ParseErrorDetailed {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("Error occurred here")]
    pub span: SourceSpan,

    pub message: String,
    pub help_text: String,
}

#[derive(Debug, Diagnostic, Error)]
#[error("Parse error: {message}")]
#[diagnostic(code(kdnlang::parser::error), help("{help_text}"))]
pub struct ParseErrorWithDetails {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("Error occurred here")]
    pub span: SourceSpan,

    pub message: String,
    pub help_text: String,
}

#[allow(dead_code)]
impl ParseErrorWithDetails {
    pub fn missing_semicolon(
        source: String,
        filename: &str,
        span: SourceSpan,
        context: &str,
    ) -> Self {
        // Calculate line and column from span
        let offset: usize = span.offset();
        let line_number: usize = source[..offset].matches('\n').count() + 1;
        let line_start: usize = source[..offset].rfind('\n').map_or(0, |pos: usize| pos + 1);
        let column: usize = offset - line_start + 1;

        let error_message: String = format!(
            "Missing semicolon after '{}' at line {}, column {}",
            context, line_number, column
        );

        let help_text: String = "KdnLang requires semicolons at the end of statements, similar to Rust and C++.\nExample: print(\"Hello\"); - Note the semicolon".to_string();

        Self {
            src: NamedSource::new(filename, source),
            span,
            message: error_message,
            help_text,
        }
    }
}

#[derive(Debug, Diagnostic, Error)]
#[error("Runtime error: {message}")]
#[diagnostic(
    code(kdnlang::runtime::error),
    help("{help_text}"),
    url("https://docs.kdnlang.org/errors#runtime-error")
)]
pub struct RuntimeError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("Error occurred here")]
    pub span: SourceSpan,

    pub message: String,
    pub help_text: String,
}

impl RuntimeError {
    pub fn new(
        source_code: String,
        filename: &str,
        span: impl Into<SourceSpan>,
        message: impl Into<String>,
        help: Option<String>,
    ) -> Self {
        let message_string: String = message.into();
        let span_value: SourceSpan = span.into();

        // Calculate line and column number
        let offset: usize = span_value.offset();
        let line_number: usize = source_code[..offset].matches('\n').count() + 1;
        let line_start: usize = source_code[..offset]
            .rfind('\n')
            .map_or(0, |pos: usize| pos + 1);
        let column: usize = offset - line_start + 1;

        let help_text: String = help.unwrap_or_else(|| {
            format!(
                "Check your code logic around line {}, column {}.",
                line_number, column
            )
        });

        Self {
            src: NamedSource::new(filename, source_code),
            span: span_value,
            message: message_string,
            help_text,
        }
    }
}
