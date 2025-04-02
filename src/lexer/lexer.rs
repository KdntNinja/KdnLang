use crate::error_handling::errors::LexingError;
use logos::{Lexer, Logos};
use miette::Result;
use std::ops::Range;

use super::tokens::{Token, TokenWithSpan};

pub fn tokenize<'a>(code: &'a str, filename: &str) -> Result<Vec<TokenWithSpan<'a>>, LexingError> {
    let mut lexer: Lexer<'a, Token> = Token::lexer(code);
    let mut tokens: Vec<TokenWithSpan<'a>> = Vec::new();

    while let Some(result) = lexer.next() {
        match result {
            Ok(token) => {
                let span: Range<usize> = lexer.span();
                let token_with_span: TokenWithSpan<'a> = TokenWithSpan {
                    token,
                    lexeme: &code[span.start..span.end],
                    span: span.start..span.end,
                };
                tokens.push(token_with_span);
            }
            Err(_) => {
                let span: Range<usize> = lexer.span();
                let unexpected_token: &str = &code[span.clone()];
                let message: String = format!("Unexpected token: '{}'", unexpected_token);
                return Err(LexingError::new(
                    code.to_string(),
                    filename,
                    span,
                    Some(message),
                ));
            }
        }
    }

    Ok(tokens)
}
