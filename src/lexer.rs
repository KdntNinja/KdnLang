use crate::errors::LexingError;
use logos::{Lexer, Logos};
use miette::{Result, SourceSpan};

// Define a basic lexer using the Logos crate
#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex(r"[0-9]+")]
    Number,

    #[regex(r"\s+", logos::skip)]
    Whitespace,

    #[regex(r"[+\-*/]")]
    Operator,

    #[regex(r"[(){}\[\]]")]
    Bracket,

    #[regex(r"[=;]")]
    Punctuation,
}

pub fn tokenize(code: &str) -> Result<Vec<(Token, String)>> {
    let mut lexer: Lexer<Token> = Token::lexer(code);
    let mut tokens: Vec<(Token, String)> = Vec::new();

    while let Some(token_result) = lexer.next() {
        match token_result {
            Ok(token) => {
                let span = lexer.span();
                let slice = &code[span.start..span.end];
                tokens.push((token, slice.to_string()));
            }
            Err(_) => {
                let span = lexer.span();
                let error_span = SourceSpan::from(span.start as usize..span.end as usize);
                return Err(LexingError::new(code.to_string(), error_span).into());
            }
        }
    }

    Ok(tokens)
}
