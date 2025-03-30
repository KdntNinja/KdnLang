use super::token::Token;
use crate::errors::{span, KdnError, KdnResult};
use logos::Logos;
use miette::NamedSource;

/// Token information with source position metadata
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TokenInfo {
    /// The actual token
    pub token: Token,
    /// Source position (start_position, length)
    pub span: (usize, usize),
}

/// Convert source code string into tokens
///
/// # Arguments
/// * `input` - The source code to tokenize
///
/// # Returns
/// * `KdnResult<Vec<String>>` - A result containing either a vector of string tokens or an error
pub fn tokenize(input: &str) -> KdnResult<Vec<String>> {
    let mut lexer: logos::Lexer<'_, Token> = Token::lexer(input);
    let mut tokens: Vec<String> = Vec::new();
    let mut position: usize = 0;

    while let Some(token_result) = lexer.next() {
        match token_result {
            Ok(token) => {
                // Track position info for error reporting
                let token_span: std::ops::Range<usize> = lexer.span();
                let _token_length: usize = token_span.end - token_span.start;
                position = token_span.start;

                tokens.push(token.to_string());
            }
            Err(_) => {
                // Create a rich error with source code context
                return Err(KdnError::LexerError {
                    src: NamedSource::new("input", input.to_string()),
                    message: format!("Invalid token at position {}", position),
                    span: span(position, 1),
                });
            }
        }
    }

    Ok(tokens)
}
