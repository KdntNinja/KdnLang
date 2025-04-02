use crate::error_handling::errors::LexingError;
use crate::lexer::tokens::{Token, TokenWithSpan};
use logos::Logos;
use miette::Result;
use std::ops::Range;

pub fn tokenize<'a>(code: &'a str, filename: &str) -> Result<Vec<TokenWithSpan<'a>>, LexingError> {
    let mut lexer: logos::Lexer<'a, Token> = Token::lexer(code);
    let mut tokens: Vec<TokenWithSpan<'a>> = Vec::new();

    while let Some(result) = lexer.next() {
        match result {
            Ok(token) => {
                let span: Range<usize> = lexer.span();
                let lexeme: &'a str = &code[span.start..span.end];

                let token_with_span: TokenWithSpan<'a> = TokenWithSpan {
                    token,
                    lexeme,
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
