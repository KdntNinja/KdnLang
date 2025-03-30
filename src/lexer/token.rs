use logos::Logos;

/// Token types for the KdnLang lexical analysis
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    /// Variable or function identifier
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string(), priority = 2)]
    Identifier(String),

    /// Numeric literal
    #[regex(r"[0-9]+", |lex| lex.slice().to_string(), priority = 1)]
    Number(String),

    /// String literal
    #[regex(r#""[^"]*""#, |lex| {
        // Remove the quotation marks and get just the string content
        let content: &str = lex.slice();
        content[1..content.len()-1].to_string()
    })]
    String(String),

    /// Function definition keyword
    #[token("fn")]
    Fn,

    /// Variable declaration keyword
    #[token("let")]
    Let,

    /// Print statement keyword
    #[token("print")]
    Print,

    /// Return statement keyword
    #[token("return")]
    Return,

    /// Conditional statement keyword
    #[token("if")]
    If,

    /// Alternative branch keyword
    #[token("else")]
    Else,

    /// Pattern matching keyword
    #[token("match")]
    Match,

    /// Pattern to expression mapping operator
    #[token("=>")]
    Arrow,

    /// Function return type arrow
    #[token("->")]
    ReturnArrow,

    /// Type annotation separator
    #[token(":")]
    Colon,

    /// Void type keyword
    #[token("none")]
    None,

    /// Type keyword: String
    #[token("str")]
    StrType,

    /// Integer type (i32)
    #[token("i32")]
    I32Type,

    /// Float type (f64)
    #[token("f64")]
    F64Type,

    /// Boolean type
    #[token("bool")]
    BoolType,

    /// Addition operator
    #[token("+")]
    Plus,

    /// Method call dot operator
    #[token(".")]
    Dot,

    /// Open brace for blocks
    #[token("{")]
    OpenBrace,

    /// Close brace for blocks
    #[token("}")]
    CloseBrace,

    /// Open parenthesis for expressions/parameters
    #[token("(")]
    OpenParen,

    /// Close parenthesis for expressions/parameters
    #[token(")")]
    CloseParen,

    /// Separator for parameters/arguments
    #[token(",")]
    Comma,

    /// Statement terminator
    #[token(";")]
    Semicolon,

    /// Single-line comments (skipped during lexing)
    #[regex(r"//[^\n]*", logos::skip)]
    SingleLineComment,

    /// Multi-line comments (skipped during lexing)
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    MultiLineComment,

    /// Whitespace (skipped during lexing)
    #[regex(r"[ \t\n\r]+", logos::skip)]
    Whitespace,
}

impl Token {
    /// Convert a token to its string representation
    ///
    /// # Returns
    /// * `String` - The string representation of the token
    pub fn to_string(&self) -> String {
        match self {
            Token::Identifier(s) => s.clone(),
            Token::Number(s) => s.clone(),
            Token::String(s) => format!("\"{}\"", s),
            Token::Fn => "fn".to_string(),
            Token::Let => "let".to_string(),
            Token::Print => "print".to_string(),
            Token::Return => "return".to_string(),
            Token::If => "if".to_string(),
            Token::Else => "else".to_string(),
            Token::Match => "match".to_string(),
            Token::Arrow => "=>".to_string(),
            Token::ReturnArrow => "->".to_string(),
            Token::Colon => ":".to_string(),
            Token::None => "none".to_string(),
            Token::StrType => "str".to_string(),
            Token::I32Type => "i32".to_string(),
            Token::F64Type => "f64".to_string(),
            Token::BoolType => "bool".to_string(),
            Token::Plus => "+".to_string(),
            Token::Dot => ".".to_string(),
            Token::OpenBrace => "{".to_string(),
            Token::CloseBrace => "}".to_string(),
            Token::OpenParen => "(".to_string(),
            Token::CloseParen => ")".to_string(),
            Token::Comma => ",".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::SingleLineComment => "// comment".to_string(),
            Token::MultiLineComment => "/* comment */".to_string(),
            Token::Whitespace => " ".to_string(),
        }
    }
}
