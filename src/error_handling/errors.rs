use miette::{Diagnostic, NamedSource, SourceSpan};
use std::ops::Range;
use thiserror::Error;

/// Common error type for all KdnLang errors
#[derive(Debug, Diagnostic, Error)]
#[error("{kind} error: {message}")]
#[diagnostic(code(kdnlang_error), help("{help_text}"))]
pub struct KdnLangError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("Error occurred here")]
    pub span: SourceSpan,

    pub kind: String,
    pub message: String,
    pub help_text: String,
}

impl KdnLangError {
    /// Create a new error with the given kind, message, and help text
    pub fn new(
        source: String,
        filename: &str,
        span: impl Into<SourceSpan>,
        kind: &str,
        message: impl Into<String>,
        help: impl Into<String>,
    ) -> Self {
        Self {
            src: NamedSource::new(filename, source),
            span: span.into(),
            kind: kind.to_string(),
            message: message.into(),
            help_text: help.into(),
        }
    }

    /// Create a lexer error
    pub fn lexer_error(
        source: String,
        filename: &str,
        span: Range<usize>,
        message: impl Into<String>,
        help: impl Into<String>,
    ) -> Self {
        Self::new(
            source,
            filename,
            (span.start, span.end - span.start),
            "lexer",
            message,
            help,
        )
    }

    /// Create a parser error
    pub fn parser_error(
        source: String,
        filename: &str,
        span: impl Into<SourceSpan>,
        message: impl Into<String>,
        help: impl Into<String>,
    ) -> Self {
        Self::new(source, filename, span, "parser", message, help)
    }

    /// Create a runtime error
    pub fn runtime_error(
        source: String,
        filename: &str,
        span: impl Into<SourceSpan>,
        message: impl Into<String>,
        help: impl Into<String>,
    ) -> Self {
        Self::new(source, filename, span, "runtime", message, help)
    }

    /// Helper for creating lexing errors from an unexpected token
    pub fn from_unexpected_token(
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

        Self::lexer_error(src, filename, span, message, help_text)
    }

    /// Helper for creating missing semicolon errors
    pub fn missing_semicolon_error(
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

        let help_text: String = "KdnLang requires semicolons at the end of statements, like Rust syntax.\nExample: let x: int = 5; - Note the semicolon".to_string();

        Self::parser_error(source, filename, span, error_message, help_text)
    }
}
