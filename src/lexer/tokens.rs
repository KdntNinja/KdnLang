use logos::Logos;
use std::ops::Range;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum Token {
    // Keywords
    #[regex(r"let|fn|struct|match|try|except|async|await", priority = 3)]
    Keyword,

    // Matches type keywords like `&str`, `i32`, `f64`, etc.
    #[regex(r"&str|i32|f64", priority = 4)]
    TypeKeyword,

    // Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 1)]
    Identifier,

    // Literals
    #[regex(r"[0-9]+")]
    Number,
    #[regex(r#"[\"'][^\"']*[\"']"#, priority = 2)]
    StringLiteral,

    // Operators
    #[regex(r"[+\-*/%]")]
    Operator,

    // Match arrow operator
    #[token("=>")]
    MatchArrow,

    // Punctuation with clear distinction for semicolons
    #[token(";")]
    Semicolon,

    #[regex(r"[=,.]")]
    Punctuation,

    // Brackets
    #[regex(r"[(){}\[\]]")]
    Bracket,

    // Colon
    #[regex(r":", priority = 5)]
    Colon,

    // Skips whitespace characters
    #[regex(r"\s+", logos::skip)]
    Whitespace,
}

#[derive(Debug)]
pub struct TokenWithSpan<'a> {
    pub token: Token,
    pub lexeme: &'a str,
    pub span: Range<usize>,
}
