use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

/// KdnLang's error type hierarchy for the compiler and runtime
#[derive(Error, Debug, Diagnostic)]
#[allow(dead_code)]
pub enum KdnError {
    /// Error that occurs during lexical analysis (tokenization phase)
    #[error("Lexer error: {message}")]
    #[diagnostic(
        code(kdnlang::lexer),
        url("https://docs.rs/kdnlang/latest/kdnlang/lexer/"),
        help("Check your syntax and ensure all tokens are valid")
    )]
    LexerError {
        #[source_code]
        src: NamedSource<String>,
        message: String,
        #[label("This token caused the error")]
        span: SourceSpan,
    },

    /// Error that occurs during parsing (syntax analysis phase)
    #[error("Parser error: {message}")]
    #[diagnostic(
        code(kdnlang::parser),
        url("https://docs.rs/kdnlang/latest/kdnlang/parser/"),
        help("Ensure your code follows KdnLang's syntax rules")
    )]
    ParserError {
        #[source_code]
        src: NamedSource<String>,
        message: String,
        #[label("The error occurred here")]
        span: SourceSpan,
    },

    /// Error that occurs during type checking (semantic analysis phase)
    #[error("Type error: {message}")]
    #[diagnostic(
        code(kdnlang::type_checker),
        url("https://docs.rs/kdnlang/latest/kdnlang/type_checker/"),
        help("Check that your variable types match the operations you're performing")
    )]
    TypeError {
        #[source_code]
        src: NamedSource<String>,
        message: String,
        #[label("Invalid type used here")]
        span: SourceSpan,
    },

    /// Runtime error during program execution
    #[error("Runtime error: {message}")]
    #[diagnostic(
        code(kdnlang::runtime),
        url("https://docs.rs/kdnlang/latest/kdnlang/interpreter/"),
        help("This error occurred while your program was running")
    )]
    RuntimeError {
        #[source_code]
        src: NamedSource<String>,
        message: String,
        #[label("The error occurred during execution of this code")]
        span: SourceSpan,
    },

    /// Generic compiler error with customizable help and label text
    #[error("Compilation error: {message}")]
    #[diagnostic(
        code(kdnlang::compiler),
        url("https://docs.rs/kdnlang/latest/kdnlang/compiler/"),
        help("{help}")
    )]
    CompilationError {
        #[source_code]
        src: NamedSource<String>,
        message: String,
        help: String,
        #[label("{label_text}")]
        span: SourceSpan,
        label_text: String,
    },

    /// An error without source position information
    #[error("{0}")]
    SimpleError(String),
}

/// Convert a string offset to a line and column position
/// 
/// # Arguments
/// * `source` - The source code string
/// * `offset` - The character offset in the source
/// 
/// # Returns
/// * A tuple of (line, column) where both are 1-indexed
#[allow(dead_code)]
pub fn offset_to_line_column(source: &str, offset: usize) -> (usize, usize) {
    let mut line: usize = 1;
    let mut col: usize = 0;
    
    for (i, c) in source.chars().enumerate() {
        if i >= offset {
            break;
        }
        
        if c == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    
    (line, col)
}

/// Generate a source span from a start position and length
/// 
/// # Arguments
/// * `start` - The starting character offset
/// * `len` - The length of the span in characters
/// 
/// # Returns
/// * A SourceSpan suitable for use with miette diagnostics
pub fn span(start: usize, len: usize) -> SourceSpan {
    let span_data: (usize, usize) = (start, len);
    span_data.into()
}

/// A result type alias with KdnError as the error type
pub type KdnResult<T> = Result<T, KdnError>;