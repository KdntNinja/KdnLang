use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

// Re-export miette's Result type for use in our code
pub type Result<T> = miette::Result<T>;

#[derive(Error, Debug, Diagnostic)]
#[error("Error in KdnLang")]
pub struct KdnLangError {
    #[source_code]
    pub src: NamedSource<String>,
    #[label("Here is the problem")]
    pub span: SourceSpan,
    #[help]
    pub help: Option<String>,
}
