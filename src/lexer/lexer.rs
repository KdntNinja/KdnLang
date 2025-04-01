use crate::error_handling::errors::LexingError;
use logos::{Lexer, Logos};
use miette::Result;
use std::ops::Range;

use super::tokens::{Token, TokenWithSpan};

pub fn tokenize(code: &str) -> Result<Vec<TokenWithSpan<'_>>, LexingError> {
    let mut lexer: Lexer<Token> = Token::lexer(code);
    let mut tokens: Vec<TokenWithSpan> = Vec::new();

    while let Some(result) = lexer.next() {
        match result {
            Ok(token) => {
                let span: Range<usize> = lexer.span();
                let token_with_span: TokenWithSpan = TokenWithSpan {
                    token,
                    lexeme: &code[span.start..span.end],
                    span: span.start..span.end,
                };
                tokens.push(token_with_span);
            }
            Err(_) => {
                let span: Range<usize> = lexer.span();
                return Err(LexingError::new(code.to_string(), span, None, None));
            }
        }
    }

    Ok(tokens)
}
