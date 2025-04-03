use logos::Logos;
use std::ops::Range;

#[derive(Debug, PartialEq, Clone, Logos)]
pub enum Token {
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 2)]
    Identifier,

    #[regex(
        r"let|fn|if|else|match|try|except|for|while|return|break|continue",
        priority = 3
    )]
    Keyword,

    #[token("true", priority = 4)]
    #[token("false", priority = 4)]
    BoolLiteral,

    #[regex(r#""([^"\\]|\\.)*""#)]
    #[regex(r#"'([^'\\]|\\.)*'"#)]
    StringLiteral,

    #[regex(r"[0-9]+(\.[0-9]+)?")]
    Number,

    #[regex(r"==|!=|<=|>=|<|>|\+|-|\*|/|=")]
    Operator,

    #[regex(r"[ \t\n\r]+", logos::skip)]
    Whitespace,

    #[regex(r"//.*", logos::skip)]
    Comment,

    #[token(";")]
    Semicolon,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token("->")]
    Arrow,

    #[token("=>")]
    FatArrow,

    #[token(".")]
    Dot,

    // These variants are kept for future use but marked to suppress warnings
    #[allow(dead_code)]
    Illegal,

    #[allow(dead_code)]
    EOF,
}

// Data structure to store a token along with its lexeme and span
#[derive(Debug, Clone)]
pub struct TokenWithSpan<'a> {
    pub token: Token,
    pub lexeme: &'a str,
    pub span: Range<usize>,
}
