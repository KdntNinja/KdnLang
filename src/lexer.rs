use logos::Logos;

#[derive(Debug, PartialEq, Logos)]
pub enum Token {
    #[token("let")]
    Let,

    #[token("print")]
    Print,

    #[token("for")]
    For,

    #[token("in")]
    In,

    #[token("..")]
    Range,

    #[token("=")]
    Equals,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token(";")]
    Semicolon,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex(r"[0-9]+(\.[0-9]+)?")]
    Number,

    #[regex(r"[ \t\n\r]+", logos::skip)]
    WHITESPACE,
}
