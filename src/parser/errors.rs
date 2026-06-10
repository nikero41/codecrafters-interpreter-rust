use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum ParseError {
    #[error("Placeholder")]
    Placeholder,
    #[error("[line {line}] Error at end: {message}")]
    Eof { line: u32, message: String },
    #[error("[line {line}] Error at '{lexeme}': {message}")]
    PlaceholderForName {
        line: u32,
        lexeme: String,
        message: String,
    },
    #[error("[line {line}] Error: Unterminated paren.")]
    UnterminatedParen { line: u32 },
}
