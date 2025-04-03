use crate::error_handling::errors::KdnLangError;
use crate::lexer::tokens::{Token, TokenWithSpan};
use logos::Logos;
use miette::Result;
use std::ops::Range;

pub fn tokenize<'a>(
    input: &'a str,
    filename: &str,
) -> Result<Vec<TokenWithSpan<'a>>, miette::Error> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();

    while let Some(result) = lexer.next() {
        match result {
            Ok(token) => {
                let span: Range<usize> = lexer.span();
                let lexeme: &'a str = &input[span.start..span.end];

                let token_with_span = TokenWithSpan {
                    token,
                    lexeme,
                    span: span.start..span.end,
                };
                tokens.push(token_with_span);
            }
            Err(_) => {
                let span: Range<usize> = lexer.span();
                let unexpected_token: &str = &input[span.clone()];
                let message: String = format!("Unexpected token: '{}'", unexpected_token);
                return Err(KdnLangError::from_unexpected_token(
                    input.to_string(),
                    filename,
                    span,
                    Some(message),
                )
                .into());
            }
        }
    }

    Ok(tokens)
}
